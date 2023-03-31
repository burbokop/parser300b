use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parser300b::{Grammar, ExtGrammar, make_ctx, parse};
use trim_margin::MarginTrimmable;


fn parse_benchmark(c: &mut Criterion) {
    let grammar: Grammar = {
        let result: ExtGrammar = r#"
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
        "#
            .try_into()
            .unwrap();
    
        result.flatten()
    };
    
    let tokens: Vec<String> = vec![
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
    ]
        .into_iter()
        .map(|x|x.to_string())
        .collect();

    let ctx = make_ctx(&grammar, &tokens, false, true);
    
    let tree = parse(ctx.clone())
        .find_map(|tree| tree.ok().map(|tree| format!("{:#}", tree)));

    assert_eq!(tree, Some(r#"
        |block
        |`stmt
        |``stmt
        |`;
        |`block
        |``stmt
        |```subs
        |````lhs
        |`````exp
        |````=
        |````postfix_rhs
        |`````rhs
        |``````call
        |```````callable_or_type_rhs
        |````````id
        |`````````ID
        |```````(
        |```````arg_list
        |````````arg
        |`````````postfix_rhs
        |``````````rhs
        |```````````call
        |````````````callable_or_type_rhs
        |`````````````id
        |``````````````ID
        |````````````(
        |````````````arg_list
        |`````````````arg
        |``````````````postfix_rhs
        |```````````````postfix_rhs
        |````````````````rhs
        |`````````````````call
        |``````````````````callable_or_type_rhs
        |```````````````````primitive_type
        |````````````````````int
        |``````````````````(
        |``````````````````)
        |```````````````.
        |```````````````id
        |````````````````ID
        |````````````)
        |```````)
        |``;
        |
    "#.trim_margin().unwrap()));

    // 430.93 s / 100 smaples
    // 4.6605 s
    c.bench_function("parse:first", |b| 
        b.iter(|| 
            parse(black_box(ctx.clone())).find(|t|t.is_ok())
        )
    );

    // 820.31 s / 100 smaples
    c.bench_function("parse:all", |b| 
        b.iter(|| 
            parse(black_box(ctx.clone()))
                .collect::<Vec<_>>()
        )
    );
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);