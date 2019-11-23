#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate pest;

use pest::Parser as _;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct PavrusParser;


#[test]
fn parse_sample() {
    let source = include_str!("./data/sample.pavrus");
    let module = PavrusParser::parse(Rule::module, &source)
        .map_err(|e| println!("{}", e))
        .unwrap();
    println!("{:?}", module);
}

#[test]
fn parse_ident() {
    parses_to!(
        parser: PavrusParser,
        input:  "abcde",
        rule:   Rule::ident,
        tokens: [
            ident(0, 5)
        ]
    );
}

// factor = {ident ~ selector | number | "(" ~ expression ~ ")" | "~" ~ factor}
#[test]
fn parse_factor() {
    parses_to!(
        parser: PavrusParser,
        input:  "a",
        rule:   Rule::factor,
        tokens: [
            factor(0, 1, [ident(0, 1)])
        ]
    );
    parses_to!(
        parser: PavrusParser,
        input:  "a.b",
        rule:   Rule::factor,
        tokens: [
            factor(0, 3, [
                ident(0, 1),
                selector(1, 3, [
                    ident(2, 3)
                ])
            ])
        ]
    );
    parses_to!(
        parser: PavrusParser,
        input:  "20",
        rule:   Rule::factor,
        tokens: [
            factor(0, 2, [
                integer(0, 2)
            ])
        ]
    );
}

#[test]
fn parse_expression() {
    let source = "x <= y";
    let rule = Rule::expression;
    let tokens = PavrusParser::parse(rule, &source)
        .map_err(|e| println!("{}", e))
        .unwrap()
        .tokens();
    println!("{:?}", tokens);
    parses_to!(
        parser: PavrusParser,
        input:  source,
        rule:   Rule::expression,
        tokens: [
            expression(0, 6, [
                SimpleExpression(0, 2, [term(0, 2, [factor(0, 2, [ident(0, 1)])])]),
                SimpleExpression(5, 6, [term(5, 6, [factor(5, 6, [ident(5, 6)])])]),
            ])
        ]
    );
}