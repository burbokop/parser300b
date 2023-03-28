import bnfgen = require('bnfgen');
import os = require('os');
import { parse } from 'ts-command-line-args';

interface Args {
    grammar: string;
    count: number;
    separator: string;
    maxReductions: number,
    maxNonprodReductions: number,
    help?: boolean;
}

const args = parse<Args>(
    {
        grammar: { type: String, optional: true, alias: 'g', description: 'Input bnf grammar' },
        count: { type: Number, optional: true, alias: 'n', defaultValue: 1, description: 'Count to generate' },
        separator: { type: String, optional: true, alias: 's', defaultValue: ' ', description: 'Symbol separator' },
        maxReductions: { type: Number, optional: true, alias: 'r', defaultValue: 1000, description: 'Max resuctions' },
        maxNonprodReductions: { type: Number, optional: true, alias: 'R', defaultValue: 1000, description: 'Max non productive reductions' },
        help: { type: Boolean, optional: true, alias: 'h', description: 'Prints this usage guide' },
    },
    {
        helpArg: 'help',
        headerContentSections: [{ header: 'Bnf generator', content: 'Generates random text using bnf grammar' }],
    },
);

if (!args.grammar) {
    throw Error("grammar is obligatory")
}

bnfgen.loadGrammar(
    args.grammar
        .split(os.EOL)
        .filter(line => line.length > 0)
        .map(line => `${line} ;`)
        .join(os.EOL)
);

bnfgen.symbolSeparator = args.separator;
bnfgen.maxReductions = args.maxReductions
bnfgen.maxNonproductiveReductions = args.maxNonprodReductions
bnfgen.debug = false
bnfgen.dumpStack = false
bnfgen.debugFunction = console.log

let result = []
for(let i = 0; i < 10000 && result.length < args.count; ++i) {
    try {
        result.push(
            bnfgen
                .generate('block')
                .split(args.separator)
                .filter(token => token.length > 0)
        )
    } catch (error) {}
}

console.log(JSON.stringify(result))