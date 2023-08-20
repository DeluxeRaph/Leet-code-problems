struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        //creates a new stack
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }
    
    fn push(&mut self, val: i32) {
        // This pushes to the stack
        self.stack.push(val);
        if self.min_stack.is_empty() || Some(&val) <= self.min_stack.last() {
            self.min_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        // This value is being removed from the stack. It will betaken from the top of the stack
        let val = self.stack.pop().unwrap(); 
        if Some(&val) == self.min_stack.last() {
            self.min_stack.pop();
        }
    
    }
    
    fn top(&self) -> i32 {
        // This is adding to the top of the stack
        self.stack.last().cloned().unwrap()

    }
    
    fn get_min(&self) -> i32 {
        // This is getting the minimum element
        self.min_stack.last().cloned().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */