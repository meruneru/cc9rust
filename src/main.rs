#[macro_use]
extern crate log;
use std::env;

extern crate cc9rust;
use cc9rust::strtol;

fn main() -> Result<(), i32> {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].is_empty() {
        error!("Command Line: {:?}", args);
        return Err(1);
    }
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let (v, s) = strtol(&args[1]);
    if let Some(t) = v {
        println!("  mov rax, {}", t);
    }
    let mut remain = s;
    while let Some(c) = remain.chars().nth(0) {
        if c == '+' {
            let (v, s) = strtol(&remain[1..]);
            if let Some(t) = v {
                println!("  add rax, {}", t);
            }
            remain = s;
            continue;
        }
        if c == '-' {
            let (v, s) = strtol(&remain[1..]);
            if let Some(t) = v {
                println!("  sub rax, {}", t);
            }
            remain = s;
            continue;
        }
        error!("Unexpected char: {}", c);
        return Err(1);
    }

    println!("  ret");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        env::set_var("RUST_LOG", "trace");
        env_logger::init();
    }
    #[test]
    fn test_strtol() {
        init();
        //(123, "+20") = strtol("123+20")
        let (v, s) = strtol("123+20");
        assert_eq!(v.unwrap(), 123);
        assert_eq!(s, "+20");
        //(20, "") = strtol("20")
        let (v, s) = strtol(&s[1..]); //20
        assert_eq!(v.unwrap(), 20);
        assert_eq!(s, "");
    }
}
