use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node(char);

#[derive(Debug, PartialEq, Eq)]
enum Algorithm {
    DFS,
    BFS,
}

fn main() {
    let mut graph = HashMap::new();

    loop {
        println!("Menu:");
        println!("1. Add edge");
        println!("2. Show graph");
        println!("3. Perform DFS");
        println!("4. Perform BFS");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter edge (format: A-B): ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let edge = input.trim();

                let nodes: Vec<&str> = edge.split('-').collect();
                if nodes.len() != 2 {
                    println!("Invalid edge format.");
                } else {
                    let node1 = Node(nodes[0].chars().next().unwrap());
                    let node2 = Node(nodes[1].chars().next().unwrap());
                    add_edge(&mut graph, node1, node2);
                }
            }
            2 => {
                println!("Graph: {:?}", graph);
            }
            3 => {
                let start_node = get_start_node();
                if let Some(node) = start_node {
                    let order = traverse(&graph, node, Algorithm::DFS);
                    println!("DFS order: {:?}", order);
                }
            }
            4 => {
                let start_node = get_start_node();
                if let Some(node) = start_node {
                    let order = traverse(&graph, node, Algorithm::BFS);
                    println!("BFS order: {:?}", order);
                }
            }
            5 => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn get_start_node() -> Option<Node> {
    print!("Enter starting node: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.len() != 1 {
        println!("Invalid node format.");
        None
    } else {
        Some(Node(input.chars().next().unwrap()))
    }
}

fn add_edge(graph: &mut HashMap<Node, Vec<Node>>, node1: Node, node2: Node) {
    graph.entry(node1).or_insert_with(Vec::new).push(node2);
    graph.entry(node2).or_insert_with(Vec::new).push(node1);
}

fn traverse(graph: &HashMap<Node, Vec<Node>>, start: Node, algo: Algorithm) -> Vec<Node> {
    let mut visited = HashSet::new();
    let mut order = Vec::new();

    match algo {
        Algorithm::DFS => dfs(graph, start, &mut visited, &mut order),
        Algorithm::BFS => bfs(graph, start, &mut visited, &mut order),
    }
    order
}

fn dfs(graph: &HashMap<Node, Vec<Node>>, node: Node, visited: &mut HashSet<Node>, order: &mut Vec<Node>) {
    if !visited.contains(&node) {
        visited.insert(node);
        order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                dfs(graph, *neighbor, visited, order);
            }
        }
    }
}

fn bfs(graph: &HashMap<Node, Vec<Node>>, start: Node, visited: &mut HashSet<Node>, order: &mut Vec<Node>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if !visited.contains(&node) {
            visited.insert(node);
            order.push(node);

            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    queue.push_back(*neighbor);
                }
            }
        }
    }
}
