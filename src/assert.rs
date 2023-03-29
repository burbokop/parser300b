

static INIT: std::sync::Once = std::sync::Once::new();

pub fn init_assert_contains_tree() {
    INIT.call_once(|| {
        if let Err(err) = std::fs::remove_dir_all("logs/tests") {
            eprintln!("warning deleting logs dir: {}", err);
        }
    });
}

#[macro_export]
#[allow_internal_unstable(core_panic)]
macro_rules! assert_contains_tree {
    ($grammar:expr, $tokens:expr) => {
        assert_contains_tree!($grammar, $tokens, "any")
    };
    ($grammar:expr, $tokens:expr, $expected_tree:expr) => {

        init_assert_contains_tree();

        pub fn add_margins(s: &str, level: usize) -> String {
            let tab = "\t".repeat(level);
            s
                .lines()
                .map(|s| tab.clone() + "|" + s)
                .collect::<Vec<_>>()
                .join("\n")
        }
        
        pub fn mirror_parse(grammar: &str, text: &str, format: &str) -> std::process::Output {
            std::process::Command::new("java")
                .arg("-jar")
                .arg("tests/mirror_parser/target/mirror_parser-TEST_ONLY-standalone.jar")
                .arg("--grammar")
                .arg(grammar)
                .arg("--text")
                .arg(text)
                .arg("--format")
                .arg(format)
                .output()
                .expect("mirror_parser-TEST_ONLY-standalone.jar executed")
        }

        let current_log_index = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("duration since epoch got")
            .as_nanos() % 2_u128.pow(16);

        let log_dir = format!("logs/tests/{}", current_log_index);
        let log_dir = log_dir.as_str();
        std::fs::create_dir_all(log_dir).expect("logs dir created");

        let log_mirror_result = |output: std::process::Output, suffix: &str| {
            if output.status.success() {
                let res = String::from_utf8(output.stdout).expect("mirror output being utf8");
                std::fs::write(format!("{}/mirror.{}", log_dir, suffix), res.as_str())
                    .expect("mirror log writed");    
                res
            } else {
                panic!(
                    "mirror parser failed with code {:?} and error '{}'", 
                    output.status.code(), 
                    String::from_utf8(output.stderr)
                        .expect("mirror error message being utf8")
                );
            }    
        };

        let grammar_str: String = $grammar.trim_margin().unwrap_or($grammar.to_string());
        let tokens_slice: &[&str] = &$tokens;
        let expected_tree_str: &str = $expected_tree;

        let expected_mirror_yaml: Result<String, Error> = Ok(log_mirror_result(mirror_parse(&grammar_str, &tokens_slice.join(" "), "yaml"), "yaml"));
        log_mirror_result(mirror_parse(&grammar_str, &tokens_slice.join(" "), "json"), "json");

        let g: ExtGrammar = grammar_str
            .as_str()
            .try_into()
            .expect("grammar parsed");

        let g = g.flatten();

        let t: Vec<_> = tokens_slice
            .into_iter()
            .map(|x| String::from(*x))
            .collect();

        let ctx = make_ctx(&g, &t, false, true);


        let start_parse_instant = std::time::Instant::now();
        let mut trees: Vec<_> = parse(ctx)
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                match t {
                    Ok(t) => {
                        let yaml = serde_yaml::to_string(&t).unwrap();
                        let display = format!("{:#}", &t);
                        std::fs::write(format!("{}/_{}.yaml", log_dir, i), &yaml).unwrap();
                        std::fs::write(format!("{}/_{}.json", log_dir, i), serde_json::to_string_pretty(&t).unwrap()).unwrap();
                        Ok((t, display, yaml))
                    },
                    Err(err) => Err(err)
                }
            })
            .collect();
        println!("parsing duration: {:?}", start_parse_instant.elapsed());
        
        trees.sort_by(|x, y| x.is_ok().cmp(&y.is_ok()) );
        let trees = trees;

        let expected: Result<String, Error> = Ok(String::from(expected_tree_str.trim_margin().unwrap()) + "\n");

        fn is_same_with_expected(actual: Result<String, String>, exp: &Result<String, String>) -> bool {
            if $expected_tree == "any" {
                actual.is_ok()
            } else {
                actual == *exp
            }
        }

        let mut found = false;
        for tree in &trees {            
            if is_same_with_expected(tree.clone().map(|t|t.1), &expected) {
                found = true;
                break;
            }
        }

        let mut mirror_found = false;
        for tree in &trees {
            if tree.clone().map(|t|t.2) == expected_mirror_yaml {
                mirror_found = true;
                break;
            }
        }

        if !found {
            let left_variants = trees.iter().map(|l| {
                match l {
                    Ok(tree) => format!("\ttree:\n{:#}\n", add_margins(&tree.1, 2)),
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

        if !mirror_found {
            let left_variants = trees.iter().map(|l| {
                match l {
                    Ok(tree) => format!("\ttree:\n{:#}\n", add_margins(&tree.2, 2)),
                    Err(err) => format!("\tfail: '{}'\n", err)
                }
            })
                .collect::<Vec<_>>()
                .join("");

            let separator = "<|".to_string() + &String::from_utf8(vec![b'-'; 32]).unwrap() + "|>";

            match expected_mirror_yaml {
                Ok(tree) => panic!("mirror assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\ttree:\n{:#}\n{}\n", 
                    separator, left_variants, add_margins(&tree, 2), separator),
                Err(err) => panic!("mirror assertion failed: `left.contains(right)`\n{}\nleft:\n{}right:\n\tfail: '{:#}'\n{}\n", 
                    separator, left_variants, err, separator)
            }
        }
    };
}
