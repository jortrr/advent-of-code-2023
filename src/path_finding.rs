// This module contains a generic implementation of the Dijkstra pathfinding algorithm,
// because these kind of problems will often occurs in Advent of Code

use std::{cell::RefCell, cmp::min, fmt::Debug, ops::Deref, rc::Rc};

/// Custom types
type Int = i32;
type Distance = Int;
type DistanceOption = Option<Distance>;
type NodeRefs<T> = Vec<NodeRef<T>>;
type Edges<T> = Vec<Edge<T>>;
/// Reference counted mutable Node<T> (shared_ptr): https://doc.rust-lang.org/book/ch15-04-rc.html
type NodeRef<T> = Rc<RefCell<Node<T>>>;

/// Shorthand trait
trait Debuggable: PartialEq + Copy + Clone + Debug {}

/// A `Node` with a `state` and an optional `distance` to some starting `Node`.
#[derive(Clone, Copy, Debug)]
struct Node<T> {
    state: T,
    distance_option: DistanceOption,
    visited: bool,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<T> Node<T> {
    fn new(state: T, distance_option: DistanceOption) -> Node<T> {
        Node {
            state,
            distance_option,
            visited: false,
        }
    }

    fn new_ref(state: T, distance_option: DistanceOption) -> NodeRef<T> {
        Rc::new(RefCell::new(Node::new(state, distance_option)))
    }
}

/// Directed `Edge` from `first` to `second`.
#[derive(PartialEq, Debug, Clone)]
struct Edge<T> {
    first: NodeRef<T>,
    second: NodeRef<T>,
    distance: Distance,
}

impl<T> Edge<T> {
    fn new(first: NodeRef<T>, second: NodeRef<T>, distance: Distance) -> Edge<T> {
        Edge {
            first: Rc::clone(&first),
            second: Rc::clone(&second),
            distance,
        }
    }
}

struct Graph<T: Debuggable> {
    visited_nodes: NodeRefs<T>,
    unvisited_nodes: NodeRefs<T>,
    edges: Edges<T>,
    starting_node: NodeRef<T>,
}

impl<T: Debuggable> Graph<T> {
    fn new(starting_state: T) -> Graph<T> {
        let starting_node: NodeRef<T> = Node::new_ref(starting_state, Some(0));
        starting_node.borrow_mut().visited = true;
        let visited_nodes = vec![Rc::clone(&starting_node)];

        Graph {
            visited_nodes,
            unvisited_nodes: NodeRefs::new(),
            edges: Edges::new(),
            starting_node,
        }
    }

    /// Add a new `Edge` to `self.edges` from new or existing `Nodes` with specified `states``
    fn add_edge(&mut self, first_state: T, second_state: T, distance: Distance) {
        let first_node_ref: NodeRef<T> = self.insert_node(first_state);
        let second_node_ref: NodeRef<T> = self.insert_node(second_state);
        let new_edge: Edge<T> = Edge::new(first_node_ref, second_node_ref, distance);
        self.edges.push(new_edge);
    }

    /// Insert a `Node` into `self.unvisited_nodes` if there is
    /// no `Node` with specified `state` in `self.
    /// unvisited_nodes` or `self.visited_nodes`, or do nothing
    /// if such a `Node` already exists.
    /// Returns a `NodeRef<T>` to the new or existing `Node`.
    fn insert_node(&mut self, state: T) -> NodeRef<T> {
        let node_ref_option = self.get_node_ref(state);
        match node_ref_option {
            Some(node_ref) => node_ref,
            _ => {
                let new_node_ref = Node::new_ref(state, None);
                self.unvisited_nodes.push(Rc::clone(&new_node_ref));
                new_node_ref
            }
        }
    }

    /// Get a `NodeRef<T>` if a `NodeRef<T>` with specified `state` is
    /// found in `visited_nodes` or `unvisited_nodes`.
    fn get_node_ref(&self, state: T) -> Option<NodeRef<T>> {
        let all_nodes: Vec<&NodeRef<T>> = self
            .unvisited_nodes
            .iter()
            .chain(self.visited_nodes.iter())
            .collect();
        let result = all_nodes
            .iter()
            .find(|node| node.borrow().state == state)
            .cloned()
            .cloned();
        result
    }

    /// Visit a single Node with a specified state, and update all of it's unvisited neighbours with the shortest_distance to those Nodes.
    /// Will panic if the Node cannot be visited.
    fn visit(&mut self, state: &T) {
        let current_node_ref_option: Option<NodeRef<T>> = self.get_node_ref(*state);
        match current_node_ref_option {
            Some(current_node_ref) => {
                match current_node_ref.borrow().visited  {
                    true => panic!("The Node with state: '{:?}' has already been visited, cannot visit.", state),
                    false => {
                        match current_node_ref.borrow().distance_option  {
                            Some(_) => {
                        // Here we can actually visit this Node
                            self.visit_valid_node_ref(Rc::clone(&current_node_ref));
                            },
                            _ => panic!("The Node with state: '{:?}' has distance `None`, cannot visit.", state)
                        }


                    }
                }
            }
            None => panic!(
                "There is no Node with state '{:?}' in `self.unvisited_nodes` or `self.visited_nodes`, cannot visit.",
                state
            ),
        }
    }

    /// Visit a `NodeRef<T>` that is assumed to be valid, meaning that it exists, is unvisited, and has a distance value.
    /// Will update all unvisited neighbours of the `Node` with the shortest distance to those `Nodes`, or panic
    fn visit_valid_node_ref(&mut self, node_ref: NodeRef<T>) {
        // Remove current_node from unvisited_nodes, and add to visited_nodes.
        self.unvisited_nodes
            .retain(|node| *node.borrow() != *node_ref.borrow());
        self.visited_nodes.push(Rc::clone(&node_ref));
        node_ref.borrow_mut().visited = true;

        // Update all unvisited neighbours with the shortest distance to that node
        let edges_to_neighbours: Edges<T> = self.get_edges(node_ref.borrow().state);
        let distance = node_ref.borrow().distance_option.unwrap();
        for edge in edges_to_neighbours {
            if !edge.second.borrow().visited {
                let shortest_distance: Distance = match edge.second.borrow().distance_option {
                    Some(other_distance) => min(distance, other_distance),
                    None => distance,
                };
                edge.second.borrow_mut().distance_option = Some(shortest_distance);
            }
        }
    }

    /// Get the next `Node` from `self.unvisited_nodes` to visit during Dijkstra's pathfinding algorithm
    fn get_next_node_to_visit(&self) -> Option<NodeRef<T>> {
        let mut nodes_that_can_be_visited: NodeRefs<T> = self
            .unvisited_nodes
            .iter()
            .filter(|node| node.borrow().distance_option.is_some())
            .cloned()
            .collect();
        nodes_that_can_be_visited.sort_by(|a, b| {
            a.borrow()
                .distance_option
                .unwrap()
                .partial_cmp(&b.borrow().distance_option.unwrap())
                .unwrap()
        });
        nodes_that_can_be_visited.first().cloned()
    }

    /// Returns all `Edges` in `self.edges` that a `Node` with `first_state` can go to
    fn get_edges(&self, first_state: T) -> Edges<T> {
        self.edges
            .iter()
            .filter(|edge| edge.first.borrow().state == first_state)
            .cloned()
            .collect()
    }

    /// Run (Dijkstra) pathfinding algorithm to find shortest distance from self.starting_node to all other Nodes.
    pub fn run_pathfinding_algorithm(&mut self) {
        self.visit_valid_node_ref(Rc::clone(&self.starting_node));
        //TODO: Continue here
    }
}
