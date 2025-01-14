use p3_field::FieldAlgebra;
use p3_mersenne_31::Mersenne31;

#[derive(Debug, Clone, PartialEq)]
pub enum Instructions {
    Push(Mersenne31),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct VMState {
    stack: [Mersenne31; 4],
    instruction: Instructions,
    extra_data: Mersenne31,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VM {
    stack: [Mersenne31; 4],
    sp: usize,
    instructions: Vec<Instructions>,
    ip: usize,
    trace: Vec<VMState>,
}

impl VM {
    pub fn new(instructions: Vec<Instructions>) -> Self {
        Self {
            stack: [Mersenne31::ZERO; 4],
            sp: 0,
            instructions,
            ip: 0,
            trace: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.ip < self.instructions.len() {
            let instruction = self.instructions[self.ip].clone();
            self.ip += 1;

            type BinaryOp = fn(Mersenne31, Mersenne31) -> Mersenne31;
            let mut extra_data: Mersenne31 = Mersenne31::ZERO;
            match &instruction {
                Instructions::Push(val) => {
                    for i in (1..self.stack.len()).rev() {
                        self.stack[i] = self.stack[i - 1];
                    }
                    self.stack[0] = *val;
                }
                Instructions::Add => {
                    self.run_opcode(|a, b| a + b, None::<BinaryOp>)?;
                }
                Instructions::Sub => {
                    self.run_opcode(|a, b| a - b, None::<BinaryOp>)?;
                }
                Instructions::Mul => {
                    self.run_opcode(|a, b| a * b, None::<BinaryOp>)?;
                }
                Instructions::Div => {
                    extra_data = self.run_opcode(|a, b| a / b, Some(|a, b| a - (a / b) * b))?;
                }
            }

            self.trace.push(VMState {
                stack: self.stack,
                instruction,
                extra_data,
            });
        }

        Ok(())
    }

    /// Runs an opcode on the top two values of the stack
    ///
    /// `opcode` is the operation to perform on the top two values.
    /// `sub_opcode` is an optional secondary operation to perform on the top two values.
    /// The result of the `sub_opcode` is stored in the `extra_data` field of the `VMState`.
    ///
    /// The top two values are popped from the stack and replaced with the result of the `opcode`.
    /// The `extra_data` value is pushed onto the stack as the new top value.
    fn run_opcode<F, G>(&mut self, opcode: F, sub_opcode: Option<G>) -> Result<Mersenne31, String>
    where
        F: Fn(Mersenne31, Mersenne31) -> Mersenne31,
        G: Fn(Mersenne31, Mersenne31) -> Mersenne31,
    {
        // load value  from stack
        let a = self.stack[0];
        let b = self.stack[1];

        let mut extra_data = Mersenne31::ZERO;
        let result = opcode(a, b);

        if let Some(sub_opcode) = sub_opcode {
            extra_data = sub_opcode(a, b);
        }

        for i in 2..4 {
            self.stack[i - 1] = self.stack[i];
        }
        self.stack[0] = result;
        self.stack[3] = Mersenne31::ZERO;
        Ok(extra_data)
    }

    pub fn get_stack(&self) -> [Mersenne31; 4] {
        self.stack
    }

    pub fn get_trace(&self) -> Vec<[Mersenne31; 11]> {
        let mut final_trace: Vec<[Mersenne31; 11]> = vec![[Mersenne31::ZERO; 11]];
        for i in self.trace.iter() {
            match i.instruction {
                Instructions::Push(val) => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        val,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Add => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Sub => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Mul => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        Mersenne31::ZERO,
                        i.extra_data,
                    ]);
                }
                Instructions::Div => {
                    final_trace.push([
                        i.stack[0],
                        i.stack[1],
                        i.stack[2],
                        i.stack[3],
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ZERO,
                        Mersenne31::ONE,
                        i.extra_data,
                    ]);
                }
            }
        }
        final_trace
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn check_sub_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 30
            Instructions::Push(Mersenne31::from_canonical_u32(30)), // Push 20
            Instructions::Sub,                                      // Add top two values (30-20)
        ];

        let mut vm = VM::new(program);

        if let Err(error) = vm.run() {
            println!("Error: {}", error);
            return;
        };

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(20),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }

    #[test]
    fn check_add_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 30
            Instructions::Push(Mersenne31::from_canonical_u32(30)), // Push 20
            Instructions::Add,                                      // Add top two values (30+20)
        ];

        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("Error: {}", error);
            return;
        };

        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(40),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }

    #[test]
    fn check_mul_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 30
            Instructions::Push(Mersenne31::from_canonical_u32(30)), // Push 20
            Instructions::Mul,                                      // Add top two values (30*20)
        ];
        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("Error: {}", error);
            return;
        };
        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(300),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }
    #[test]
    fn check_div_operation() {
        let program = vec![
            Instructions::Push(Mersenne31::from_canonical_u32(10)), // Push 30
            Instructions::Push(Mersenne31::from_canonical_u32(30)), // Push 20
            Instructions::Div,                                      // Add top two values (30/20)
        ];
        let mut vm = VM::new(program);
        if let Err(error) = vm.run() {
            println!("Error: {}", error);
            return;
        };
        assert_eq!(
            vm.stack,
            [
                Mersenne31::from_canonical_u32(3),
                Mersenne31::ZERO,
                Mersenne31::ZERO,
                Mersenne31::ZERO
            ]
        );
    }
}
