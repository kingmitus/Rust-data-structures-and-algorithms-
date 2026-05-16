use std::collections::VecDeque;

pub struct Queue<T>(VecDeque<T>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(VecDeque::new())
    }

    pub fn enqueue(&mut self, item: T){
        self.0.push_back(item);
    }
    pub fn dequeue(&mut self) -> Option<T> {
        self.0.pop_front()
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.front()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn size(&self) -> usize {
        self.0.len()
    }
}




fn main() {
    println!("queue in rust ");

    let mut  queue: Queue<i32> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(11);
    queue.enqueue(111);

    println!("size:  {}", queue.size());       
    println!("peek:  {:?}", queue.peek());     
    println!("dequeue: {:?}", queue.dequeue()); 
    println!("dequeue: {:?}", queue.dequeue()); 
    println!("empty: {}", queue.is_empty())

}
