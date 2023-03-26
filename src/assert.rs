

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
                .unwrap()
        }

        let current_log_index = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 2_u128.pow(16);

        let log_mirror_result = |output: std::process::Output, suffix: &str| {
            if output.status.success() {
                std::fs::create_dir_all("logs/tests").unwrap();
                std::fs::write(format!("logs/tests/{}_mirror.{}", current_log_index, suffix), String::from_utf8(output.stdout).unwrap()).unwrap();    
            } else {
                panic!("mirror parser failed with code {:?} and error '{}'", output.status.code(), String::from_utf8(output.stderr).unwrap());
            }    
        };

        let grammar_str: String = $grammar.trim_margin().unwrap_or($grammar.to_string());
        let tokens_slice: &[&str] = &$tokens;
        let expected_tree_str: &str = $expected_tree;

        log_mirror_result(mirror_parse(&grammar_str, &tokens_slice.join(""), "yaml"), "yaml");
        log_mirror_result(mirror_parse(&grammar_str, &tokens_slice.join(""), "json"), "json");

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
            .enumerate()
            .map(|(i, t)| {
                if let Ok(t) = t {
                    std::fs::create_dir_all("logs/tests").unwrap();
                    std::fs::write(format!("logs/tests/{}_{}.yaml", current_log_index, i), serde_yaml::to_string(&t).unwrap()).unwrap();
                    std::fs::write(format!("logs/tests/{}_{}.json", current_log_index, i), serde_json::to_string_pretty(&t).unwrap()).unwrap();
                    Ok(t)
                } else { t }
            })
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
