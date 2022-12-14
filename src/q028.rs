use std::io;
use std::collections::VecDeque;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("");
    let n: usize = n.trim().parse().expect("");

    // グラフ
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let line: Vec<usize> = line.split_whitespace().map(|x| (x.parse::<isize>().expect("") - 1) as usize).collect();
        graph[i] = line[2..].to_vec();
    }
    // println!("{:?}", graph);

    let mut distance = vec![-1; n];
    distance[0] = 0;
    // Queue
    let mut todo: VecDeque<usize> = VecDeque::new();
    todo.push_back(0);
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        for next_v in &graph[v] {
            if distance[*next_v] != -1 {continue}
            distance[*next_v] = distance[v] + 1;
            todo.push_back(*next_v)
        }
    }

    for i in 0..n {
        println!("{} {}", i + 1, distance[i]);
    }
}
