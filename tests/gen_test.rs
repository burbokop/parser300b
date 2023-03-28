use parser300b::*;
use trim_margin::MarginTrimmable;

pub struct GenerateArgs<'l> {
    grammar: &'l Grammar,
    count: usize, 
    separator: &'l str,
    max_reductions: usize,
    max_nonprod_reductions: usize,
}

pub fn generate(args: GenerateArgs) -> Result<Vec<Vec<String>>, String> {

    println!("grammar: {}", args.grammar);

    let out = std::process::Command::new("node")
        .arg("tests/generator/target/main.js")
        .arg("-g")
        .arg(format!("{}", args.grammar))
        .arg(format!("--count={}", args.count))
        .arg(format!("--separator={}", args.separator))
        .arg(format!("--maxReductions={}", args.max_reductions))
        .arg(format!("--maxNonprodReductions={}", args.max_nonprod_reductions))
        .output()
        .unwrap();
    if out.status.success() {
        Ok(serde_json::from_slice(&out.stdout).unwrap())
    } else {   
        Err(String::from_utf8(out.stderr).unwrap())
    }
}

#[test]
fn gen_test() {
    let grammar_str = include_str!("superhard.bnf");
    let grammar: ExtGrammar = grammar_str
        .try_into()
        .unwrap();
    let grammar = grammar.flatten();
    let separator = " ";

    match generate(GenerateArgs { 
        grammar: &grammar, 
        count: 1, 
        separator: separator, 
        max_reductions: 10, 
        max_nonprod_reductions: 10
    }) {
        Ok(gens) => {
            for gen in gens {
                let gen: Vec<&str> = gen.iter().map(|g|g.as_str()).collect();
                println!("gen: {:?}", gen);
                assert_contains_tree!(grammar_str, &gen, "");
            }
            panic!("");
        },
        Err(err) => panic!("{}", err),
    }
}
