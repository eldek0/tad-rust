use std::{fmt::Debug, hash::Hash};

use crate::{graph::traits::graph_traits::GraphTrait, hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait}, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}};

/// A generic directed or undirected graph data structure.
///
/// Vertices hold values of type `T`. Edges can be unidirectional or bidirectional,
/// and optionally carry a weight.
///
/// # Examples
///
/// ## Creating a graph
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let graph: Graph<i32> = Graph::new();
/// assert_eq!(0, graph.vertex_count());
/// assert_eq!(0, graph.edge_count());
/// ```
///
/// ## Adding vertices
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
///
/// assert_eq!(2, graph.vertex_count());
/// assert!(graph.has_vertex(&1));
/// assert!(!graph.has_vertex(&99));
/// ```
///
/// ## Adding a unidirectional edge
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
///
/// graph.add_edge(1, 2, false, None); // 1 → 2 only
///
/// assert!(graph.has_edge(&1, &2));
/// assert!(!graph.has_edge(&2, &1)); // reverse does not exist
/// assert_eq!(1, graph.edge_count());
/// ```
///
/// ## Adding a bidirectional edge
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
///
/// graph.add_edge(1, 2, true, None); // 1 ↔ 2
///
/// assert!(graph.has_edge(&1, &2));
/// assert!(graph.has_edge(&2, &1)); // reverse also exists
/// assert_eq!(2, graph.edge_count()); // counts as 2 directed edges
/// ```
///
/// ## Adding a weighted edge
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
///
/// graph.add_edge(1, 2, false, Some(45)); // edge with weight 45
/// assert!(graph.has_edge(&1, &2));
/// ```
///
/// ## Getting neighbors of a vertex
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// use eldek_tad::linked_list::traits::linked_list_traits::LinkedListTrait;
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
/// graph.add_vertex(3);
///
/// graph.add_edge(1, 2, false, None);
/// graph.add_edge(1, 3, false, None);
///
/// let neighbors = graph.get_neighbors(&1);
/// assert_eq!(2, neighbors.size());
/// assert!(neighbors.contains(&2));
/// assert!(neighbors.contains(&3));
/// ```
///
/// ## Multiple edges between different vertices
/// ```
/// use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};
/// 
/// let mut graph: Graph<i32> = Graph::new();
/// graph.add_vertex(1);
/// graph.add_vertex(2);
/// graph.add_vertex(3);
///
/// graph.add_edge(1, 2, false, None);
/// graph.add_edge(2, 3, false, None);
///
/// assert_eq!(2, graph.edge_count());
/// ```
pub struct Graph<T:Hash+Clone>{
    map:HashMap<T, HashMap<T, i32>>,
}

impl <T:Hash+Debug+Eq+Clone> GraphTrait<T> for Graph<T>  {
    fn new()->Self {
        return Graph { map: HashMap::new(1024) }
    }

    fn add_vertex(&mut self, vertex:T) {
        self.map.insert(vertex, HashMap::new(1024));
    }

    fn add_edge(&mut self, source:T, destination:T, bidireccional:bool, weight:Option<i32>) {
        let w = weight.unwrap_or(0);

        if !self.map.contains_key(&source){
            self.add_vertex(source.clone());
        }

        if !self.map.contains_key(&destination){
            self.add_vertex(destination.clone());
        }

        self.map.get_mut(&source).unwrap().insert(destination.clone(), w);
        if bidireccional{
            self.map.get_mut(&destination).unwrap().insert(source, w);
        }
    }

    fn has_vertex(&self, vertex:&T)->bool {
        return self.map.contains_key(vertex);
    }

    fn has_edge(&self, vertex1:&T, vertex2:&T)->bool {
        if self.has_vertex(vertex1){
            return self.map.get(vertex1).unwrap().contains_key(vertex2);
        }
        return false;
    }

    fn get_neighbors(&self, vertex:&T)->LinkedList<T> {
        let mut neighbors = LinkedList::new();
        if !self.has_vertex(vertex){
            return neighbors;
        }

        for (k, _) in self.map.get(vertex).unwrap().iter() {
            neighbors.push(k.clone());
        }

        neighbors
    }

    fn vertex_count(&self)->usize {
        return self.map.size();
    }

    fn edge_count(&self)->usize {
        let mut count = 0;
        for (_, v) in self.map.iter(){
            count += v.size();
        }

        return count;
    }
    
    fn is_empty(&self)->bool {
        return self.map.is_empty();
    }
}

/// Debug print
impl <T:Hash+Debug+Clone+PartialEq> Debug for Graph<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (k1, v1) in self.map.iter(){
            write!(f, "\t{:?} -> ", k1)?;
            write!(f, "[")?;
            let mut first = true;
            for (k2, _) in v1.iter() {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", k2)?;
                first = false;
            }
            writeln!(f, "]")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}