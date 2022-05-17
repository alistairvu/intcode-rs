use crate::vm::VM;
use clap::Parser;

mod constants;
mod param;
mod vm;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Intcode binary to interpret
    filename: String,

    /// Turn ascii mode on or off
    #[clap(short, long)]
    ascii: bool,
}

fn main() {
    let args = Args::parse();

    let mut vm = VM::new(args.ascii);
    vm.read(&args.filename);
    vm.run();
}
