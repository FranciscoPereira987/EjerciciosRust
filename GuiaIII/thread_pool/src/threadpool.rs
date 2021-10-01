/*
    Tengo que hacer una estructura de datos, que contenga un par de
    threads,especificados por el usuario. Les puedo pedir un trabajo
    y lo realizan.
*/

use crate::queue::Queue;
use crate::runable::Runable;
use crate::task::Task;
use std::sync::{mpsc, Arc, Mutex};

use std::collections::HashMap;

use std::thread::{spawn, JoinHandle};

#[derive(PartialOrd, Clone)]
pub enum TaskNumber {
    NotRunning(usize),

    Running(usize), // (task_number)

    Finished(usize, usize), //(task_number, thread_number) -> thread number me sirve para saber que thread termino la tarea
}

pub struct ThreadPool {
    ready_queue: Queue<(JoinHandle<Result<(), String>>, usize)>, //Aca se guardan los que estan listos para aceptar un trabajo

    working: HashMap<usize, JoinHandle<Result<(), String>>>, //Aca se guardan los que estan trabajando

    recievers: Vec<mpsc::Receiver<TaskNumber>>, //Aca guardo los recievers de los threads, por numero

    senders: Vec<mpsc::Sender<Task<TaskNumber>>>, //Aca guardo los senders, para pasar las tareas

    tasks: Vec<TaskNumber>, //Aca guardo el estado de las tareas que se le pasan al thread pool

    task_list: Queue<Task<usize>>,

    task_manager: usize,

    pool_size: usize,
}

struct RunableDummy;

impl Runable for RunableDummy {
    fn run_mut(&mut self) {}

    fn run(&self) {}
}

impl TaskNumber {
    pub fn is_valid(&self) -> bool {
        !matches!(*self, TaskNumber::NotRunning(0))
    }

    pub fn get_task_number(&self) -> usize {
        match *self {
            TaskNumber::NotRunning(number) => number,

            TaskNumber::Running(number) => number,

            TaskNumber::Finished(number, _) => number,
        }
    }

    pub fn get_id_number(&self) -> usize {
        match *self {
            TaskNumber::Finished(_, number) => number,

            _ => 1000,
        }
    }
}

impl PartialEq for TaskNumber {
    fn eq(&self, other: &TaskNumber) -> bool {
        self.get_task_number() == other.get_task_number()
    }
}

impl ThreadPool {
    pub fn new(number_of_threads: usize) -> ThreadPool {
        let mut ready_queue = Queue::new();
        let mut recievers = Vec::new();
        let mut senders = Vec::new();

        thread_initialization(
            number_of_threads,
            &mut ready_queue,
            &mut recievers,
            &mut senders,
        );

        ThreadPool {
            ready_queue,
            working: HashMap::new(),
            recievers,
            senders,
            tasks: Vec::new(),
            task_list: Queue::new(),
            task_manager: 1,
            pool_size: number_of_threads,
        }
    }

    /*
        Checks if there is any free thread
    */
    fn any_free(&self) -> bool {
        !self.ready_queue.is_empty()
    }
    /*
        Sends a task to a thread
    */
    fn send_task(&mut self, task: Task<usize>) {
        let task_number = task.get_task_number();
        if let Ok((handle, handle_number)) = self.ready_queue.pop() {
            self.working.insert(handle_number, handle);
            if let Some(tx) = self.senders.get_mut(handle_number) {
                if let Err(_error) = tx.send(task.to_task_number()) {
                    println!("Error in sending task: {}", task_number);
                }
            }
        }
    }

    /*
        updates the queue and allocate new tasks
    */
    fn uptdate_tasks(&mut self) -> bool {
        if self.any_free() && !self.task_list.is_empty() {
            //Allocate new task
            if let Ok(task) = self.task_list.pop() {
                self.update_task_status(TaskNumber::Running(task.get_task_number()));
                self.send_task(task);
            }
            return true;
        }

        false
    }

    /*
        Generates a new task
    */
    fn generate_new_task(&mut self, new_task: Arc<Mutex<Box<dyn Runable + Send>>>) {
        let task_number = self.task_manager;
        self.task_manager += 1;
        self.task_list.put(Task::new(task_number, new_task));
        self.tasks.push(TaskNumber::NotRunning(task_number));
    }
    /*
    Adds the task to the task lisk, if any thread is available then it sends the task
    if not, stores the task.
    */
    pub fn spawn(&mut self, new_task: Arc<Mutex<Box<dyn Runable + Send>>>) {
        self.generate_new_task(new_task);
        self.check();
    }

    fn update_task_status(&mut self, task_number: TaskNumber) {
        let index = self.tasks.iter().position(|task| *task == task_number);
        if let Some(valid_index) = index {
            self.tasks.remove(valid_index);
        }

        self.tasks.push(task_number);
    }
    /*
        Updates the waiting queue
    */
    fn update_waiting_queue(&mut self, finished_task: &TaskNumber) {
        let id = finished_task.get_id_number();

        if id < 1000 {
            if let Some((_, handle)) = self.working.remove_entry(&id) {
                self.ready_queue.put((handle, id));
            }
        }
    }
    /*
    Checks if any thread has finished its tasks, updates the queue and allocates new tasks
    */
    pub fn check(&mut self) {
        let mut status_updates = Vec::new();

        for reciever in self.recievers.iter_mut() {
            if let Ok(task_number) = reciever.try_recv() {
                status_updates.push(task_number);
            }
        }

        for task_number in status_updates {
            self.update_waiting_queue(&task_number);
            self.update_task_status(task_number);
        }

        while self.uptdate_tasks() {
            //Updates all tasks posible
        }
    }

    /*
    Kills all running threads, waiting for those that have unfinished tasks and finishes
    every job on the queue
    */
    pub fn join(&mut self) {
        for _i in 0..self.pool_size {
            self.task_list
                .put(Task::new(0, Arc::new(Mutex::new(Box::new(RunableDummy)))));
            //Adding the killing tasks
        }
        while !self.task_list.is_empty() {
            self.check();
        }
        while !self.ready_queue.is_empty() {
            self.check();
            if let Ok((task_handler, _id)) = self.ready_queue.pop() {
                if task_handler.join().is_err() {
                    println!("A thread exited with an error");
                }
            }
        }
    }
}

/*
    Los threads tienen que correr un closure que haga:
                                                        1- espere que se le entregue una tarea
                                                            -Si no es valida mato al thread
                                                        2- cuando se le entrega acusar el recibo de la misma
                                                        3- hacerla
                                                        4- Terminarla y acusar que la terminaron
                                                        5- volver a 1
*/

fn validate_task(
    waiting_channel: &mut mpsc::Receiver<Task<TaskNumber>>,
    task_number: &mut TaskNumber,
    to_run: &mut Arc<Mutex<Box<dyn Runable + Send>>>,
) -> bool {
    if let Ok(task) = waiting_channel.recv() {
        *task_number = task.get_task_number();
        *to_run = task.get_task();
        if task_number.is_valid() {
            return true;
        }
    }

    false
}

fn run_task(
    status_sender: &mpsc::Sender<TaskNumber>,
    running_task: &mut TaskNumber,
    to_run: &mut Arc<Mutex<Box<dyn Runable + Send>>>,
    self_id: usize,
) -> Result<(), ()> {
    if let Err(_error) = status_sender.send(TaskNumber::Running(running_task.get_task_number())) {
        return Err(());
    }
    if let Ok(guard) = to_run.lock() {
        guard.as_ref().run();
    }
    if let Err(_error) = status_sender.send(TaskNumber::Finished(
        running_task.get_task_number(),
        self_id,
    )) {
        return Err(());
    }
    Ok(())
}

fn get_closure(
    sender: mpsc::Sender<TaskNumber>,
    reciever: mpsc::Receiver<Task<TaskNumber>>,
    id: usize,
) -> impl FnOnce() -> Result<(), String> {
    move || -> Result<(), String> {
        let status_sender = sender;
        let mut task_channel = reciever;
        let mut running_task: TaskNumber = TaskNumber::NotRunning(0);
        let mut to_run: Arc<Mutex<Box<dyn Runable + Send>>> =
            Arc::new(Mutex::new(Box::new(RunableDummy)));
        while validate_task(&mut task_channel, &mut running_task, &mut to_run) {
            if run_task(&status_sender, &mut running_task, &mut to_run, id).is_err() {
                println!("Failed to run task: {}", running_task.get_task_number());
            }
        }

        Ok(())
    }
}

fn thread_initialization(
    number_of_threads: usize,
    ready_queue: &mut Queue<(JoinHandle<Result<(), String>>, usize)>,
    recievers: &mut Vec<mpsc::Receiver<TaskNumber>>,
    senders: &mut Vec<mpsc::Sender<Task<TaskNumber>>>,
) {
    for i in 0..number_of_threads {
        let (task_number_sender, task_number_reciever) = mpsc::channel(); //Me quedo con los recievers
        let (task_sender, task_reciever) = mpsc::channel(); //Me quedo con los senders
        let closure = get_closure(task_number_sender, task_reciever, i);
        let thread = spawn(closure);
        ready_queue.put((thread, i));
        recievers.push(task_number_reciever);
        senders.push(task_sender);
    }
}
