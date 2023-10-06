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
mod tests1 {
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

struct Code (Vec<String>);

impl Code {
  fn parse(src :String) -> Self {
    Code(src.split_ascii_whitespace().map(|x| {
        x.to_string()
    }).collect())
  }

  fn step(&mut self, stack :&mut Stack<u64>) {
    let codes = self.0.clone();
    let (head, rest) = codes.split_first().unwrap();
    let op: &str = head;
    self.0 = rest.to_vec();
    match op {
      "." => {
        println!("{:}", stack.pop().unwrap())
      },
      "+" => {
        let x = stack.pop().unwrap();
        let y = stack.pop().unwrap();
        stack.push(x + y);
      }
      num => {
        let x = num.parse::<u64>().unwrap();
        stack.push(x);
      },
    }
  }
}

#[cfg(test)]
mod tests2 {
  use crate::{Stack, Code};

  #[test]
  fn code_plus() {
    let mut stack = Stack::<u64>::new();
    let mut code = Code::parse("1 2 +".to_string());
    code.step(&mut stack);
    code.step(&mut stack);
    code.step(&mut stack);
    let actual = stack.pop().unwrap();
    assert_eq!(actual, 3)
  }
}