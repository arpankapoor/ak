#![feature(once_cell)]

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::process;

use crate::parser::Parser;
use crate::tok::Tokenizer;
use crate::util::TrimEnd;

mod error;
mod interpreter;
mod k;
mod parser;
mod span;
mod sym;
mod tok;
mod util;

fn print_banner() {
    println!(
        "{} {} (c){}\n",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
}

fn print_prompt() -> io::Result<()> {
    print!(" ");
    io::stdout().flush()
}

fn run(src: &[u8]) {
    match Tokenizer::new(src).collect::<Result<Vec<_>, _>>() {
        Ok(tokens) => {
            //for token in &tokens {
            //    //println!("({}, {:?}, {})", token.0, token.1, token.2);
            //    println!("{:?}", token);
            //}
            if tokens.is_empty() {
                return;
            }
            match Parser::new(tokens).parse() {
                Ok(Some(ast)) => {
                    //println!("{}", ast);
                    match ast.interpret() {
                        Ok(k) => println!("{}", k),
                        Err(e) => println!("interpreter error: {:?}", e),
                    }
                }
                Ok(None) => println!("empty!!!"),
                Err(e) => println!("parsing error: {:?}", e),
            }
        }
        Err(e) => println!("tokenizer error: {:?}", e),
    }
}

fn run_prompt() -> io::Result<()> {
    print_prompt()?;
    let stdin = io::stdin();
    let mut buf = Vec::new();
    while stdin.lock().read_until(b'\n', &mut buf)? > 0 {
        let line = buf.trim_end();
        if !line.is_empty() {
            if line == br"\\" {
                process::exit(0);
            } else {
                run(line);
            }
        }
        buf.clear();
        print_prompt()?;
    }
    println!();
    Ok(())
}

fn run_file(fname: OsString) -> io::Result<()> {
    run(&fs::read(fname)?);
    Ok(())
}

fn main() -> io::Result<()> {
    print_banner();
    let mut args = env::args_os();
    if args.len() > 2 {
        eprintln!("usage: {} [script]", env!("CARGO_BIN_NAME"));
        process::exit(64)
    } else {
        match args.nth(1) {
            Some(arg) => run_file(arg)?,
            None => run_prompt()?,
        }
    }
    Ok(())
}
