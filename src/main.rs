use std::{io::{stdin, stdout, Write}, process::exit, thread, time};

use colored::*;
use parser300b::{Grammar, parse, make_ctx, ExtGrammar};


fn main() {
    let grammar: ExtGrammar = r#"
        <c> ::= "K" "W" "=" <b>
        <b> ::= "W" | <b> "." "W"
    "#.try_into().unwrap();

    let grammar = grammar.flatten();

    println!("grammar: {:#?}", grammar);

    let tokens = vec![
        "K", "W", "=", "W", ".", "W", ".", "W"
    ];

    let tokens: Vec<_> = tokens
        .into_iter()
        .map(|x| String::from(x))
        .collect();

    let ctx = make_ctx(&grammar, &tokens, true, false);
    let mut a = parse(ctx);

    println!("q - quit");
    println!("p - switch print result (default = true)");
    println!("go - go until success");
    println!("start");

    let mut print_res: bool = true;
    let mut go_until_success: bool = false;

    loop {
        let mut line: String = String::new();

        if !go_until_success {
            print!(">> ");
            let _ = stdout().flush();
    
            stdin().read_line(&mut line).unwrap();
            line.pop();

            if line == "q" {
                exit(0);
            }
            if line == "p" {
                print_res = !print_res;
                println!("print_res = {}", print_res);
                thread::sleep(time::Duration::from_millis(3000));
            }
            if line == "go" {
                go_until_success = true
            }
        }

        println!("{}", format!("going next...").blue());

        if print_res {
            match a.next() {
                Some(tree) => match tree {
                    Ok(tree) => {
                        println!("{}", format!("tree: {:#}", tree).green());
                        go_until_success = false
                    },
                    Err(err) => println!("{}", format!("err: {}", err).red()),
                },
                None => exit(0),
            }
        } else {
            match a.next() {
                Some(_) => {},
                None => exit(0),
            }
        }
        println!("{}", format!("OK").blue());

        //match a {
        //    Ok(tree) => println!("{:#}", tree),
        //    Err(err) => println!("{}", err),
        //}
    }

}