/*
    El trait Runable permite hacer que los threads de ThreadPool
    puedan correr algo que implemente este trait.

    La idea, es que se puedan armar wrappers de cosas que implementen run
    y el thread lo corra, avise que corrio y termine
*/

pub trait Runable {
    fn run_mut(&mut self);

    fn run(&self);
}
