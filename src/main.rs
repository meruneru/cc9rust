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

    let tokens = tokenize(args[1].clone());
    let mut i = 0;
    println!("  mov rax, {}", expect_number(&tokens, &mut i));

    while at_eof(&tokens, &i) == false {
        if consume(&tokens, &mut i, '+' as i64) {
            println!("  add rax, {}", expect_number(&tokens, &mut i));
            continue;
        }

        expect(&tokens, &mut i, '-' as i64);
        println!("  sub rax, {}", expect_number(&tokens, &mut i));
    }
    println!("  ret");
    Ok(())
}

//let token = tokenize("12 + 2 - 4");
fn tokenize(s: String) -> Vec<Token> {
    let mut sentence = s.clone();
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
                val: c as i64,
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
    tokens
}

// 次のtokenが期待しているopの場合、読み進めて真を返す
fn consume(tokens: &Vec<Token>, i: &mut usize, op: i64) -> bool {
    if tokens[*i].kind != TokenKind::Sign || tokens[*i].val != op {
        return false;
    }
    *i += 1;
    true
}

// 次のtokenが期待しているopの場合、読み進める
fn expect(tokens: &Vec<Token>, i: &mut usize, op: i64) {
    if tokens[*i].kind != TokenKind::Sign || tokens[*i].val != op {
        panic!("not a operand!");
    }
    *i += 1;
}

// 次のtokenが数値の場合、読み進めてその数値を返す
fn expect_number(tokens: &Vec<Token>, i: &mut usize) -> i64 {
    if tokens[*i].kind != TokenKind::Num {
        panic!("not a number!");
    }
    let val = tokens[*i].val;
    *i += 1;
    val
}

fn at_eof(tokens: &Vec<Token>, i: &usize) -> bool {
    tokens[*i].kind == TokenKind::Eof
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
        assert_eq!(tokens.len(), 6);
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
                val: '+' as i64,
                c: "+ 2 - 4".to_string(),
            }
        );
    }
}
