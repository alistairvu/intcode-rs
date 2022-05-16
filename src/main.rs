use crate::vm::VM;
use std::env::args;

mod constants;
mod param;
mod vm;

fn main() {
    let args = args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: cargo run [filename]");
        return;
    }

    let mut vm = VM::new();
    vm.read(&args[1]);
    vm.run();
    // println!("{:?}", vm);
}
