use crate::grammar::*;
use crate::tree::*;
use crate::combination::*;
use crate::ctx::*;

use colored::*;

pub type Error = String;

pub type ParseTreeIter<'tg, T> = Box<dyn Iterator<Item = Result<ParseTree<'tg, 'tg, T>, Error>> + 'tg>;
pub type ParseTreeNodeIter<'tg, T> = Box<dyn Iterator<Item = Result<ParseTreeNode<'tg, 'tg, T>, Error>> + 'tg>;

pub fn do_production<'tg, T: Token>(ctx: Ctx<'tg, 'tg, T>, production: &'tg Production) -> ParseTreeIter<'tg, T> {
    println!("{:<48}{:#}", format!("->{}{}", "`".repeat(ctx.level), production.lhs), ctx);

    //println!("{}", format!("do_production: {}, {:?}", ctx, production).yellow());
    let r = production
        .rhs
        .iter()
        .map(move |expression| do_expression(ctx.clone(), &production.lhs, expression))
        .flatten();

    //println!("{}", format!("do_production end: {:?}", r).yellow().on_black());
    Box::new(r)
}


pub fn do_term<'tg, T: Token>(ctx: Ctx<'tg, 'tg, T>, term: &'tg Term) -> ParseTreeNodeIter<'tg, T> {
    println!("{:<48}{:#}", format!("T {}{}", "`".repeat(ctx.level), term), ctx);

    //println!("{}", format!("do_term: {}, {:?}", ctx, term).magenta());
    let r = match term {
        Term::Terminal(terminal) => {
            Box::new(if ctx.len() == 1 {
                if ctx.front().name() == terminal {
                    vec![ Ok(ParseTreeNode::Terminal(ctx.front())) ]
                } else {
                    vec![ Err(format!("front token '{}' is not given terminal '{}'", ctx.front(), terminal)) ]
                }
            } else {
                vec![ Err(format!("Ctx len is not 1 on {} != {:#}", term, ctx)) ]
            }.into_iter())
        },
        Term::Nonterminal(nonterminal) => {
            if let Some(p) = ctx.grammar.productions.iter().find(|p| &p.lhs == nonterminal) {
                Box::new(
                    do_production(ctx.next_level(), p)
                        .map(|tree| 
                            tree.map(|tree| ParseTreeNode::Nonterminal(tree))
                        )
                )
            } else {
                Box::new(vec![ Err(format!("production '{}' not found", nonterminal)) ].into_iter()) 
                    as ParseTreeNodeIter<T>
            }
        },
    };
    //println!("{}", format!("do_term end: {:?}", r).magenta().on_black());
    r
}

pub fn do_expression<'tg, T: Token>(ctx: Ctx<'tg, 'tg, T>, production_name: &'tg String, expression: &'tg Expression) -> ParseTreeIter<'tg, T> {
    println!("{:<48}{:#}", format!("E {}{}", "`".repeat(ctx.level), VecDisplay { v: expression.terms.iter().collect() }), ctx);

    for c in ctx.combinations(expression.terms.len()) {
        println!("{:<48}{:#}", format!("C {}", "`".repeat(ctx.level)), VecDisplay { v: ctx.split(c) });
    }
    //let ctx = &ctx;

    //println!("{}", format!("do_expression: {}, '{}', {:?}", ctx, production_name, expression).blue());
    let r = ctx.combinations(expression.terms.len()).into_iter().map(move |combination|{
        //println!("{}", format!("\tcombination: {:?}, {}", combination, VecDisplay { v: ctx.split(combination.clone()) }).blue().italic());

        let a = expand_combinations_iter(
            ctx
                .split(combination)
                .into_iter()
                .zip(expression.terms.iter())
                .map(|(subctx, term): (Ctx<'tg, 'tg, T>, _)| do_term(subctx, term))
        ).map(|subcombination| {
            //println!("{}", format!("\t\tsubcombination: {:?}", subcombination).blue().italic());



            let mut tree = ParseTree { lhs: production_name, rhs: vec![] };
            let mut error: Option<Error> = None;

            for t in subcombination {        
                match t {
                    Ok(node) => tree.rhs.push(node),
                    Err(e) => {
                        error = Some(e);
                        break
                    }
                }
            }
    
            if let Some(error) = error {
                Err(error)
            } else {
                Ok(tree)
            }
    
        });
        a
    })
        .flatten();

    //println!("{}", format!("do_expression end: {:?}", r).blue().on_black());
    Box::new(r)
}


pub fn make_ctx<'tg, T: Token>(grammar: &'tg Grammar, tokens: &'tg Vec<T>) -> Ctx<'tg, 'tg, T> {
    Ctx {
        begin: 0,
        end: tokens.len(),
        tokens: &tokens,
        grammar: grammar,
        level: 0
    }
}

pub fn parse<'tg, T: Token>(ctx: Ctx<'tg, 'tg, T>) -> ParseTreeIter<'tg, T> {
    let err = Err("grammar is empty".to_string()) as Result<ParseTree<'tg, 'tg, T>, _>;

    if let Some(first) = ctx.grammar.productions.first() {
        let a = do_production(ctx, first);
        a
    } else {
        Box::new(std::iter::once(err)) as ParseTreeIter<'tg, T>
    }
}


//pub fn parse<'tg, T: Token>(grammar: &'tg Grammar, tokens: &'tg Vec<T>) -> ParseTreeIter<'tg, T> {
//    println!("input: {:?} <- {:#?}", tokens, grammar);
//
//    let mut ctx = Ctx {
//        begin: 0,
//        end: tokens.len(),
//        tokens: &tokens,
//        grammar: grammar,
//        level: 0
//    };
//    let err = Err("grammar is empty".to_string()) as Result<ParseTree<'tg, 'tg, T>, _>;
//
//    if let Some(first) = grammar.productions.first() {
//        let a = do_production(&mut ctx, first);
//        a
//    } else {
//        
//
//        Box::new(std::iter::once(err)) as ParseTreeIter<'tg, T>
//    }
//}

#[cfg(test)]
mod tests {
    use crate::{
        parse, 
        Error, 
        assert_contains_tree
    };
    use crate::grammar::Grammar;
    use trim_margin::MarginTrimmable;

    #[test]
    fn postfix_test() {
        assert_contains_tree!(
            r#"
                <b> ::= <a> | <b> . <a>
                <a> ::= N
            "#,
            [ "N", ".", "N", ".", "N" ],
            r#"
                |b
                |`b
                |``b
                |```a
                |````N
                |``.
                |``a
                |```N
                |`.
                |`a
                |``N
            "#
        );
    }

    #[test]
    fn preffix_postfix_test() {
        assert_contains_tree!(
            r#"
                <c> ::= K W = <b>
                <b> ::= W | <b> . W
            "#,
            [ "K", "W", "=", "W", ".", "W", ".", "W" ],
            r#"
                |c
                |`K
                |`W
                |`=
                |`b
                |``b
                |```b
                |````W
                |```.
                |```W
                |``.
                |``W
            "#
        );
    }

    #[test]
    fn block_test() {
        assert_contains_tree!(
            r#"
                <block> ::= <subs> | <subs> ; <block>
                <subs>  ::= W
            "#,
            [
                "W", ";",
                "W", ";",
                "W"
            ],
            r#"
                |block
                |`subs
                |``W
                |`;
                |`block
                |``subs
                |```W
                |``;
                |``block
                |```subs
                |````W
            "#
        );
    }

    #[test]
    fn block_subs_simple_test() {
        assert_contains_tree!(
            r#"
                <block> ::= <subs> | <subs> ; <block>
                <subs>  ::= Y = X
            "#,
            [
                "Y", "=", "X", ";",
                "Y", "=", "X",
            ],
            r#"
                |block
                |`subs
                |``Y
                |``=
                |``X
                |`;
                |`block
                |``subs
                |```Y
                |```=
                |```X
            "#
        );
    }

    #[test]
    fn block_subs_test() {
        assert_contains_tree!(
            r#"
                <block> ::= <subs> | <subs> ; <block>
                <subs>  ::= <lhs> = <rhs>
                <lhs>   ::= Y
                <rhs>   ::= X
            "#,
            [
                "Y", "=", "X", ";",
                "Y", "=", "X",
            ],
            r#"
                |block
                |`subs
                |``lhs
                |```Y
                |``=
                |``rhs
                |```X
                |`;
                |`block
                |``subs
                |```lhs
                |````Y
                |```=
                |```rhs
                |````X
            "#
        );
    }


    #[test]
    fn block_postfix_test() {
        assert_contains_tree!(
            r#"
                <block> ::= <subs> | <subs> ; <block>
                <subs>  ::= <lhs> = <rhs>
                <lhs>   ::= ID
                <rhs>   ::= <expr> | <rhs> . <expr>
                <expr>  ::= W
            "#,
            [ 
                "ID", "=", "W", ".", "W", ";", 
                "ID", "=", "W", ".", "W", ".", "W", ";", 
                "ID", "=", "W" 
            ],
            r#"
                |block
                |`subs
                |``lhs
                |```ID
                |``=
                |``rhs
                |```rhs
                |````expr
                |`````W
                |```.
                |```expr
                |````W
                |`;
                |`block
                |``subs
                |```lhs
                |````ID
                |```=
                |```rhs
                |````rhs
                |`````rhs
                |``````expr
                |```````W
                |`````.
                |`````expr
                |``````W
                |````.
                |````expr
                |`````W
                |``;
                |``block
                |```subs
                |````lhs
                |`````ID
                |````=
                |````rhs
                |`````expr
                |``````W
            "#
        );
    }

    static HARD_LVL_GRAMMAR: &str = r#"
        <syntax>         ::= <rule> | <rule> <syntax>
        <rule>           ::= "<" <rule_name> ">" "::=" <expression> <line_end>
        <expression>     ::= <list> | <list> "|" <expression>
        <line_end>       ::= "\n" | <line_end> <line_end>
        <list>           ::= <term> | <term> <list>
        <term>           ::= <literal> | "<" <rule_name> ">"
        <literal>        ::= "`" <text> "`"
        <text>           ::= <character> <text> | <character>
        <character>      ::= <letter> | <digit>
        <letter>         ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
        <digit>          ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        <rule_name>      ::= <letter> | <rule_name> <rule_char>
        <rule_char>      ::= <letter> | <digit> | "_"
    "#;

    #[test]
    fn hard_level_test() {
        assert_contains_tree!(
            HARD_LVL_GRAMMAR,
            [
                "<", "K", ">", "::=", "<", "L", "J", ">", "<", "q", ">", "\n"
            ],
            ""
        );
    }

    #[test]
    fn hard_level_test2() {
        assert_contains_tree!(
            HARD_LVL_GRAMMAR,
            [
                "<", "n", ">", "::=", "`", "2", "`", "<", "D", ">", "\n",
                "<", "j", ">", "::=", "<", "l", "q", "v", ">", "|", "<", "e", ">", "\n",
                "<", "m", "N", ">", "::=", "`", "5", "`", "\n"
            ],
            ""
        );    
    }

    #[test]
    fn hard_level_test3() {
        assert_contains_tree!(
            HARD_LVL_GRAMMAR,
            [
                "<", "p", "7", ">", "::=", "<", "t", ">", "\n",
                "<", "T", "_", ">", "::=", "`", "2", "1", "`", "<", "f", ">", "\n",
                "<", "y", ">", "::=", "<", "m", ">", "`", "y", "9", "`", "\n"
            ],
            ""
        );    
    }
}
