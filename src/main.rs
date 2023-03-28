use std::{io::{stdin, stdout, Write}, process::exit, thread, time};

use colored::Colorize;
use parser300b::{Grammar, parse, make_ctx, ExtGrammar};


fn main() {
    let grammar: ExtGrammar = r#"
        <block> ::= <stmt> ";" | <stmt> ";" <block>
        <namespace> ::= "namespace" "{" <block>? "}"
        <subs> ::= <lhs> "=" <postfix_rhs>
        <lhs> ::= <id> | "exp" | "_"
        <postfix_rhs> ::= <rhs> | <postfix_rhs> "." <id>
        <rhs> ::= <namespace> | <literal> | <subs> | <callable_or_type_rhs> | <call>
        <stmt> ::= <subs> | "stmt"
        <class_decl> ::= "class" "(" <decl_arg_list>? ")" "{" <block>? "}"
        <arg_list> ::= <arg> | <arg_list> "," <arg>
        <arg> ::= <postfix_rhs>
        <decl_arg_list> ::= <decl_arg> | <decl_arg_list> "," <decl_arg>
        <decl_arg> ::= <id> " " <callable_or_type_rhs>
        <primitive_type> ::= "int" | "float"
        <func_decl> ::= "(" <decl_arg_list>? ")" "{" <block>? "}"
        <callable_or_type_rhs> ::= <id> | <func_decl> | <primitive_type> | <class_decl>
        <call> ::= <callable_or_type_rhs> "(" <arg_list>? ")"
        <literal> ::= "STR" | "NUM"
        <id> ::= "ID"
    "#.try_into().unwrap();

    let grammar = grammar.flatten();

    println!("grammar: {:#?}", grammar);

    let tokens = vec![
        "stmt",
        ";",
        "exp",
        "=",
        "ID",
        "(",
        "ID",
        "(",
        "int",
        "(",
        ")",
        ".",
        "ID",
        ")",
        ")",
        ";",
    ];

    let tokens: Vec<_> = tokens
        .into_iter()
        .map(|x| String::from(x))
        .collect();

    let ctx = make_ctx(&grammar, &tokens, true, true);
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

        println!("going next...");
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
        println!("OK");

        //match a {
        //    Ok(tree) => println!("{:#}", tree),
        //    Err(err) => println!("{}", err),
        //}
    }

}