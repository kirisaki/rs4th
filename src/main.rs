fn main() {
  println!("Hello, world!");
}

struct Stack<T> (Vec<T>);

impl<T> Stack<T> {
  fn new() -> Self {
    Stack(Vec::new())
  }

  fn push(&mut self, v: T) {
    self.0.push(v);
  }

  fn pop(&mut self) -> Option<T> {
    self.0.pop()
  }
}

#[cfg(test)]
mod tests {
  use crate::Stack;

  #[test]
  fn stack_pop() {
    let mut stack = Stack::<u64>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    let _ = stack.pop();
    assert_eq!(stack.pop(), Some(2))
  }
  #[test]
  fn stack_pop_failed() {
    let mut stack = Stack::<u64>::new();
    stack.push(1);
    let _ = stack.pop();
    assert_eq!(stack.pop(), None)
  }
}