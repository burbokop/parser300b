
use std::fmt::Display;


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum ParseTreeNode<'t, 'g, T> 
where
    T: Sized
{
    Terminal(&'t T),
    Nonterminal(ParseTree<'t, 'g, T>),
    None
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct ParseTree<'t, 'g, T>
where
    T: Sized
{
    pub lhs: &'g String,
    pub rhs: Vec<ParseTreeNode<'t, 'g, T>>,
}

fn format_node<T: Display>(tree: &ParseTreeNode<T>, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    let tab = String::from_utf8(vec![b'`'; level]).unwrap();
    match tree {
        ParseTreeNode::Terminal(terminal) => {
            f.write_fmt(format_args_nl!("{}{}", tab, terminal))?;
        },
        ParseTreeNode::Nonterminal(nonterminal) => {
            format_tree(nonterminal, f, level)?;
        },
        ParseTreeNode::None => {}
    }
    Ok(())
}

fn format_tree<T: Display>(tree: &ParseTree<T>, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    let tab = String::from_utf8(vec![b'`'; level]).unwrap();
    f.write_fmt(format_args_nl!("{}{}", tab, tree.lhs))?;
    for r in &tree.rhs {
        format_node(r, f, level + 1)?;
    }
    Ok(())
}

impl<'t, 'g, T: Display> Display for ParseTree<'t, 'g, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            format_tree(self, f, 0)
        } else {
            panic!("can not display ParseTree without #")
        }
    }
}