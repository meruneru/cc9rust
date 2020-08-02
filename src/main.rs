#[macro_use]
extern crate log;
use std::env;

extern crate cc9rust;
use cc9rust::strtol;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Sign, // 記号
    Num,  // 整数トークン
    Eof,  // 入力の終わりを表すトークン
}

// トークン型
#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind, // トークンの型
    val: i64,        // kindがTK_NUMの場合、その数値
    c: String,       // トークン文字列
}

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

    //let token = tokenize(&argv[1]);

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
    }

    println!("  ret");
    Ok(())
}

//let token = tokenize("12 + 2 - 4");
fn tokenize(s: String) -> Vec<Token> {
    let mut sentence = s;
    let mut tokens = vec![];
    while let Some(c) = sentence.chars().next() {
        //print!("{}", c);
        if c.is_whitespace() {
            // next char
            sentence = sentence.split_off(1);
            continue;
        }
        if c == '+' || c == '-' {
            let token = Token {
                kind: TokenKind::Sign,
                c: sentence.clone(),
                val: 0,
            };
            tokens.push(token);
            sentence = sentence.split_off(1);
            continue;
        }
        if c.is_numeric() {
            let (v, s) = strtol(&sentence);
            let token = Token {
                kind: TokenKind::Num,
                c: sentence.clone(),
                val: v.unwrap(),
            };
            tokens.push(token);
            sentence = s.to_string();
            continue;
        }
        error!("Unexpected char: {}", c);
    }
    // EOF
    let token = Token {
        kind: TokenKind::Eof,
        c: sentence.clone(),
        val: 0,
    };
    tokens.push(token);
    println!("{:?}", tokens);
    tokens
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strtol() {
        //(123, "+20") = strtol("123+20")
        let (v, s) = strtol("123+20");
        assert_eq!(v.unwrap(), 123);
        assert_eq!(s, "+20");
        //(20, "") = strtol("20")
        let (v, s) = strtol(&s[1..]); //20
        assert_eq!(v.unwrap(), 20);
        assert_eq!(s, "");
        //(None, "") = strtol("")
        let (v, s) = strtol("");
        assert_eq!(v, None);
        assert_eq!(s, "");
    }
    #[test]
    fn test_tokenize() {
        let tokens = tokenize("12 + 2 - 4".to_string());
        assert_eq!(tokens.len(), 6); // Eof included
        assert_eq!(
            tokens[0],
            Token {
                kind: TokenKind::Num,
                val: 12,
                c: "12 + 2 - 4".to_string(),
            }
        );
        assert_eq!(
            tokens[1],
            Token {
                kind: TokenKind::Sign,
                val: 0,
                c: "+ 2 - 4".to_string(),
            }
        );
    }
}
