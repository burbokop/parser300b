use parser300b::*;
use trim_margin::MarginTrimmable;

#[test]
fn hard_parse00_test() {
    assert_contains_tree!(
        r#"
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
        "#,
        [
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
        ],
        r#"
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
        "#
    );   
}