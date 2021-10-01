mod queue;
mod runable;
mod task;
mod threadpool;

use runable::Runable;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

struct DigoHola;
struct DigoChau;

impl runable::Runable for DigoHola {
    fn run_mut(&mut self) {
        println!("Hola!");
    }

    fn run(&self) {
        println!("Hola inmutable!");
    }
}

impl runable::Runable for DigoChau {
    fn run_mut(&mut self) {
        println!("Chau");
    }

    fn run(&self) {
        println!("Hola inmutable!");
    }
}

struct RunANumber {
    number: usize,
}

impl runable::Runable for RunANumber {
    fn run(&self) {
        println!("Hi im: {}", self.number)
    }

    fn run_mut(&mut self) {}
}

struct PutToAVec{

    vec: Vec<usize>,

}

struct AddTheNumber{

    vec: Arc<Mutex<PutToAVec>>,

    number: usize

}

impl PutToAVec{

    pub fn new() -> PutToAVec {
        PutToAVec{
            vec: Vec::new()
        }
    }

    pub fn add(&mut self, number: usize){
        self.vec.push(number);
    }

    pub fn print_vec(&self){
        for number in self.vec.iter(){
            println!("Number: {}", number);
        }
    }

}

impl AddTheNumber{

    pub fn new(vec: Arc<Mutex<PutToAVec>>, number: usize) -> AddTheNumber{
        AddTheNumber{

            vec,

            number

        }
    }
}

impl Runable for AddTheNumber{

    fn run(&self){
        if let Ok(mut guard) = self.vec.lock(){
            guard.add(self.number);
        }
    }

    fn run_mut(&mut self){}

}

fn main() {
    let mut pool = ThreadPool::new(4);

    let vec = Arc::new(Mutex::new(PutToAVec::new()));

    for i in 1..9 {
        let new_task: Arc<Mutex<Box<dyn Runable + Send>>> =
            Arc::new(Mutex::new(Box::new(AddTheNumber::new(vec.clone(), i))));
        pool.spawn(new_task);
    }

    
    pool.join();

    if let Ok(guard) = vec.lock(){
        guard.print_vec();
        drop(guard);
    }

    drop(vec);
    
}
