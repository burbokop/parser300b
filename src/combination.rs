use std::{collections::{VecDeque}, fmt::Display};

#[derive(Clone, Debug, PartialEq)]
pub struct Combination {
    pub marks: Vec<usize>
}

impl Display for Combination {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for m in self.marks.iter() {
            list.entry(m);
        }
        list.finish()
    }
}

struct Comb<T> {
    values: Vec<T>
}

impl<T: Clone> Comb<T> {
    #[inline]
    fn clone_with(&self, v: T) -> Comb<T> {
        let mut new = Vec::with_capacity(self.values.len() + 1);
        new.clone_from(&self.values);
        new.push(v);
        Comb { values: new }
    }
}

/// 1|2, 3|4, 5
/// 
/// initial:
/// (1, ...)
/// (1, ...), (2, ...)
/// continuous:
/// (2, ...),    (1, 3, ...), (1, 4, ...)
/// (1, 3, ...), (1, 4, ...), (2, 3, ...), (2, 4, ...)
/// (1, 4, ...), (2, 3, ...), (2, 4, ...), (1, 3, 5)
/// (2, 3, ...), (2, 4, ...), (1, 3, 5),   (1, 4, 5)
/// (2, 4, ...), (1, 3, 5),   (1, 4, 5),   (2, 3, 5)
/// (1, 3, 5),   (1, 4, 5),   (2, 3, 5),   (2, 4, 5)
/// result:
/// (1, 3, 5),   (1, 4, 5),   (2, 3, 5),   (2, 4, 5)
#[inline]
pub fn expand_combinations<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> 
where
    T: Clone,
{
    if input.len() > 0 {
        let mut deq: VecDeque<Comb<T>> = input[0]
            .clone()
            .into_iter()
            .map(|v| Comb { values: vec![ v ] })
            .collect();

        while let Some(front) = deq.pop_front() {
            if front.values.len() < input.len() {
                for to_append in input[front.values.len()].clone() {
                    deq.push_back(front.clone_with(to_append))
                }            
            } else {
                deq.push_front(front);
                break;
            }
        }
        deq.into_iter().map(|c| c.values).collect()
    } else {
        Vec::new()
    }
}

#[inline]
pub fn expand_combinations_iter<T: Clone>(input: impl Iterator<Item = impl Iterator<Item = T>>) -> impl Iterator<Item = impl Iterator<Item = T>> {
    if true {
        let r = expand_combinations(input.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>())
            .into_iter()
            .map(|x|x.into_iter());
        r
    } else {
        todo!()
        //ExpandCombinationsOuterIter { input: input }
    }
}

#[inline]
pub fn expand_combinations_iter_dbg<T, E>(input: impl Iterator<Item = impl Iterator<Item = Result<T, E>>>) -> impl Iterator<Item = impl Iterator<Item = Result<T, E>>>
where 
    Result<T, E>: Clone,
    T: Display,
    E: Display
{
    if true {
        let input_vec = input.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>();

        println!("start:");
        for i in input_vec.clone() {
            println!("  outer:");
            for j in i {
                match j {
                    Ok(tree) => println!("    inner.tree:\n{:#}", tree),
                    Err(err) => println!("    inner.err: {}", err)
                }                
            }
            println!("  outer.end");
        }
        println!("end");

        let r = expand_combinations(input_vec)
            .into_iter()
            .map(|x|x.into_iter());
        r
    } else {
        todo!()
        //ExpandCombinationsOuterIter { input: input }
    }
}


#[inline]
pub fn generate_combinations(begin: usize, end: usize, count: usize) -> Vec<Combination> {
    if end - begin < count {
        vec![]
    } else if count == 0 {
        vec![ Combination { marks: vec![] } ]
    } else {
        let mut marks: Vec<usize> = (0..count)
            .map(|i| begin + i)
            .collect();

        let mut result: Vec<Combination> = Vec::new();
        loop {
            if marks[marks.len() - 1] < end {
                result.push(Combination { marks: marks.clone() });
            }
            
            if (0..count).rev().all(|i| {
                if marks[i] < end {
                    marks[i] += 1;
                    ((i + 1)..count).enumerate().for_each(|(offset, j)| {
                        marks[j] = marks[i] + offset + 1;
                    });
                    false
                } else {
                    true
                }
            }) { break; }
        }
        result            
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_combinations_test() {
        assert_eq!(
            generate_combinations(2, 6, 3),
            vec![
                Combination { marks: vec![2, 3, 4] }, 
                Combination { marks: vec![2, 3, 5] }, 
                Combination { marks: vec![2, 4, 5] }, 
                Combination { marks: vec![3, 4, 5] }
            ]
        )
    }

    #[test]
    fn generate_combinations_invalid_test() {
        assert_eq!(
            generate_combinations(2, 4, 3),
            vec![]
        )
    }

    #[test]
    fn expand_combinations_test() {
        // 1|2, 3|4, 5
        // (1, 3, 5),   (1, 4, 5),   (2, 3, 5),   (2, 4, 5
        assert_eq!(
            expand_combinations(vec![
                vec![ 1, 2 ],
                vec![ 3, 4 ],
                vec![ 5 ],
            ]),
            vec![
                vec![ 1, 3, 5 ],
                vec![ 1, 4, 5 ],
                vec![ 2, 3, 5 ],
                vec![ 2, 4, 5 ],
            ]
        );
    }

    #[test]
    fn expand_combinations_iter_test() {
        // 1|2, 3|4, 5

        // (1, 3, 5),   (1, 4, 5),   (2, 3, 5),   (2, 4, 5)

        assert_eq!(
            expand_combinations_iter(vec![
                vec![ 1, 2 ].into_iter(),
                vec![ 3, 4 ].into_iter(),
                vec![ 5 ].into_iter(),
            ].into_iter())
                .map(|v|v.collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![
                vec![ 1, 3, 5 ],
                vec![ 1, 4, 5 ],
                vec![ 2, 3, 5 ],
                vec![ 2, 4, 5 ],
            ]
        );
    }
}