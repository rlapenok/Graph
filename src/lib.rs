pub mod graph_hm;

#[cfg(test)]
mod tests_vertex {

    

    use crate::graph_hm::{graph::Graph};
    #[test]
    fn graph() {
        let data = ["Hello", "ccx", "aerewr", "tjyjty", "iuouioui", "lkjklk", "ewrwer", "bbb", "poiu"];
        let mut graph = Graph::new();

        for i in 0..data.len() {
            graph.push_vertex();
            graph.insert_change_value(i + 1, data[i]);
        }
        graph.insert_edge(1, 2);
        graph.insert_edge(1, 3);
        graph.insert_edge(2, 4);
        graph.insert_edge(2, 5);
        graph.insert_edge(2, 6);
        graph.insert_edge(3, 7);
        graph.insert_edge(3, 8);
        graph.insert_edge(3, 9);

        //println!("{:?}",graph);
        graph.bfs("ccx");
    }
}
