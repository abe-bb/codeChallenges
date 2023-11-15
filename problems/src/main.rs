fn main() {
    let graph_input = vec![
        vec![7, 2, 131570],
        vec![9, 4, 622890],
        vec![9, 1, 812365],
        vec![1, 3, 399349],
        vec![10, 2, 407736],
        vec![6, 7, 880509],
        vec![1, 4, 289656],
        vec![8, 0, 802664],
        vec![6, 4, 826732],
        vec![10, 3, 567982],
        vec![5, 6, 434340],
        vec![4, 7, 833968],
        vec![12, 1, 578047],
        vec![8, 5, 739814],
        vec![10, 9, 648073],
        vec![1, 6, 679167],
        vec![3, 6, 933017],
        vec![0, 10, 399226],
        vec![1, 11, 915959],
        vec![0, 12, 393037],
        vec![11, 5, 811057],
        vec![6, 2, 100832],
        vec![5, 1, 731872],
        vec![3, 8, 741455],
        vec![2, 9, 835397],
        vec![7, 0, 516610],
        vec![11, 8, 680504],
        vec![3, 11, 455056],
        vec![1, 0, 252721],
    ];
    let mut graph = problems::Graph::new(13, graph_input);

    // assert_eq!(1211714, graph.shortest_path(9, 3));

    // println!("{:?}", graph.adjacency.get(&11));

    println!("{}", graph.shortest_path(3, 10));
    // graph.add_edge(vec![11, 1, 873094]);

    // println!("{:?}", graph.adjacency.get(&11));
    // assert_eq!(1943345, graph.shortest_path(3, 10));
}
