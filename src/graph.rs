use std::ops::{Index, IndexMut};

use crate::{app::App, note::Note, vec2::Vector2};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    pub read: bool,
    pub write: bool,
    pub edges: Vec<usize>,
    pub position: Vector2,
    pub note: Option<Note>,
    pub ruleset: String,
}

impl Index<usize> for Graph {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Graph {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Node {
    pub fn new(
        read: bool,
        write: bool,
        edges: Vec<usize>,
        position: Vector2,
        ruleset: String,
    ) -> Self {
        Self {
            read,
            write,
            edges,
            position,
            note: None,
            ruleset,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn copy(&self, selection: &[usize]) -> Self {
        let mut indexes = vec![None; self.nodes.len()];
        let mut new_graph = Graph::new();

        for selected in selection {
            let mut new_node = self.nodes[*selected].clone();
            new_node.edges = vec![];
            new_graph.add_node(new_node);
            indexes[*selected] = Some(new_graph.nodes.len() - 1)
        }
        println!("{:?}", indexes);
        for (i, &selected) in selection.iter().enumerate() {
            for j in &self.nodes[selected].edges {
                if let Some(new_index) = indexes[*j] {
                    println!("{i}");
                    new_graph.add_edge(i, new_index);
                }
            }
        }
        new_graph
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node)
    }
    pub fn remove_node(&mut self, idx: usize) {
        self.nodes.swap_remove(idx);
        let len = self.nodes.len();
        for node in self.nodes.iter_mut() {
            node.edges.retain(|a| *a != idx);
            node.edges = node
                .edges
                .iter()
                .map(|a| if *a == len { idx } else { *a })
                .collect();
        }
    }

    pub fn remove_node_from_app(&mut self, idx: usize, app: &mut App) {
        self.remove_node(idx);

        let len = self.nodes.len();
        app.selected.retain(|a| *a != idx);
        app.selected = app
            .selected
            .iter()
            .map(|a| if *a == len { idx } else { *a })
            .collect();
    }

    pub fn add_edge(&mut self, u: usize, v: usize) -> bool {
        if !self.nodes[u].edges.contains(&v) {
            self.nodes[u].edges.push(v);
            true
        } else {
            false
        }
    }

    pub fn remove_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].edges.retain(|a| *a != v);
    }
}
