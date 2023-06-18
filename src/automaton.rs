use std::collections::HashMap;

use crate::graph::{Graph, Node};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Automaton {
    pub rules: HashMap<String, Ruleset>,
    pub graph: Graph,
}

impl Automaton {
    pub fn new(rules: HashMap<String, Ruleset>, graph: Graph) -> Self {
        Self { rules, graph }
    }
    pub fn step(&mut self) {
        for node in self.graph.nodes.iter_mut() {
            std::mem::swap(&mut node.read, &mut node.write);
        }

        for node in 0..self.graph.nodes.len() {
            if let Some(rule) = self.rules.get(&self.graph[node].ruleset) {
                rule.apply(node, &mut self.graph);
            } else {
                println!("no rule found for '{}'", self.graph[node].ruleset)
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Ruleset {
    pub pattern: BoolPattern,
    pub case: bool,
    pub name: String,
}

impl Ruleset {
    fn apply(&self, node: usize, graph: &mut Graph) {
        graph.nodes[node].write = if self.pattern.calculate(&graph.nodes[node], graph) {
            self.case
        } else {
            !self.case
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum BoolPattern {
    Or(Box<BoolPattern>, Box<BoolPattern>),
    And(Box<BoolPattern>, Box<BoolPattern>),
    Not(Box<BoolPattern>),
    Equal(IntExpr, IntExpr),
    Gth(IntExpr, IntExpr),
    Lth(IntExpr, IntExpr),
    MyValue,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum IntExpr {
    On,
    Off,
    In,
    Lit(i32),
    Add(Box<IntExpr>, Box<IntExpr>),
    Sub(Box<IntExpr>, Box<IntExpr>),
    Mul(Box<IntExpr>, Box<IntExpr>),
    Div(Box<IntExpr>, Box<IntExpr>),
    Mod(Box<IntExpr>, Box<IntExpr>),
}

struct Statement {
    replacement: bool,
    pattern: BoolPattern,
}

impl BoolPattern {
    fn calculate(&self, node: &Node, graph: &Graph) -> bool {
        match self {
            BoolPattern::Or(left, right) => {
                left.calculate(node, graph) || right.calculate(node, graph)
            }
            BoolPattern::And(left, right) => {
                left.calculate(node, graph) && right.calculate(node, graph)
            }
            BoolPattern::Not(left) => !left.calculate(node, graph),
            BoolPattern::Equal(left, right) => {
                left.calculate(node, graph) == right.calculate(node, graph)
            }
            BoolPattern::Gth(left, right) => {
                left.calculate(node, graph) > right.calculate(node, graph)
            }
            BoolPattern::Lth(left, right) => {
                left.calculate(node, graph) < right.calculate(node, graph)
            }
            BoolPattern::MyValue => node.read,
        }
    }
}

impl IntExpr {
    fn calculate(&self, node: &Node, graph: &Graph) -> i32 {
        match self {
            IntExpr::On => node.edges.iter().filter(|a| graph[**a].read).count() as i32,
            IntExpr::Off => node.edges.iter().filter(|a| !graph.nodes[**a].read).count() as i32,
            IntExpr::In => node.edges.len() as i32,
            IntExpr::Lit(num) => *num,
            IntExpr::Add(left, right) => left.calculate(node, graph) + right.calculate(node, graph),
            IntExpr::Sub(left, right) => left.calculate(node, graph) - right.calculate(node, graph),
            IntExpr::Mul(left, right) => left.calculate(node, graph) * right.calculate(node, graph),
            IntExpr::Div(left, right) => left.calculate(node, graph) / right.calculate(node, graph),
            IntExpr::Mod(left, right) => left.calculate(node, graph) % right.calculate(node, graph),
        }
    }
}
