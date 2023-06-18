use std::iter::Peekable;

use logos::{Lexer, Logos};

use crate::automaton::{BoolPattern, IntExpr, Ruleset};
peg::parser! {
    pub grammar expr_parser() for str {
        rule on() -> IntExpr
            = whitespace()? "on" whitespace()? {IntExpr::On}
        rule off() -> IntExpr
            = whitespace()? "off" whitespace()? {IntExpr::Off}
        rule input() -> IntExpr
            = whitespace()? "in" whitespace()? {IntExpr::In}
        rule value() -> IntExpr = x:int() / x:on() / x:off() / x:input()  {
            x
        }
        rule number() -> i32
          = whitespace()? n:$(['0'..='9']+) whitespace()? {? n.parse().or(Err("u32")) }
        rule int() -> IntExpr =
            x:number() {IntExpr::Lit(x)}

        rule whitespace() = quiet!{[' ' | '\t']+}

        pub rule arithmetic() -> IntExpr = precedence!{
          x:(@) "+" y:@ { IntExpr::Add(Box::new(x), Box::new(y)) }
          x:(@) "-" y:@ { IntExpr::Sub(Box::new(x), Box::new(y)) }
          --
          x:(@) "*" y:@ { IntExpr::Mul(Box::new(x), Box::new(y)) }
          x:(@) "/" y:@ { IntExpr::Div(Box::new(x), Box::new(y)) }
          x:(@) "%" y:@ { IntExpr::Mod(Box::new(x), Box::new(y)) }
          --
          n:value() { n }
          "(" e:arithmetic() ")" { e }
        }

        rule gth() -> BoolPattern =
          x:arithmetic() ">" y:arithmetic() { BoolPattern::Gth(x, y) }
        rule lth() -> BoolPattern =
          x:arithmetic() "<" y:arithmetic() { BoolPattern::Lth(x, y) }
        rule eq() -> BoolPattern =
          x:arithmetic() "=" y:arithmetic() { BoolPattern::Equal(x, y) }
        rule geq() -> BoolPattern =
          x:arithmetic() "<=" y:arithmetic() { BoolPattern::Not(Box::new(BoolPattern::Gth(x, y))) }
        rule leq() -> BoolPattern =
          x:arithmetic() ">=" y:arithmetic() { BoolPattern::Not(Box::new(BoolPattern::Lth(x, y))) }

        rule compare() -> BoolPattern =
            a:eq() / a:lth() / a:gth() / a:geq() / a:leq() {a}

        rule or() -> BoolPattern =
            x:compare() "|" y:compare() {BoolPattern::Or(Box::new(x), Box::new(y))}
        rule and() -> BoolPattern =
            x:compare() "&" y:compare() {BoolPattern::And(Box::new(x), Box::new(y))}
        rule not() -> BoolPattern =
            "!" "(" x:compare()")" {BoolPattern::Not(Box::new(x))}

        pub rule bools() -> BoolPattern = precedence! {
            x:(@) "|" y:@ {BoolPattern::Or(Box::new(x), Box::new(y))}
            --
            x:(@) "&" y:@ {BoolPattern::And(Box::new(x), Box::new(y))}
            --
            "!" x:(@) {BoolPattern::Not(Box::new(x))}
            --
            n:compare() {n}
            "self" {BoolPattern::MyValue}
        }

        rule on_to_bool() -> bool =
            "on" {true}
        rule off_to_bool() -> bool =
            "off" {false}
        rule on_off_to_bool() -> bool =
            x:on_to_bool() / x:off_to_bool() {x}

        pub rule ruleset() -> Ruleset =
            name:$(['a'..='z']+) whitespace() pattern:bools() whitespace()? ":" whitespace()? case: on_off_to_bool() whitespace()? {Ruleset { pattern: pattern, case, name: name.to_string() }}
    }
}
