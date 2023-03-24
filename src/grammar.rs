use std::{
    fmt::Display, 
    str::Chars
};

use crate::combination::{expand_combinations, expand_combinations_iter};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct OptTerm {
    pub term: Term,
    pub is_optional: bool
}

impl OptTerm {
    pub fn opt(t: Term) -> Self {
        Self { term: t, is_optional: true }
    }
    /// obligatory
    pub fn obl(t: Term) -> Self {
        Self { term: t, is_optional: false }
    }
    pub fn into_obl(self) -> Self {
        Self { term: self.term, is_optional: false }
    }
}

impl Display for OptTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_optional {
            f.write_fmt(format_args!("{}?", self.term))        
        } else {
            f.write_fmt(format_args!("{}", self.term))        
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub terms: Vec<OptTerm>,
}

impl Expression {
    pub fn flatten(self) -> Vec<Expression> {
        expand_combinations_iter(self.terms.into_iter().map(|t| {
            if t.is_optional {
                vec![ Some(t.into_obl()), None ].into_iter()                
            } else {
                vec![ Some(t) ].into_iter()
            }
        })).map(|terms| {
            Expression { terms: terms.flatten().collect() }
        }).collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Expression>,
}

impl Production {
    pub fn flatten(self) -> Production {
        Production { 
            lhs: self.lhs, 
            rhs: self.rhs
                .into_iter()
                .map(|t|t.flatten())
                .flatten()
                .collect()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grammar {
    pub productions: Vec<Production>,
}

impl Grammar {
    /// removes all optional term conveting them into more expression valiants
    pub fn flatten(self) -> Grammar {
        Grammar { 
            productions: self.productions
                .into_iter()
                .map(|prod| prod.flatten())
                .collect() 
        }
    }
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

fn parse_opt_term(term: Chars<'_>) -> OptTerm {
    let vec: Vec<_> = term.clone().collect();
    if vec.len() > 1 && vec[vec.len() - 1] == '?' {
        OptTerm { term: parse_term(String::from_iter(vec[0..vec.len() - 1].into_iter()).chars()), is_optional: true }
    } else {
        OptTerm { term: parse_term(term), is_optional: false }
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
                                            .map(|term| parse_opt_term(term.chars()))
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
    use crate::{grammar::{Term, Production, Expression}, OptTerm};

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
                                        OptTerm::obl(Term::Nonterminal(
                                            "expr".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "t".to_string(),
                                        )),
                                    ],
                                },
                            ],
                        },
                        Production {
                            lhs: "expr".to_string(),
                            rhs: vec![
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "id".to_string(),
                                        )),
                                        OptTerm::obl(Term::Terminal(
                                            "=".to_string(),
                                        )),
                                        OptTerm::obl(Term::Terminal(
                                            "val".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "t".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "c".to_string(),
                                        )),
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

    #[test]
    fn try_from_opt_test() {
        let grammar: Result<Grammar, _> = r#"
            <block> ::= <expr>? | t
            <expr> ::= id = val? | t | c
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
                                        OptTerm::opt(Term::Nonterminal(
                                            "expr".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "t".to_string(),
                                        )),
                                    ],
                                },
                            ],
                        },
                        Production {
                            lhs: "expr".to_string(),
                            rhs: vec![
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "id".to_string(),
                                        )),
                                        OptTerm::obl(Term::Terminal(
                                            "=".to_string(),
                                        )),
                                        OptTerm::opt(Term::Terminal(
                                            "val".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "t".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Terminal(
                                            "c".to_string(),
                                        )),
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

    //#[test]
    fn try_from_or_test() {
        let grammar: Result<Grammar, _> = r#"
            <expr>     ::= <list> | <list> "|" <expr>
        "#.try_into();

        match grammar {
            Ok(grammar) => {
                println!("grammar: {:#?}", &grammar);
                assert_eq!(grammar, Grammar {
                    productions: vec![
                        Production {
                            lhs: "expr".to_string(),
                            rhs: vec![
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Nonterminal(
                                            "list".to_string(),
                                        )),
                                    ],
                                },
                                Expression {
                                    terms: vec![
                                        OptTerm::obl(Term::Nonterminal(
                                            "list".to_string(),
                                        )),
                                        OptTerm::obl(Term::Terminal(
                                            "|".to_string(),
                                        )),
                                        OptTerm::obl(Term::Nonterminal(
                                            "expr".to_string(),
                                        )),
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
