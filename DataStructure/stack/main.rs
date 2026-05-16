pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack(Vec::new())
    }
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

fn main(){
    let mut stack: Stack<i32> = Stack::new();

    stack.push(22);
    stack.push(222);
    stack.push(111);
    stack.push(67);

    println!("size: {}", stack.size());
    println!("peek {:?}", stack.peek());
    println!("pop {:?}", stack.pop());
}