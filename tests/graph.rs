use rust_dsa::graph::Graph;

#[test]
fn test_graph() {
    let mut graph = Graph::new();
    graph.route(1, 2);
    graph.route(2, 3);

    let shortest_length = graph.get_shortest_path_length(&1, &3);
    assert_eq!(shortest_length, 2);

    let shortest_path = graph.get_shortest_path(&1, &3);
    assert_eq!(shortest_path, vec![1, 2, 3]);

    graph.route(1, 3);
    let shortest_path = graph.get_shortest_path(&1, &3);
    assert_eq!(shortest_path, vec![1, 3]);

    let children_of_1 = graph.get_children(1);
    assert_eq!(children_of_1.unwrap(), &vec![2, 3]);
}
