use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Range;

use crate::{combination::*, grammar::Grammar};


pub trait Token: Debug + Display + Clone {
    fn name(&self) -> &str;
}

impl Token for String {
    fn name(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Copy)]
pub struct Ctx<'t, 'g, T> {
    pub begin: usize,
    pub end: usize,
    pub tokens: &'t Vec<T>,
    pub grammar: &'g Grammar,
    pub level: usize
}

impl<'t, 'g, T> Ctx<'t, 'g, T> {
    pub fn next_level(&self) -> Ctx<'t, 'g, T> {
        Ctx {
            begin: self.begin, 
            end: self.end, 
            tokens: self.tokens, 
            grammar: self.grammar, 
            level: self.level + 1 
        }
    }

    pub fn at(&self, range: Range<usize>) -> Ctx<'t, 'g, T> {
        Ctx { 
            begin: range.start, 
            end: range.end, 
            tokens: self.tokens, 
            grammar: self.grammar, 
            level: self.level 
        }
    }

    pub fn split(&self, combination: Combination) -> Vec<Ctx<'t, 'g, T>> {
        let mut result: Vec<Ctx<T>> = Vec::with_capacity(combination.marks.len());

        if combination.marks.len() > 0 {
            if combination.marks[0] > self.begin {
                result.push(Ctx {
                    begin: self.begin,
                    end: combination.marks[0],
                    tokens: self.tokens,
                    grammar: self.grammar,
                    level: self.level
                })
            }
        } else {
            result.push(Ctx {
                begin: self.begin,
                end: self.end,
                tokens: self.tokens,
                grammar: self.grammar,
                level: self.level
            })
        }

        for i in 0..combination.marks.len() {

            if i + 1 < combination.marks.len() {
                result.push(Ctx {
                    begin: combination.marks[i],
                    end: combination.marks[i + 1],
                    tokens: self.tokens,
                    grammar: self.grammar,
                    level: self.level
                })                
            } else if combination.marks[i] < self.end {
                result.push(Ctx {
                    begin: combination.marks[i],
                    end: self.end,
                    tokens: self.tokens,
                    grammar: self.grammar,
                    level: self.level
                })                
            }
        }
        result
    }

    pub fn combinations(&self, subctx_count: usize) -> Vec<Combination> {
        //generate_combinations(self.begin + 1, self.end, subctx_count - 1)
        __gen(self.begin + 1, self.end, subctx_count - 1)
    }

    pub fn len(&self) -> usize {
        return self.end - self.begin
    }

    pub fn front(&self) -> &'t T {
        &self.tokens[self.begin]
    }
}

impl<'t, 'g, T> PartialEq for Ctx<'t, 'g, T> {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin && 
        self.end == other.end && 
        self.tokens as *const Vec<_> == other.tokens as *const Vec<_> && 
        self.grammar as *const Grammar == other.grammar as *const Grammar
    }
}

impl<'t, 'g, T: Display> Display for Ctx<'t, 'g, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!(
                "<{}, {}, '{}'> ", 
                self.begin, 
                self.end, 
                self.tokens[self.begin..self.end]
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<_>>()
                    .join("")
            ))
        } else {
            f.write_fmt(format_args!("<{}, {}>", self.begin, self.end))
        }
    }
}

impl<'t, 'g, T> Debug for Ctx<'t, 'g, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{}, {}>", self.begin, self.end))
    }
}

pub struct VecDisplay<T> {
    pub v: Vec<T>
}

impl<T> Display for VecDisplay<T>
where 
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.v.len() == 0 {
            f.write_str("[]")
        } else {
            let mut i: usize = 0;
            f.write_str("[ ")?;
            for v in &self.v {

                v.fmt(f)?;

                //f.write_fmt(format_args!("{}", v))?;
                if i + 1 < self.v.len() {
                    f.write_str(", ")?;
                }
                i += 1;
            }
            f.write_str(" ]")?;
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::grammar::Grammar;
    use super::Ctx;

    #[test]
    fn split_ctx_test() {
        let tokens: Vec<String> = Vec::new();
        let grammar = Grammar { productions: vec![] };
        let ctx = Ctx { begin: 4, end: 9, tokens: &tokens, grammar: &grammar, level: 0 };

        let combinations: Vec<_> = ctx
            .combinations(3)
            .into_iter()
            .map(|combination| ctx.split(combination))
            .collect();

        assert_eq!(
            combinations,
            vec![
                vec![ ctx.at(4..5), ctx.at(5..6), ctx.at(6..9) ],
                vec![ ctx.at(4..5), ctx.at(5..7), ctx.at(7..9) ],
                vec![ ctx.at(4..5), ctx.at(5..8), ctx.at(8..9) ],
                vec![ ctx.at(4..6), ctx.at(6..7), ctx.at(7..9) ],
                vec![ ctx.at(4..6), ctx.at(6..8), ctx.at(8..9) ],
                vec![ ctx.at(4..7), ctx.at(7..8), ctx.at(8..9) ],
            ]
        );
    }

    ///
    /// Expected result: 
    /// 1|2|3|4 5 6 7    0..1|1..2|2..3|3..7
    /// 1|2|3 4|5 6 7    0..1|1..2|2..4|4..7
    /// 1|2|3 4 5|6 7    0..1|1..2|2..5|5..7
    /// 1|2|3 4 5 6|7    0..1|1..2|2..6|6..7
    /// 1|2 3|4|5 6 7    0..1|1..3|3..4|4..7
    /// 1|2 3|4 5|6 7    0..1|1..3|3..5|5..7
    /// 1|2 3|4 5 6|7    0..1|1..3|3..6|6..7
    /// 1|2 3 4|5|6 7    0..1|1..4|4..5|5..7
    /// 1|2 3 4|5 6|7    0..1|1..4|4..6|6..7
    /// 1|2 3 4 5|6|7    0..1|1..5|5..6|6..7
    ///
    /// 1 2|3|4|5 6 7    0..2|2..3|3..4|4..7
    /// 1 2|3|4 5|6 7    0..2|2..3|3..5|5..7
    /// 1 2|3|4 5 6|7    0..2|2..3|3..6|6..7
    /// 1 2|3 4|5|6 7    0..2|2..4|4..5|5..7
    /// 1 2|3 4|5 6|7    0..2|2..4|4..6|6..7
    /// 1 2|3 4 5|6|7    0..2|2..5|5..6|6..7
    ///
    /// 1 2 3|4|5|6 7    0..3|3..4|4..5|5..7
    /// 1 2 3|4|5 6|7    0..3|3..4|4..6|6..7
    /// 1 2 3|4 5|6|7    0..3|3..5|5..6|6..7
    ///
    /// 1 2 3 4|5|6|7    0..4|4..5|5..6|6..7
    ///
    /// total: 20
    /// 
    #[test]
    fn split_ctx_test2() {
        let tokens: Vec<String> = Vec::new();
        let grammar = Grammar { productions: vec![] };
        let ctx = Ctx { begin: 0, end: 7, tokens: &tokens, grammar: &grammar, level: 0 };

        let combinations: Vec<_> = ctx
            .combinations(4)
            .into_iter()
            .map(|combination| ctx.split(combination))
            .collect();

        assert_eq!(
            combinations,
            vec![
                vec![ ctx.at(0..1), ctx.at(1..2), ctx.at(2..3), ctx.at(3..7) ],
                vec![ ctx.at(0..1), ctx.at(1..2), ctx.at(2..4), ctx.at(4..7) ],
                vec![ ctx.at(0..1), ctx.at(1..2), ctx.at(2..5), ctx.at(5..7) ],
                vec![ ctx.at(0..1), ctx.at(1..2), ctx.at(2..6), ctx.at(6..7) ],
                vec![ ctx.at(0..1), ctx.at(1..3), ctx.at(3..4), ctx.at(4..7) ],
                vec![ ctx.at(0..1), ctx.at(1..3), ctx.at(3..5), ctx.at(5..7) ],
                vec![ ctx.at(0..1), ctx.at(1..3), ctx.at(3..6), ctx.at(6..7) ],
                vec![ ctx.at(0..1), ctx.at(1..4), ctx.at(4..5), ctx.at(5..7) ],
                vec![ ctx.at(0..1), ctx.at(1..4), ctx.at(4..6), ctx.at(6..7) ],
                vec![ ctx.at(0..1), ctx.at(1..5), ctx.at(5..6), ctx.at(6..7) ],
                vec![ ctx.at(0..2), ctx.at(2..3), ctx.at(3..4), ctx.at(4..7) ],
                vec![ ctx.at(0..2), ctx.at(2..3), ctx.at(3..5), ctx.at(5..7) ],
                vec![ ctx.at(0..2), ctx.at(2..3), ctx.at(3..6), ctx.at(6..7) ],
                vec![ ctx.at(0..2), ctx.at(2..4), ctx.at(4..5), ctx.at(5..7) ],
                vec![ ctx.at(0..2), ctx.at(2..4), ctx.at(4..6), ctx.at(6..7) ],
                vec![ ctx.at(0..2), ctx.at(2..5), ctx.at(5..6), ctx.at(6..7) ],
                vec![ ctx.at(0..3), ctx.at(3..4), ctx.at(4..5), ctx.at(5..7) ],
                vec![ ctx.at(0..3), ctx.at(3..4), ctx.at(4..6), ctx.at(6..7) ],
                vec![ ctx.at(0..3), ctx.at(3..5), ctx.at(5..6), ctx.at(6..7) ],
                vec![ ctx.at(0..4), ctx.at(4..5), ctx.at(5..6), ctx.at(6..7) ],
            ]
        );
    }

    #[test]
    fn split_ctx_into_same_test() {
        let tokens: Vec<String> = Vec::new();
        let grammar = Grammar { productions: vec![] };
        let ctx = Ctx { begin: 0, end: 7, tokens: &tokens, grammar: &grammar, level: 0 };

        let combinations: Vec<_> = ctx
            .combinations(1)
            .into_iter()
            .map(|combination| ctx.split(combination))
            .collect();

        assert_eq!(combinations, vec![ vec![ ctx ] ]);
    }


}