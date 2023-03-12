#pragma once

#include <cstddef>
#include <cstdlib>

#ifdef __cplusplus
extern "C" {
#endif

struct parser300b_Term {
    const char* value;
    bool is_terminal;
};

void parser300b_Term_free(const parser300b_Term* terms, size_t count) {
    for(size_t i = 0; i < count; ++i) {
        free((char*)terms[i].value);
    }
    free((parser300b_Term*)terms);
}

struct parser300b_Expression {
    const parser300b_Term* terms;
    size_t term_count;
};

void parser300b_Expression_free(const parser300b_Expression* expressions, size_t count) {
    for(size_t i = 0; i < count; ++i) {
        parser300b_Term_free(expressions[i].terms, expressions[i].term_count);
    }
    free((parser300b_Expression*)expressions);
}

struct parser300b_Production {
    const char* lhs;
    const parser300b_Expression* rhs;
    size_t rhs_count;
};

void parser300b_Production_free(const parser300b_Production* productions, size_t count) {
    for(size_t i = 0; i < count; ++i) {
        free((char*)productions[i].lhs);
        parser300b_Expression_free(productions[i].rhs, productions[i].rhs_count);
    }
    free((parser300b_Production*)productions);
}

struct parser300b_Grammar {
    const parser300b_Production* productions;
    size_t production_count;
};

void parser300b_Grammar_free(const parser300b_Grammar* grammar) {
    parser300b_Production_free(grammar->productions, grammar->production_count);
    free((parser300b_Grammar*)grammar);
}

void parser300b_parse(const parser300b_Grammar* grammar);

#ifdef __cplusplus
}

#include <string.h>
#include <string>
#include <vector>

namespace parser300b {

struct Term {
    std::string value;
    bool is_terminal;

    static parser300b_Term* c_ref(const std::vector<Term>& terms) {
        parser300b_Term* result = (parser300b_Term*)malloc(terms.size() * sizeof(parser300b_Term));
        for(size_t i = 0; i < terms.size(); ++i) {
            result[i].value = strdup(terms[i].value.c_str());
            result[i].is_terminal = terms[i].is_terminal;
        }
        return result;
    }
};

struct Expression {
    std::vector<Term> terms;

    static parser300b_Expression* c_ref(const std::vector<Expression>& expressions) {
        parser300b_Expression* result = (parser300b_Expression*)malloc(expressions.size() * sizeof(parser300b_Expression));
        for(size_t i = 0; i < expressions.size(); ++i) {
            result[i].terms = Term::c_ref(expressions[i].terms);
            result[i].term_count = expressions[i].terms.size();
        }
        return result;
    }
};

struct Production {
    std::string lhs;
    std::vector<Expression> rhs;

    static parser300b_Production* c_ref(const std::vector<Production>& productions) {
        parser300b_Production* result = (parser300b_Production*)malloc(productions.size() * sizeof(parser300b_Production));
        for(size_t i = 0; i < productions.size(); ++i) {
            result[i].lhs = strdup(productions[i].lhs.c_str());
            result[i].rhs = Expression::c_ref(productions[i].rhs);
            result[i].rhs_count = productions[i].rhs.size();
        }
        return result;
    }
};

struct Grammar {
    std::vector<Production> productions;

    const parser300b_Grammar* c_ref() const {
        parser300b_Grammar* result = (parser300b_Grammar*)malloc(sizeof(parser300b_Grammar));
        result->productions = Production::c_ref(productions);
        result->production_count = productions.size();
        return result;
    }
};

inline void parse(const Grammar& grammar) {
    auto c_grammar = grammar.c_ref();
    parser300b_parse(c_grammar);
    parser300b_Grammar_free(c_grammar);
}

}

#endif
