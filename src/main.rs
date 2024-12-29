use p3_field::FieldAlgebra;
use p3_mersenne_31::Mersenne31;
use zkvm_demo::vm::*;

fn main() {
    println!("Hello, world!");

    let mut vm = VM::new(vec![
        Instructions::Push(Mersenne31::from_canonical_u32(1)),
        Instructions::Push(Mersenne31::from_canonical_u32(2)),
        Instructions::Add,
        Instructions::Push(Mersenne31::from_canonical_u32(3)),
        Instructions::Mul,
        Instructions::Push(Mersenne31::from_canonical_u32(4)),
        Instructions::Div,
    ]);

    vm.run().unwrap();
    println!("{:?}", vm.get_trace()[0]);
}
