mod error;

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{self, Formatter},
    pin::Pin,
    time::Duration,
};

pub use error::ScheduleError;
use time::OffsetDateTime;
use tokio::{
    runtime::Runtime,
    select, spawn,
    sync::mpsc::{UnboundedSender, unbounded_channel},
    time::sleep,
};

struct Task {
    pub at: OffsetDateTime,
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task").field("at", &self.at).finish()
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.at.cmp(&other.at)
    }
}

enum Request {
    Add(Task),
    Stop,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Upper bound on how long the loop will sleep; bounds worst-case delay after resume.
    pub max_poll_interval: Duration,
    /// If `Some(d)`, skip (do not run) tasks that are more than `d` late.
    /// If `None`, always run missed tasks.
    pub catch_up_limit: Option<Duration>,
    /// If true, tasks scheduled in the past run immediately instead of waiting up to max_poll.
    pub prioritize_overdue: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_poll_interval: Duration::from_secs(60),
            catch_up_limit: None,
            prioritize_overdue: true,
        }
    }
}

#[derive(Debug)]
pub struct Scheduler {
    config: Config,
    runtime: Runtime,

    tx: Option<UnboundedSender<Request>>,
}

impl Scheduler {
    /// Creates a new Scheduler
    pub fn new(runtime: Runtime, config: Config) -> Self {
        Self {
            config,
            runtime,
            tx: None,
        }
    }

    pub fn schedule<F>(&mut self, at: OffsetDateTime, future: F) -> Result<(), ScheduleError>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        if self.config.prioritize_overdue && at <= now_local_or_utc() {
            self.runtime.spawn(future);
            return Ok(());
        }

        if self.tx.is_none() {
            self.launch_task_thread();
        }
        if let Some(tx) = &self.tx {
            let task = Task {
                at,
                future: Box::pin(future),
            };
            let _ = tx.send(Request::Add(task));
            Ok(())
        } else {
            Err(ScheduleError::TaskRunnerFailedToStart)
        }
    }

    fn launch_task_thread(&mut self) {
        let (tx, mut rx) = unbounded_channel::<Request>();
        self.tx = Some(tx);

        let config = self.config;
        self.runtime.spawn(async move {
            let mut heap: BinaryHeap<Task> = BinaryHeap::new();

            loop {
                let now = now_local_or_utc();
                while let Some(task) = heap.pop_if(|task| task.at <= now) {
                    // Should never fail since the loop only runs if the task is in the past.
                    // If it is negative, indicating that the task is in the future, we can skip the task.
                    let late_by = (now - task.at).try_into().unwrap_or(Duration::MAX);
                    if let Some(limit) = config.catch_up_limit
                        && late_by > limit
                    {
                        eprintln!(
                            "skipping stale task scheduled for {} (late by {}s)",
                            task.at,
                            late_by.as_secs()
                        );
                        continue;
                    }
                    spawn(async move {
                        task.future.await;
                    });
                }

                let sleep_time = heap
                    .peek()
                    .map(|next| {
                        let time_until_next = next.at - now_local_or_utc();
                        // cap the lower bound to zero in case the task is scheduled for the past
                        let duration = time_until_next.try_into().unwrap_or(Duration::ZERO)
                        duration.min(config.max_poll_interval)
                    })
                    .unwrap_or(config.max_poll_interval);

                select! {
                    cmd = rx.recv() => {
                        match cmd {
                            Some(Request::Add(task)) => {
                                heap.push(task);
                            }
                            Some(Request::Stop) | None => {
                                break;
                            }
                        }
                    }
                    _ = sleep(sleep_time) => {}
                }
            }
        });
    }

    pub fn stop(&mut self) {
        if let Some(tx) = &self.tx {
            // safe to ignore the return type since if `send()` fails, the worker thread is likely dead
            let _ = tx.send(Request::Stop);
            self.tx = None;
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.stop();
    }
}

fn now_local_or_utc() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

trait PopIf<T: Ord> {
    fn pop_if(&mut self, predicate: impl FnOnce(&T) -> bool) -> Option<T>;
}

impl<T: Ord> PopIf<T> for BinaryHeap<T> {
    fn pop_if(&mut self, predicate: impl FnOnce(&T) -> bool) -> Option<T> {
        let greatest = self.peek()?;
        if predicate(greatest) {
            self.pop()
        } else {
            None
        }
    }
}
