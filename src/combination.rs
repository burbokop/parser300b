use std::{collections::{VecDeque}};

#[derive(Clone, Debug, PartialEq)]
pub struct Combination {
    pub marks: Vec<usize>
}

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

pub fn generate_combinations(begin: usize, end: usize, count: usize) -> Vec<Combination> {

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