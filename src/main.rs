use p3_field::FieldAlgebra;
use p3_mersenne_31::Mersenne31;
use zkvm_demo::{vm::*,config::*};

fn main() {
    println!("Hello, world!");

    let mut vm = VM::new(vec![
        Instructions::Push(Mersenne31::from_canonical_u32(1)),
        Instructions::Push(Mersenne31::from_canonical_u32(2)),
        Instructions::Add,
        Instructions::Push(Mersenne31::from_canonical_u32(3)),
        Instructions::Push(Mersenne31::from_canonical_u32(3)),
        Instructions::Mul,
    ]);

    vm.run().unwrap();
    println!("{:?}", vm.get_trace()[0]);
    println!("{:?}", vm.get_stack());

    let vmair = VMAir {};
    vmair.generate_proof(vm);
}
