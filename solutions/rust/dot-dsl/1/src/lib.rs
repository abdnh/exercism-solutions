pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge<'a> {
                pub node1: &'a str,
                pub node2: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(node1: &'a str, node2: &'a str) -> Self {
                    Self {
                        node1,
                        node2,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    let attrs_map = HashMap::from_iter(attrs.iter().copied());
                    Self {
                        attrs: attrs_map,
                        ..self
                    }
                }

                pub fn attr(&self, key: &'a str) -> Option<&'a str> {
                    self.attrs.get(key).copied()
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self {
                        name: name.into(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    let attrs_map = HashMap::from_iter(attrs.iter().copied());
                    Self {
                        attrs: attrs_map,
                        ..self
                    }
                }

                pub fn attr(&self, key: &'a str) -> Option<&'a str> {
                    self.attrs.get(key).copied()
                }
            }
        }
    }

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node<'a>]) -> Self {
            Self {
                nodes: nodes.into(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge<'a>]) -> Self {
            Self {
                edges: edges.into(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
            let attrs_map = HashMap::from_iter(attrs.iter().copied());
            Self {
                attrs: attrs_map,
                ..self
            }
        }

        pub fn node(&self, name: &'a str) -> Option<Node<'a>> {
            self.nodes.iter().find(|n| n.name == name).cloned()
        }
    }
}
