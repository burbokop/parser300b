use std::{ffi::{c_char, CStr, c_void}, str::Utf8Error, slice, fmt::Display};

use crate::{Term, Expression, Production, Grammar, ctx::Token, OptTerm};



#[repr(C)]
pub struct parser300b_Term {
    pub value: *const c_char,
    pub is_terminal: bool,
    pub is_optional: bool
}

impl parser300b_Term {
    pub unsafe fn to_non_c(&self) -> Result<OptTerm, Utf8Error> {
        let value = CStr::from_ptr(self.value).to_str()?;
        if self.is_terminal {
            Ok(OptTerm { term: Term::Terminal(value.to_string()), is_optional: self.is_optional })
        } else {
            Ok(OptTerm { term: Term::Nonterminal(value.to_string()), is_optional: self.is_optional })
        }
    }
}

#[repr(C)]
pub struct parser300b_Expression {
    pub terms: *const parser300b_Term,
    pub term_count: usize,
}

impl parser300b_Expression {
    pub unsafe fn to_non_c(&self) -> Result<Expression, Utf8Error> {        
        slice::from_raw_parts(self.terms, self.term_count)
            .iter()
            .map(|t| t.to_non_c())
            .collect::<Result<Vec<_>, _>>()
            .map(|t| Expression { terms: t })
    }
}

#[repr(C)]
pub struct parser300b_Production {
    pub lhs: *const c_char,
    pub rhs: *const parser300b_Expression,
    pub rhs_count: usize,
}

impl parser300b_Production {
    pub unsafe fn to_non_c(&self) -> Result<Production, Utf8Error> {
        let lhs = CStr::from_ptr(self.lhs).to_str()?;
        slice::from_raw_parts(self.rhs, self.rhs_count)
            .iter()
            .map(|t| t.to_non_c())
            .collect::<Result<Vec<_>, _>>()
            .map(|e| Production { lhs: lhs.to_string(), rhs: e })
    }
}

#[repr(C)]
pub struct parser300b_Grammar {
    pub productions: *const parser300b_Production,
    pub production_count: usize,
}

impl parser300b_Grammar {
    pub unsafe fn to_non_c(&self) -> Result<Grammar, Utf8Error> {
        slice::from_raw_parts(self.productions, self.production_count)
            .iter()
            .map(|t| t.to_non_c())
            .collect::<Result<Vec<_>, _>>()
            .map(|p| Grammar { productions: p })
    }
}

#[repr(C)]
pub struct parser300b_Token {
    pub name: *const c_char,
    pub data: *const c_void
}

impl parser300b_Token {
    unsafe fn to_non_c(&self) -> Result<CToken, Utf8Error> {
        Ok(CToken {
            name: CStr::from_ptr(self.name).to_str()?,
            data: self.data
        })
    }
}

#[derive(Debug, Clone)]
struct CToken<'n> {
    pub name: &'n str,
    #[allow(dead_code)]
    pub data: *const c_void
}

impl<'n> Display for CToken<'n> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl<'n> Token for CToken<'n> {
    fn name(&self) -> &str {
        self.name
    }
}

#[no_mangle]
pub unsafe extern "C" fn parser300b_parse(grammar: *const parser300b_Grammar, tokens: *const parser300b_Token, token_count: usize) {
    if !grammar.is_null() {
        let grammar: &parser300b_Grammar = &*grammar;
        let grammar = grammar.to_non_c().unwrap();
        let tokens = slice::from_raw_parts(tokens, token_count)
            .iter()
            .map(|t| t.to_non_c())
            .collect::<Result<Vec<_>, _>>();

        println!("parser300b_parse: {:#?} <- {:#?}", tokens, grammar);
    } else {
        panic!("grammar is null")
    }
}
