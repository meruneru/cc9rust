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

/// strtol
/// str to long
/// 
/// ```
/// use super::*;
/// let (v, s) = strtol("123+20");
/// assert_eq!(v.unwrap(), 123);
/// assert_eq!(s, "+20");
/// ```
pub fn strtol(s: &str) -> (Option<i64>, &str) {
    if s.is_empty() || !char::is_numeric(s.chars().nth(0).unwrap()) {
        return (None, s);
    }
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    let (v, remain) = s.split_at(first_non_num);
    info!("v={:?}, remain: {:?}", v, remain);
    match v.parse::<i64>() {
        Ok(t) => (Some(t), remain),
        Err(_e) => (None, remain),
    }
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
