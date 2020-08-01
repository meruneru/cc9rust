#[macro_use]
extern crate log;
use std::env;

fn main() -> Result<(), i32> {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Command Line: {:?}", args);
        return Err(1);
    }
    info!("{:?} {}", args, args.len());
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", args[1]);
    println!("  ret");
    Ok(())
}
