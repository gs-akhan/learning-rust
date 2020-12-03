#[allow(unused_imports)]
use graph::graph_items::node::Node;
use graph::*;
fn main() {
    println!("Learning Graph");
    let azhar = Node::new("Azhar").with_attributes(&[("name", "azhar"), ("roll_no", "21")]);
    println!("This is node {:?}", azhar);
    let g = Graph::new().with_nodes(vec![azhar]);

    println!("This is Graph {:?}", g);
}

pub mod graph {

    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq)]

    pub struct Graph {
        nodes: Vec<graph_items::node::Node>,
        edges: Vec<graph_items::edge::Edge>,
        attributes: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attributes: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes;
            self
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Eq, PartialEq)]
            pub struct Node {
                name: String,
                attributes: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attributes: HashMap::new(),
                    }
                }

                pub fn with_attributes(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut m = HashMap::new();
                    for attr in attrs {
                        m.insert(String::from(attr.0), String::from(attr.1));
                    }
                    self.attributes = m;
                    self
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Eq, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attributes: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: String, to: String) -> Self {
                    Edge {
                        from: String::from(from),
                        to: String::from(to),
                        attributes: HashMap::new(),
                    }
                }
            }
        }
    }
}
