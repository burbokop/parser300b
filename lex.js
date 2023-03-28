#!/bin/node

const lexing = require('lexing');
const util = require('util')
const cmdargs = require('command-line-args')([
    { name: 'names', alias: 'n', type: Boolean },
    { name: 'values', alias: 'v', type: Boolean },
    { name: 'input', alias: 'i', type: String },
])

const rules = [
    [ /^$/, match => lexing.Token('EOF', null) ],
    [ /^\s+/, match => null ],
    [ /^_/, match => lexing.Token('ANON', match[0]) ],
    [ /^::=/, match => lexing.Token('BNF_EQ', match[0]) ],
    [ /^[+]/, match => lexing.Token('PLUS', match[0]) ],
    [ /^[*]/, match => lexing.Token('MUL', match[0]) ],
    [ /^[(]/, match => lexing.Token('OP', match[0]) ],
    [ /^[)]/, match => lexing.Token('EP', match[0]) ],
    [ /^[{]/, match => lexing.Token('OB', match[0]) ],
    [ /^[}]/, match => lexing.Token('EB', match[0]) ],
    [ /^[;]/, match => lexing.Token('SEMICOLON', match[0]) ],
    [ /^[.]/, match => lexing.Token('DOT', match[0]) ],
    [ /^[,]/, match => lexing.Token('COMA', match[0]) ],
    [ /^=/, match => lexing.Token('EQ', match[0]) ],
    [ /^"([^"\\]|\\.)*"/, match => lexing.Token('QTERM', match[0]) ],
    [ /^<\w+>/, match => lexing.Token('NONTERM', match[0]) ],
    [ /^\w+/, match => lexing.Token('TERM', match[0]) ],
    [ /^[|]/, match => lexing.Token('OR', match[0]) ],
];


const text = cmdargs.input ? cmdargs.input : `
    <syntax>         ::= <rule> | <rule> <syntax>
    <rule>           ::= "<" <rule_name> ">" "::=" <expression> <line_end>
    <expression>     ::= <list> | <list> "|" <expression>
    <line_end>       ::= "\n" | <line_end> <line_end>
    <list>           ::= <term> | <term> <list>
    <term>           ::= <literal> | "<" <rule_name> ">"
    <literal>        ::= "\\"" <text> "\\""
    <text>           ::= TEXT
    <rule_name>      ::= NAME
`

if (!(cmdargs.names || cmdargs.values)) {
    console.log("text:", text);
}

var tokenizer = new lexing.Tokenizer(rules);
var input = new lexing.StringIterator(text);
var output = tokenizer.map(input);

if (cmdargs.names && !cmdargs.values) {
    arr = []
    do {
        var token = output.next();
        if (token.value != null) {
            arr.push(token.name);
        }
    } while (token.name !== 'EOF');
    console.log(util.inspect(arr, { maxArrayLength: null }))
} else if (cmdargs.values && !cmdargs.names) {
    arr = []
    console.log('[')
    do {
        var token = output.next();
        if (token.value != null) {
            console.log('    "%s",', token.value)
        }
    } while (token.name !== 'EOF');
    console.log(']')
} else {
    do {
        var token = output.next();
        console.log('token=%s => %j', token.name, token.value);
    } while (token.name !== 'EOF');
}

