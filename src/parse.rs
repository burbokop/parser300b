use crate::grammar::*;
use crate::tree::*;
use crate::combination::*;
use crate::ctx::*;

use colored::*;

pub type Error = String;

pub fn do_production<'t, 'g>(ctx: &Ctx<'t, 'g>, production: &'g Production) -> Vec<Result<ParseTree<'t, 'g>, Error>> {
    println!("{:<48}{:#}", format!("->{}{}", "`".repeat(ctx.level), production.lhs), ctx);

    //println!("{}", format!("do_production: {}, {:?}", ctx, production).yellow());
    let r = production
        .rhs
        .iter()
        .map(|expression| do_expression(ctx, &production.lhs, expression))
        .flatten()
        .collect();

    //println!("{}", format!("do_production end: {:?}", r).yellow().on_black());
    r
}


pub fn do_term<'t, 'g>(ctx: &Ctx<'t, 'g>, term: &'g Term) -> Vec<Result<ParseTreeNode<'t, 'g>, Error>> {
    println!("{:<48}{:#}", format!("T {}{}", "`".repeat(ctx.level), term), ctx);

    //println!("{}", format!("do_term: {}, {:?}", ctx, term).magenta());
    let r = match term {
        Term::Terminal(terminal) => {
            if ctx.len() == 1 {
                if ctx.front() == terminal {
                    vec![ Ok(ParseTreeNode::Terminal(ctx.front())) ]
                } else {
                    vec![ Err(format!("front token '{}' is not given terminal '{}'", ctx.front(), terminal)) ]
                }
            } else {
                vec![ Err(format!("Ctx len is not 1 on {} != {:#}", term, ctx)) ]
            }
        },
        Term::Nonterminal(nonterminal) => {
            if let Some(p) = ctx.grammar.productions.iter().find(|p| &p.lhs == nonterminal) {
                do_production(&ctx.next_level(), p)
                    .into_iter()
                    .map(|tree| 
                        tree.map(|tree| ParseTreeNode::Nonterminal(tree))
                    )
                    .collect()
            } else {
                vec![ Err(format!("production '{}' not found", nonterminal)) ]
            }
        },
    };
    //println!("{}", format!("do_term end: {:?}", r).magenta().on_black());
    r
}

pub fn do_expression<'t, 'g>(ctx: &Ctx<'t, 'g>, production_name: &'g String, expression: &'g Expression) -> Vec<Result<ParseTree<'t, 'g>, Error>> {
    println!("{:<48}{:#}", format!("E {}{}", "`".repeat(ctx.level), VecDisplay { v: expression.terms.iter().collect() }), ctx);

    for c in ctx.combinations(expression.terms.len()) {
        println!("{:<48}{:#}", format!("C {}", "`".repeat(ctx.level)), VecDisplay { v: ctx.split(c) });
    }
    

    //println!("{}", format!("do_expression: {}, '{}', {:?}", ctx, production_name, expression).blue());
    let r = ctx.combinations(expression.terms.len()).into_iter().map(|combination|{
        //println!("{}", format!("\tcombination: {:?}, {}", combination, VecDisplay { v: ctx.split(combination.clone()) }).blue().italic());

        expand_combinations(
            ctx
                .split(combination)
                .iter()
                .zip(expression.terms.iter())
                .map(|(subctx, term): (&Ctx<'t, 'g>, _)| do_term(subctx, term))
                .collect()
        ).into_iter().map(|subcombination| {
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
    
        })
    })
        .flatten()
        .collect();

    //println!("{}", format!("do_expression end: {:?}", r).blue().on_black());
    r
}

pub fn parse<'t, 'g>(grammar: &'g Grammar, tokens: &'t Vec<Token>) -> Vec<Result<ParseTree<'t, 'g>, Error>> {
    println!("input: {:?} <- {:#?}", tokens, grammar);

    let mut ctx = Ctx {
        begin: 0,
        end: tokens.len(),
        tokens: &tokens,
        grammar: grammar,
        level: 0
    };

    if let Some(first) = grammar.productions.first() {
        do_production(&mut ctx, first)
    } else {
        vec![ Err("grammar is empty".to_string()) ]
    }
}

#[cfg(test)]
mod tests {
    use crate::combination::Combination;
    use crate::ctx::{Ctx, VecDisplay};
    use crate::{
        parse, 
        Error, 
        assert_contains_tree
    };
    use crate::grammar::Grammar;

    use trim_margin::MarginTrimmable;

    #[test]
    fn aaa() {


        let mut cc: Vec<Combination> = Vec::new();

        

        for i in 0..(3-1) {
            for j in 1..7 {
                for k in 1..7 {
                    for l in 1..7 {
                        if j + k + l < 7 {
                            println!("{}, {}, {}", j, j + k, j + k + l);
                            cc.push(Combination { marks: vec![ j, j + k, j + k + l ] })
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let tokens = Vec::new();
        let grammar = Grammar { productions: vec![] };
        let ctx = Ctx { begin: 4, end: 9, tokens: &tokens, grammar: &grammar, level: 0 };


        for c in cc {
            println!("{}", VecDisplay { v: ctx.split(c) })
        }

    }

    #[test]
    fn postfix_test() {
        assert_contains_tree!(
            r#"
                <b> := <a> | <b> . <a>
                <a> := N
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
                <c> := K W = <b>
                <b> := W | <b> . W
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
                <block> := <subs> | <subs> ; <block>
                <subs> := W
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
                <block> := <subs> | <subs> ; <block>
                <subs> := Y = X
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

    //#[test]
    fn block_subs_test() {
        assert_contains_tree!(
            r#"
                <block> := <subs> | <subs> ; <block>
                <subs> := <lhs> = <rhs>
                <lhs> := Y
                <rhs> := X
            "#,
            [
                "Y", "=", "X", ";",
                "Y", "=", "X",
            ],
            r#"
                |block
                |`subs
                |``lhs
                |```X
                |``=
                |``rhs
                |```Y
                |`;
                |`block
                |``subs
                |```lhs
                |````X
                |```=
                |```rhs
                |````Y
            "#
        );
    }


    //#[test]
    fn block_postfix_test() {
        assert_contains_tree!(
            r#"
                <block> := <subs> | <subs> ; <block>
                <subs> := <lhs> = <rhs>
                <lhs> := ID
                <rhs> := <expr> | <rhs> . <expr>
                <expr> := W
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
}
