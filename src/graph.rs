#[allow(unused_imports)]

use graph::graph_items::node::Node;

fn main() {
    println!("Learning Graph");
    let azhar = Node::new("Azhar").with_attributes(&[("name", "azhar"), ("roll_no", "21")]);
    println!("{:?}", azhar);
}

pub mod graph {

    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq)]

    pub struct Graph {
        nodes: Vec<graph_items::node::Node>,
        edges: Vec<graph_items::edge::Edge>,
        attributes: HashMap<String, String>,
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
            #[derive(Debug, Eq, PartialEq)]
            pub struct Edge {}
        }
    }
}
