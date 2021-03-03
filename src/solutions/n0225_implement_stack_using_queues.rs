/**
 * [225] Implement Stack using Queues
 *
 * Implement a last in first out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal queue (push, top, pop, and empty).
 * Implement the MyStack class:
 * 
 * 	void push(int x) Pushes element x to the top of the stack.
 * 	int pop() Removes the element on the top of the stack and returns it.
 * 	int top() Returns the element on the top of the stack.
 * 	boolean empty() Returns true if the stack is empty, false otherwise.
 * 
 * Notes:
 * 
 * 	You must use only standard operations of a queue, which means only push to back, peek/pop from front, size, and is empty operations are valid.
 * 	Depending on your language, the queue may not be supported natively. You may simulate a queue using a list or deque (double-ended queue), as long as you use only a queue's standard operations.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["MyStack", "push", "push", "top", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * Output
 * [null, null, null, 2, 2, false]
 * Explanation
 * MyStack myStack = new MyStack();
 * myStack.push(1);
 * myStack.push(2);
 * myStack.top(); // return 2
 * myStack.pop(); // return 2
 * myStack.empty(); // return False
 * 
 *  
 * Constraints:
 * 
 * 	1 <= x <= 9
 * 	At most 100 calls will be made to push, pop, top, and empty.
 * 	All the calls to pop and top are valid.
 * 
 *  
 * Follow-up: Can you implement the stack such that each operation is <a href="https://en.wikipedia.org/wiki/Amortized_analysis" target="_blank">amortized</a> O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer. You can use more than two queues.
 */
struct MyStack {
    q: std::collections::VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack{
            q: std::collections::VecDeque::new()
        }
    }
    
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.q.push_front(x);
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }
    
    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_225() {
        let mut obj = MyStack::new();
        obj.push(2);
        obj.push(3);
        assert_eq!(obj.top(), 3);
        assert_eq!(obj.pop(), 3);
        assert_eq!(obj.pop(), 2);
        assert!(obj.empty());
    }
}