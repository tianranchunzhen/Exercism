pub mod graph {
    use crate::graph::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub mod graph_items {
        use std::collections::HashMap;

        pub mod edge {
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: super::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: super::HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
        pub mod node {
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                name: String,
                attrs: super::HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: super::HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(|attr| attr.as_str())
                }

                pub fn name(&self) -> String {
                    self.name.clone()
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name() == name)
        }
    }
}
