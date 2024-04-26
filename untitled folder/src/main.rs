use std::fs::File;
use std::io::{self, BufRead, BufReader};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use petgraph::dot::Dot;
use rand::{thread_rng, seq::SliceRandom};
use std::collections::VecDeque;

fn main() {
    let edges = read_file("roadNet-PA.txt");
    let sample_size = 100; 
    let sampled_edges = shuffle_and_sample(&edges, sample_size);
    
    let adjacency_list = create_adjacency_list(&edges);
    for &start in adjacency_list.keys() {
        let distances = bfs(&adjacency_list, start);
        println!("Distances from node {}: {:?}", start, distances);
    }
}


fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    for (idx, line) in buf_reader.enumerate() {
        let line_str = line.expect("Error reading line from file");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to parse 'from' node identifier");
        let y = v[1].parse::<usize>().expect("Failed to parse 'to' node identifier");
        result.push((x, y));
    }
    println!("Read {} lines of edge data.", result.len());
    result
}

fn construct_graph(edges: Vec<(usize, usize)>, num_nodes: usize) -> DiGraph<(), ()> {
    let mut graph = DiGraph::<(), ()>::with_capacity(num_nodes, edges.len());
    let mut node_indices: HashMap<usize, NodeIndex> = HashMap::new();

    for (from_node, to_node) in edges {
        let from_index = *node_indices.entry(from_node).or_insert_with(|| graph.add_node(()));
        let to_index = *node_indices.entry(to_node).or_insert_with(|| graph.add_node(()));
        graph.add_edge(from_index, to_index, ());
    }

    graph
}

fn shuffle_and_sample(edges: &Vec<(usize, usize)>, sample_size: usize) -> Vec<(usize, usize)> {
    let mut rng = thread_rng();
    let mut shuffled_edges = edges.clone();
    shuffled_edges.shuffle(&mut rng);
    shuffled_edges.into_iter().take(sample_size).collect()
}

fn create_adjacency_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
    }
    adjacency_list
}


fn bfs(adjacency_list: &HashMap<usize, Vec<usize>>, start: usize) -> HashMap<usize, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    queue.push_back(start);
    distances.insert(start, 0);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }
    distances
}