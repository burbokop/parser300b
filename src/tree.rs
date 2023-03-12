
use std::fmt::{Display};

use crate::ctx::Token;



#[derive(PartialEq, Debug, Clone, Eq)]
pub enum ParseTreeNode<'t, 'g> {
    Terminal(&'t Token),
    Nonterminal(ParseTree<'t, 'g>),
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct ParseTree<'t, 'g> {
    pub lhs: &'g String,
    pub rhs: Vec<ParseTreeNode<'t, 'g>>,
}

impl<'t, 'g> PartialEq<&str> for ParseTree<'t, 'g> {
    fn eq(&self, other: &&str) -> bool {
        format!("{}", self).as_str() == *other
    }
}

fn format_node(tree: &ParseTreeNode, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
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

fn format_tree(tree: &ParseTree, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    let tab = String::from_utf8(vec![b'`'; level]).unwrap();
    f.write_fmt(format_args_nl!("{}{}", tab, tree.lhs))?;
    for r in &tree.rhs {
        format_node(r, f, level + 1)?;
    }
    Ok(())
}

impl<'t, 'g> Display for ParseTree<'t, 'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            format_tree(self, f, 0)
        } else {
            panic!("can not display ParseTree without #")
        }
    }
}