use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{self, Formatter},
    pin::Pin,
    time::Duration,
};

use time::OffsetDateTime;
use tokio::{
    select, spawn,
    sync::mpsc::{UnboundedSender, unbounded_channel},
    time::sleep,
};

struct Task {
    pub at: OffsetDateTime,
    pub future: Option<Pin<Box<dyn Future<Output = ()> + Send>>>,
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
    pub max_poll: Duration,
    /// If Some(d), skip (do not run) tasks that are more than d late.
    pub catch_up_limit: Option<Duration>,
    /// If true, tasks scheduled in the past run immediately instead of waiting up to max_poll.
    pub run_past_due_immediately: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_poll: Duration::from_secs(60),
            catch_up_limit: None,
            run_past_due_immediately: true,
        }
    }
}

#[derive(Debug)]
pub struct Scheduler {
    config: Config,

    tx: UnboundedSender<Request>,
}

impl Scheduler {
    /// Creates a new Scheduler and starts it
    pub fn new(config: Config) -> Self {
        let (tx, mut rx) = unbounded_channel::<Request>();

        spawn(async move {
            let mut heap: BinaryHeap<Task> = BinaryHeap::new();

            loop {
                let now = now_local_or_utc();
                while let Some(mut task) = heap.pop_if(|task| task.at <= now) {
                    let time_since_task = now - task.at;
                    // Should never fail since the while loop only runs if the task is in the past.
                    // If it is negative (indicating that the task is in the future), we can skip the task.
                    let late_by = time_since_task.try_into().unwrap_or(Duration::MAX);
                    // Don't run tasks that are too old
                    if let Some(limit) = config.catch_up_limit
                        && late_by > limit
                    {
                        eprintln!(
                            "Skipping stale task scheduled at {} (late by {}s)",
                            task.at,
                            late_by.as_secs()
                        );
                        continue;
                    }
                    if let Some(future) = task.future.take() {
                        spawn(async move {
                            future.await;
                        });
                    }
                }

                // Determine sleep duration
                let sleep_time = heap
                    .peek()
                    .map(|next| {
                        let time_until_next = next.at - now_local_or_utc();
                        // Cap the lower bound to zero in case the task is scheduled for the past
                        time_until_next.try_into().unwrap_or(Duration::ZERO)
                    })
                    .unwrap_or(config.max_poll);

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

        Scheduler { config, tx }
    }

    pub fn schedule<F>(&mut self, at: OffsetDateTime, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let now = now_local_or_utc();
        if self.config.run_past_due_immediately && at <= now {
            spawn(future);
            return;
        }
        let task = Task {
            at,
            future: Some(Box::pin(future)),
        };
        let _ = self.tx.send(Request::Add(task));
    }

    pub fn stop(&self) {
        let _ = self.tx.send(Request::Stop);
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

trait BinaryHeapPopIf<T: std::cmp::Ord> {
    fn pop_if(&mut self, predicate: impl FnOnce(&T) -> bool) -> Option<T>;
}

impl<T: std::cmp::Ord> BinaryHeapPopIf<T> for BinaryHeap<T> {
    fn pop_if(&mut self, predicate: impl FnOnce(&T) -> bool) -> Option<T> {
        let greatest = self.peek()?;
        if predicate(greatest) {
            self.pop()
        } else {
            None
        }
    }
}
