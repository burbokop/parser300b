
use std::{fmt::Display, iter::once};

use serde::{Serialize, ser::SerializeSeq};

use crate::ctx::Token;


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum ParseTreeNode<'t, 'g, T> 
where
    T: Sized + Token
{
    Terminal(&'t T),
    Nonterminal(ParseTree<'t, 'g, T>),
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct ParseTree<'t, 'g, T>
where
    T: Sized + Token
{
    pub lhs: &'g String,
    pub rhs: Vec<ParseTreeNode<'t, 'g, T>>,
}

impl<'t, 'g, T: Token + Serialize> Serialize for ParseTreeNode<'t, 'g, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        match self {
            ParseTreeNode::Terminal(term) => serializer.serialize_str(term.name()),
            ParseTreeNode::Nonterminal(nonterm) => nonterm.serialize(serializer),
        }
    }
}

impl<'t, 'g, T: Token + Serialize> Serialize for ParseTree<'t, 'g, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut seq = serializer.serialize_seq(Some(self.rhs.len() + 1))?;
        seq.serialize_element(self.lhs)?;
        for node in &self.rhs {
            seq.serialize_element(node)?;
        }
        seq.end()
    }
}

fn format_node<T: Token>(tree: &ParseTreeNode<T>, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    let tab = String::from_utf8(vec![b'`'; level]).unwrap();
    match tree {
        ParseTreeNode::Terminal(terminal) => {
            f.write_fmt(format_args_nl!("{}{}", tab, terminal))?;
        },
        ParseTreeNode::Nonterminal(nonterminal) => {
            format_tree(nonterminal, f, level)?;
        },
    }
    Ok(())
}

fn format_tree<T: Token>(tree: &ParseTree<T>, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    let tab = String::from_utf8(vec![b'`'; level]).unwrap();
    f.write_fmt(format_args_nl!("{}{}", tab, tree.lhs))?;
    for r in &tree.rhs {
        format_node(r, f, level + 1)?;
    }
    Ok(())
}


impl<'t, 'g, T: Token> Display for ParseTreeNode<'t, 'g, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            format_node(self, f, 0)
        } else {
            panic!("can not display ParseTreeNode without #")
        }
    }
}

impl<'t, 'g, T: Token> Display for ParseTree<'t, 'g, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            format_tree(self, f, 0)
        } else {
            panic!("can not display ParseTree without #")
        }
    }
}