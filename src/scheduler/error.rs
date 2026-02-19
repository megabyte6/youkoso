use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScheduleError {
    /// The task runner is not executing. This also implies that the task has been discarded.
    #[error("the task runner does not seem to be able to start")]
    TaskRunnerFailedToStart,
}
