use std::{
    fmt::Display, 
    str::Chars
};

#[derive(Debug, PartialEq)]
pub enum Term {
    Terminal(String),
    Nonterminal(String),
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Terminal(t) => f.write_fmt(format_args!("'{}'", t)),
            Term::Nonterminal(n) => f.write_fmt(format_args!("<{}>", n)),
        }        
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub terms: Vec<Term>,
}

#[derive(Debug, PartialEq)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Grammar {
    pub productions: Vec<Production>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    LhsNotFound(String),
    WrongLhs(String),
    RhsNotFound(String)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::LhsNotFound(line) => f.write_fmt(format_args!("lhs not found in '{}'", line)),
            ParseError::WrongLhs(lhs) => f.write_fmt(format_args!("lhs must be '<***>' byt found '{}'", lhs)),
            ParseError::RhsNotFound(line) => f.write_fmt(format_args!("rhs not found in '{}'", line)),
        }        
    }
}

impl std::error::Error for ParseError {}

fn parse_term(term: Chars<'_>) -> Term {
    let vec: Vec<_> = term.collect();
    if vec.len() > 1 && vec[0] == '<' && vec[vec.len() - 1] == '>' {
        Term::Nonterminal(String::from_iter(vec[1..vec.len() - 1].into_iter()))
    } else if vec.len() > 1 && vec[0] == '"' && vec[vec.len() - 1] == '"' {
        Term::Terminal(String::from_iter(vec[1..vec.len() - 1].into_iter()))
    } else {
        Term::Terminal(String::from_iter(vec.into_iter()))
    }
}

fn parse_lhs(term: Chars<'_>) -> Result<String, ParseError> {
    let vec: Vec<_> = term.collect();
    if vec.len() > 1 && vec[0] == '<' && vec[vec.len() - 1] == '>' {
        Ok(String::from_iter(vec[1..vec.len() - 1].into_iter()))
    } else {
        Err(ParseError::WrongLhs(String::from_iter(vec.into_iter())))
    }
}

impl TryFrom<&str> for Grammar {
    type Error = ParseError;    

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result = Grammar { productions: vec![] };
        for line in value.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            let delim = "::=";
            match line.find(delim) {
                Some(split_pos) => {
                    Ok(result.productions.push(Production {
                        lhs: parse_lhs(line[0..split_pos].trim().chars())?,
                        rhs: line[(split_pos + delim.len())..line.len()].split("|").map(|expr| {                             
                                    Expression { 
                                        terms: expr
                                            .split(" ")
                                            .map(str::trim)
                                            .filter(|s| !s.is_empty())
                                            .map(|term| parse_term(term.chars()))
                                            .collect() 
                                    }
                                })
                                    .collect()
                    }))    
                },
                None => Err(ParseError::RhsNotFound(String::from(line))),
            }?;
        }
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use crate::grammar::{Term, Production, Expression};

    use super::{parse_term, Grammar};

    #[test]
    fn parse_term_test() {
        assert_eq!(parse_term("goga".chars()), Term::Terminal("goga".to_string()));
        assert_eq!(parse_term("<goga>".chars()), Term::Nonterminal("goga".to_string()));
    }

    #[test]
    fn try_from_test() {
        let grammar: Result<Grammar, _> = r#"
            <block> ::= <expr> | t
            <expr> ::= id = val | t | c
        "#.try_into();

        match grammar {
            Ok(grammar) => {
                println!("grammar: {:#?}", &grammar);
                assert_eq!(grammar, Grammar {
                    productions: vec![
                        Production {
                            lhs: "block".to_string(),
                            rhs: vec![
                                Expression {
                                    terms: vec![
                                        Term::Nonterminal(
                                            "expr".to_string(),
                                        ),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        Term::Terminal(
                                            "t".to_string(),
                                        ),
                                    ],
                                },
                            ],
                        },
                        Production {
                            lhs: "expr".to_string(),
                            rhs: vec![
                                Expression {
                                    terms: vec![
                                        Term::Terminal(
                                            "id".to_string(),
                                        ),
                                        Term::Terminal(
                                            "=".to_string(),
                                        ),
                                        Term::Terminal(
                                            "val".to_string(),
                                        ),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        Term::Terminal(
                                            "t".to_string(),
                                        ),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        Term::Terminal(
                                            "c".to_string(),
                                        ),
                                    ],
                                },
                            ],
                        },
                    ],
                })
            },
            Err(err) => panic!("{}", err),
        } 
        
    }
}
