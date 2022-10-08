use proconio::input;
// 参考: https://itliberta.tech/ichi-ichi-abc138-d/

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n-1 {
        input!{
            a: usize,
            b: usize,
        }
        // 木は無向グラフ
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }

    // 単純にやると計算量をNQ <= 4 * 10 ^ 10 になる
    // 制限時間2秒なので2*10^8ループの計算量しかできない
    // 子に親のcounterの値を足すようにすれば、N <= 2 * 10 ^ 5にできる
    let mut counter = vec![0; n];
    for _ in 0..q {
        input! {
            p: usize,
            x: usize,
        }        
        counter[p - 1] += x;
    }

    // println!("{} {}", n, q);
    // println!("px {:?}", px);
    // println!("graph {:?}", graph);

    let mut seen = vec![false; n];
    let parent = -1;
    dfs(0, &mut graph, &mut counter, parent);
        
    for i in counter {
        print!("{} ", i);
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, counter: &mut Vec<usize>, parent: isize) {
    // println!("v: {}", v);
    // println!("counter[]: {:?}", counter);
    for v_next in graph[v].iter() {
        // 逆流を防ぐ
        if *v_next as isize == parent {
            // println!("prevent back stream");
            continue
        }
        // if seen[*v_next] {continue}
        counter[*v_next] += counter[v];
        dfs(*v_next, graph, counter, v as isize);
    }
}