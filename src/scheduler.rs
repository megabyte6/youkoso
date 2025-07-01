use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::{self, Formatter},
    sync::{Arc, Mutex},
    time::Duration,
};

use time::OffsetDateTime;
use tokio::{
    spawn,
    task::{self, JoinHandle},
    time::sleep,
};
use uuid::Uuid;

struct Task {
    pub id: Uuid,
    pub execution_time: OffsetDateTime,
    pub callback: Box<dyn FnOnce() + Send>,
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("execution_time", &self.execution_time)
            .finish()
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.execution_time == other.execution_time
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
        self.execution_time.cmp(&other.execution_time)
    }
}

#[derive(Debug)]
pub struct TaskScheduler {
    tasks: Arc<Mutex<BinaryHeap<Reverse<Task>>>>,
    max_poll_delay: Duration,
    task_runner_handle: Option<JoinHandle<()>>,
}

impl TaskScheduler {
    pub fn new(max_poll_delay: Duration) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(BinaryHeap::new())),
            max_poll_delay,
            task_runner_handle: None,
        }
    }

    pub fn schedule_at<F>(&mut self, time: OffsetDateTime, callback: F) -> Uuid
    where
        F: FnOnce() + Send + 'static,
    {
        let uuid = Uuid::new_v4();

        let should_spawn_processor = {
            let mut tasks = self.tasks.lock().unwrap_or_else(|err| err.into_inner());
            tasks.push(Reverse(Task {
                id: uuid,
                execution_time: time,
                callback: Box::new(callback),
            }));

            self.task_runner_handle
                .as_ref()
                .is_none_or(|handle| handle.is_finished())
        };
        if should_spawn_processor {
            self.launch_task_runner();
        }

        uuid
    }

    fn launch_task_runner(&mut self) {
        let tasks = Arc::clone(&self.tasks);
        let max_poll_delay = self.max_poll_delay.as_secs_f64();

        self.task_runner_handle = Some(task::spawn(async move {
            loop {
                let now = OffsetDateTime::now_utc();

                let next_task_time = tasks
                    .lock()
                    .unwrap_or_else(|err| err.into_inner())
                    .peek()
                    .map(|task| task.0.execution_time);
                match next_task_time {
                    Some(task_time) if task_time <= now => {
                        if let Some(Reverse(task)) =
                            tasks.lock().unwrap_or_else(|err| err.into_inner()).pop()
                        {
                            spawn(async move { (task.callback)() });
                        }
                    }
                    Some(task_time) => {
                        let sleep_duration = (task_time - now).as_seconds_f64().max(max_poll_delay);
                        sleep(Duration::from_secs_f64(sleep_duration)).await;
                    }
                    None => {
                        break;
                    }
                }
            }
        }));
    }
}

impl Drop for TaskScheduler {
    fn drop(&mut self) {
        if let Some(handle) = &self.task_runner_handle {
            handle.abort();
        }
    }
}
