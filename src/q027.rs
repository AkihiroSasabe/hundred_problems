use std::io;

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

    println!("{:?}", graph);

}