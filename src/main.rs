use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Hello, world!");
}

struct Machine {
    code: VecDeque<String>,
    stack: Vec<u64>,
}

impl Machine {
    fn new() -> Self {
        Machine {
            code: VecDeque::new(),
            stack: Vec::new(),
        }
    }

    fn compile(&mut self, src: String) {
        self.code.extend(
            src.split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    fn step(&mut self) {
        let op = &*self.code.pop_front().unwrap();
        match op {
            "." => {
                println!("{:}", self.stack.pop().unwrap())
            }
            "+" => {
                let x = self.stack.pop().unwrap();
                let y = self.stack.pop().unwrap();
                self.stack.push(x + y);
            }
            num => {
                let x = num.parse::<u64>().unwrap();
                self.stack.push(x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::*;

    #[test]
    fn code_plus() {
        let mut machine = Machine::new();
        machine.compile("1 2 +".to_string());
        machine.step();
        machine.step();
        machine.step();
        let actual = machine.stack.pop().unwrap();
        assert_eq!(actual, 3)
    }
}
