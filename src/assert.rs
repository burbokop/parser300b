
#[warn(dead_code)]
pub fn add_margins(s: &str, level: usize) -> String {
    let tab = "\t".repeat(level);
    s
        .lines()
        .map(|s| tab.clone() + "|" + s)
        .collect::<Vec<_>>()
        .join("\n")
}

#[macro_export]
#[allow_internal_unstable(core_panic)]
macro_rules! assert_contains_tree {
    ($grammar:expr, $tokens:expr, $expected_tree:expr) => {

        let grammar_str: &str = $grammar;
        let tokens_slice: &[&str] = &$tokens;
        let expected_tree_str: &str = $expected_tree;

        let g: Grammar = grammar_str
            .try_into()
            .unwrap();

        let t: Vec<_> = tokens_slice
            .into_iter()
            .map(|x| String::from(*x))
            .collect();

        let ctx = crate::parse::make_ctx(&g, &t);

        let trees: Vec<_> = parse(ctx)
            .into_iter()
            .map(|t|t.map(|t|format!("{:#}", t)))
            .collect();
        
        let expected: Result<String, Error> = Ok(String::from(expected_tree_str.trim_margin().unwrap()) + "\n");

        let mut found = false;
        for tree in &trees {
            if *tree == expected {
                found = true;
                break;
            }
        }

        if !found {
            let left_variants = trees.iter().map(|l| {
                match l {
                    Ok(tree) => format!("\ttree:\n{:#}\n", crate::assert::add_margins(tree, 2)),
                    Err(err) => format!("\tfail: '{}'\n", err)
                }
            })
                .collect::<Vec<_>>()
                .join("");

            let separator = "<|".to_string() + &String::from_utf8(vec![b'-'; 32]).unwrap() + "|>";

            match expected {
                Ok(tree) => panic!("assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\ttree:\n{:#}\n{}\n", 
                    separator, left_variants, crate::assert::add_margins(&tree, 2), separator),
                Err(err) => panic!("assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\tfail: '{:#}'\n{}\n", 
                    separator, left_variants, err, separator)
            }
        }
    };
}
