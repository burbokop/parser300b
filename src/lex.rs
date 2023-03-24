use std::collections::BTreeMap;


struct LexToken {
    name: String,
    text: String,
    position: usize,
}

impl ctx::Token for Token {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

struct Pattern {
    reg: Regex,
    name: String
}

static ignore: &str = "ignore";

fn find_repetition(patterns: Vec<Pattern>) -> Option<String> {
    let mut m: BTreeMap<Pattern, char> = BTreeMap::new();
    for p in patterns {
        if p.name == ignore { continue; }

        if let Some(_) = m.insert(p, 0) {
            return Some(p.name);            
        }
    }
    return None;
}

void wall_e::lex::remove_character(std::string *text, char c) {
    if (text) {
        text->erase(std::remove(text->begin(), text->end(), c), text->end());
    }
}

std::string wall_e::lex::remove_character(std::string text, char c) {
    remove_character(&text, c);
    return text;
}

std::string wall_e::lex::replace(std::string str, const wall_e::vec<wall_e::pair<std::string, std::string> > &mapping) {
    const auto&& replace_all = [](std::string& data, const std::string& to_search, const std::string& replace_str) {
        size_t pos = data.find(to_search);
        while(pos != std::string::npos) {
            data.replace(pos, to_search.size(), replace_str);
            pos = data.find(to_search, pos + replace_str.size());
        }
    };

    for(const auto& m : mapping) {
        replace_all(str, m.first, m.second);
    }
    return str;
}

pub fn make_tokents(text: String, patterns: Vec<Pattern>, replacer: char) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    if let Some(rep) = find_repetition(pattern_list) {
        panic!("repetition found ({})", rep);
    }

    for pattern in patterns {
        std::sregex_token_iterator it(text.begin(), text.end(), pattern.reg);
        std::sregex_token_iterator end;
        while(it != end) {
            if(pattern.name.size() > 0 && pattern.name != special::ignore) {
                wall_e::lex::token token;
                token.uri = uri;
                token.name = pattern.name;
                token.text = *it;
                token.position = it->first - text.begin();
                result.push_back(token);
            }
            text.replace(it->first, it->second, std::string(it->length(), replacer));
            ++it;
        }
    }
    std::sort(result.begin(), result.end(), [](const token& a, const token& b){
        return a.position < b.position;
    });
    return result;
}
