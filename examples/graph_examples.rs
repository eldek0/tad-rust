use eldek_tad::graph::{graph::Graph, traits::graph_traits::GraphTrait};

fn main(){
    let mut graph:Graph<i32> = Graph::new();
    graph.add_edge(10, 20, false, None);
    graph.add_edge(10, 40, true, None);
    graph.add_edge(30, 10, true, None);
    println!("{:?}", graph);
}