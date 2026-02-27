#[cfg(test)]
mod test{
    use crate::{graph::{graph::Graph, traits::graph_traits::GraphTrait}, linked_list::traits::linked_list_traits::LinkedListTrait};

    #[test]
    fn create() {
        let graph: Graph<i32> = Graph::new();

        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn add_vertex_increases_count() {
        let mut graph: Graph<i32> = Graph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);

        assert_eq!(graph.vertex_count(), 2);
        assert!(graph.has_vertex(&1));
        assert!(graph.has_vertex(&2));
        assert!(!graph.has_vertex(&3));
    }

    #[test]
    fn add_edge_unidirectional() {
        let mut graph: Graph<i32> = Graph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);

        graph.add_edge(1, 2, false, None);

        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn add_edge_bidirectional() {
        let mut graph: Graph<i32> = Graph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);

        graph.add_edge(1, 2, true, None);

        assert!(graph.has_edge(&1, &2));
        assert!(graph.has_edge(&2, &1));
        assert_eq!(graph.edge_count(), 2); 
    }

    #[test]
    fn get_neighbors_returns_correct_list() {
        let mut graph: Graph<i32> = Graph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);

        graph.add_edge(1, 2, false, None);
        graph.add_edge(1, 3, false, None);

        let neighbors = graph.get_neighbors(&1);

        assert_eq!(neighbors.size(), 2);
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
    }

    #[test]
    fn edge_count_correct_multiple_edges() {
        let mut graph: Graph<i32> = Graph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);

        graph.add_edge(1, 2, false, None);
        graph.add_edge(2, 3, false, None);

        assert_eq!(graph.edge_count(), 2);
    }
}