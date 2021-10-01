/*
    Implementacion simple de una cola.
*/

pub struct Queue<T> {
    queue: Vec<T>,

    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            queue: Vec::new(),

            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn put(&mut self, element: T) {
        self.queue.push(element);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Result<T, String> {
        if !self.is_empty() {
            let poped = self.queue.remove(0);
            self.length -= 1;
            return Ok(poped);
        }

        Err(String::from("Queue is empty"))
    }
}

#[cfg(test)]
mod test_queue {
    use crate::queue::Queue;
    #[test]
    fn new_queue_is_empty() {
        let queue: Queue<usize> = Queue::new();

        if !queue.is_empty() {
            panic!("Queue is not empty");
        }
    }

    #[test]
    fn after_adding_an_element_the_queue_is_not_empty() {
        let mut queue: Queue<usize> = Queue::new();

        queue.put(1);

        if queue.is_empty() {
            panic!("Queue is empty");
        }
    }

    #[test]
    fn after_adding_two_elements_they_come_out_in_the_same_order_they_got_int() {
        let mut queue = Queue::new();

        queue.put(1);

        queue.put(2);

        if let Ok(some) = queue.pop() {
            if some != 1 {
                panic!("The first one to get out was not a one");
            }
        }

        if let Ok(some) = queue.pop() {
            if some != 2 {
                panic!("The second one to get out was not a two");
            }
        }

        if !queue.is_empty() {
            panic!("The queue was not empty at the end");
        }
    }
}
