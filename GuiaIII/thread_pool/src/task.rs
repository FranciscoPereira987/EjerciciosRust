use crate::threadpool::TaskNumber;

use crate::runable::Runable;

use std::sync::{Arc, Mutex};

pub struct Task<T> {
    task_number: T,

    to_run: Arc<Mutex<Box<dyn Runable + Send>>>,
}

impl<T: Clone> Task<T> {
    pub fn new(task_number: T, to_run: Arc<Mutex<Box<dyn Runable + Send>>>) -> Task<T> {
        Task {
            task_number,
            to_run,
        }
    }

    pub fn get_task_number(&self) -> T {
        self.task_number.clone()
    }

    pub fn get_task(&self) -> Arc<Mutex<Box<dyn Runable + Send>>> {
        self.to_run.clone()
    }
}

impl Task<usize> {
    pub fn to_task_number(&self) -> Task<TaskNumber> {
        Task {
            task_number: TaskNumber::NotRunning(self.get_task_number()),
            to_run: self.get_task(),
        }
    }
}
