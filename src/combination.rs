use std::{collections::{VecDeque, btree_map::Iter}, marker::PhantomData};

#[derive(Clone, Debug, PartialEq)]
pub struct Combination {
    pub marks: Vec<usize>
}

pub enum CombinationOrCount {
    Combination(Combination),
    Count(usize)
}


pub fn generate_combination(begin: usize, end: usize, prev_combination: CombinationOrCount) -> Option<Combination> {
    match prev_combination {
        CombinationOrCount::Combination(c) => {
            if end - begin < c.marks.len() {
                None
            } else {
                let mut result = Combination { marks: Vec::new() };
                result.marks.resize(c.marks.len(), begin);

                let mut shift_happened = false;
                for i in (0..c.marks.len()).rev() {
                    if shift_happened {
                        result.marks[i] = c.marks[i];
                    } else {
                        let right_border: usize = if i + 1 < c.marks.len() {
                            c.marks[i + 1]
                        } else {
                            end
                        };

                        if c.marks[i] + 1 < right_border {
                            result.marks[i] = c.marks[i] + 1;
                            shift_happened = true;
                        } else {
                            result.marks[i] = c.marks[i];
                        }
                    }
                }
                if shift_happened {
                    Some(result)
                } else {
                    None
                }
            }
        },
        CombinationOrCount::Count(c) => {
            if end - begin < c {
                None
            } else {
                let mut result = Combination { marks: Vec::with_capacity(c) };
                for i in 0..c {
                    result.marks.push(begin + i);
                }
                Some(result)
            }
        },
    }
}

pub fn generate_combinations(begin: usize, end: usize, count: usize) -> Vec<Combination> {
    let mut result: Vec<Combination> = Vec::new();
    let mut prev: CombinationOrCount = CombinationOrCount::Count(count);

    while let Some(current) = generate_combination(begin, end, prev) {
        result.push(current.clone());
        prev = CombinationOrCount::Combination(current);
    }
    result
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
pub fn expand_combinations<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> 
where
    T: Clone,
{
    struct Comb<T> {
        values: Vec<T>
    }

    impl<T: Clone> Comb<T> {
        fn clone_with(&self, v: T) -> Comb<T> {
            let mut c = Comb { values: self.values.clone() };
            c.values.push(v);
            c
        }
    }

    let mut deq: VecDeque<Comb<T>> = VecDeque::new();

    if input.len() > 0 {
        for v in input[0].clone() {
            deq.push_back(Comb { values: vec![ v ] });
        }

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



/// next: 1|2


struct CombinationIter<T, II, OI> 
where
    II: Iterator<Item = T>,
    OI: Iterator<Item = II>
{
    input: OI
}

impl<T, II, OI> Iterator for CombinationIter<T, II, OI> 
where
    II: Iterator<Item = T>,
    OI: Iterator<Item = II>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct ExpandCombinationsOuterIter<T, II, OI> 
where
    II: Iterator<Item = T>,
    OI: Iterator<Item = II>
{
    input: OI
}

impl<T, II, OI> Iterator for ExpandCombinationsOuterIter<T, II, OI>
where
    II: Iterator<Item = T>,
    OI: Iterator<Item = II>,
{
    type Item = CombinationIter<T, II, OI>;

    fn next(&mut self) -> Option<Self::Item> {
        //Some(CombinationIter { input: self.input })
        None
    }
}


pub fn expand_combinations_iter<T: Clone>(input: impl Iterator<Item = impl Iterator<Item = T>>) -> impl Iterator<Item = impl Iterator<Item = T>> {
    let r = expand_combinations(input.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>())
        .into_iter()
        .map(|x|x.into_iter());
    r
    //ExpandCombinationsOuterIter { input: input }
}


pub fn __gen(begin: usize, end: usize, count: usize) -> Vec<Combination> {

    if end - begin < count {
        vec![]
    } else if count == 0 {
        vec![ Combination { marks: vec![] } ]
    } else {                
        let mut marks: Vec<usize> = Vec::new();
        for i in 0..count {
            marks.push(begin + i);
        }

        let mut result: Vec<Combination> = Vec::new();
        
        loop {
            if marks[marks.len() - 1] < end {
                //println!("m: {:?}", marks.clone());
                result.push(Combination { marks: marks.clone() });
            }
            
            let mut moved: bool = false;
            for i in (0..count).rev() {
                if marks[i] < end {
                    marks[i] += 1;

                    for (offset, j) in ((i + 1)..count).enumerate() {
                        marks[j] = marks[i] + offset + 1;
                    }

                    moved = true;
                    break;
                } else {
                    //marks[i] += 1;
                }
            }
            if !moved {
                break;
            }
        }
        result            
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_combination_some_test() {
        assert_eq!(
            generate_combination(2, 10, CombinationOrCount::Count(3)), 
            Some(Combination { marks: vec![2, 3, 4] })
        );
    }

    #[test]
    fn generate_combination_none_test() {
        assert_eq!(
            generate_combination(2, 4, CombinationOrCount::Count(3)),
            None
        );
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

        // (1, 3, 5),   (1, 4, 5),   (2, 3, 5),   (2, 4, 5)

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

}