

#[macro_export]
#[allow_internal_unstable(core_panic)]
macro_rules! assert_contains_tree {
    ($grammar:expr, $tokens:expr, $expected_tree:expr) => {

        pub fn add_margins(s: &str, level: usize) -> String {
            let tab = "\t".repeat(level);
            s
                .lines()
                .map(|s| tab.clone() + "|" + s)
                .collect::<Vec<_>>()
                .join("\n")
        }
        

        let grammar_str: String = $grammar.trim_margin().unwrap_or($grammar.to_string());
        let tokens_slice: &[&str] = &$tokens;
        let expected_tree_str: &str = $expected_tree;

        let mirror_result = std::process::Command::new("java")
            .arg("-jar")
            .arg("tests/mirror_parser/target/mirror_parser-TEST_ONLY-standalone.jar")
            .arg("--grammar")
            .arg(&grammar_str)
            .arg("--text")
            .arg(tokens_slice.join(""))
            .arg("--format")
            .arg("yaml")
            .output()
            .unwrap();

        if mirror_result.status.success() {
            let current_log_index = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            std::fs::create_dir_all("logs/mirror").unwrap();
            std::fs::write(format!("logs/mirror/{}.txt", current_log_index), String::from_utf8(mirror_result.stdout).unwrap()).unwrap();    
        } else {
            panic!("mirror parser failed with code {:?} and error '{}'", mirror_result.status.code(), String::from_utf8(mirror_result.stderr).unwrap());
        }

        let g: Grammar = grammar_str
            .as_str()
            .try_into()
            .unwrap();

        let g = g.flatten();

        let t: Vec<_> = tokens_slice
            .into_iter()
            .map(|x| String::from(*x))
            .collect();

        let ctx = make_ctx(&g, &t, false, true);

        let mut trees: Vec<_> = parse(ctx)
            .into_iter()
            .map(|t|t.map(|t|format!("{:#}", t)))
            .collect();
        
        trees.sort_by(|x, y| x.is_ok().cmp(&y.is_ok()) );

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
                    Ok(tree) => format!("\ttree:\n{:#}\n", add_margins(tree, 2)),
                    Err(err) => format!("\tfail: '{}'\n", err)
                }
            })
                .collect::<Vec<_>>()
                .join("");

            let separator = "<|".to_string() + &String::from_utf8(vec![b'-'; 32]).unwrap() + "|>";

            match expected {
                Ok(tree) => panic!("assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\ttree:\n{:#}\n{}\n", 
                    separator, left_variants, add_margins(&tree, 2), separator),
                Err(err) => panic!("assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\tfail: '{:#}'\n{}\n", 
                    separator, left_variants, err, separator)
            }
        }
    };
}
