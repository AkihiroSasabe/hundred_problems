// 答えが合わないのでパス 2022-04-12
// おそらく、DAGでは無いテストケースで死んでいるっぽい
use std::io;
use std::cmp::min;

// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
#[derive(Debug, Clone)]
struct Edge {
    neighbor: usize,
    weight: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let v_num = input[0];
    let e_num = input[1];
    let r = input[2];

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; v_num];
    for i in 0..e_num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let s = input[0];
        let t = input[1];
        let d = input[2];
        graph[s].push(Edge {neighbor: t, weight: d});
    }

    // // 入力確認
    // for i in 0..v_num {
    //     println!("{}: {:?}", i, graph[i]);
    // }

    // トポロジカルソートをする (計算量: O(v_num))
    let mut order = vec![];
    let mut seen = vec![false; v_num];
    dfs(r, &mut graph, &mut seen, &mut order);
    order.reverse();
    // println!("order: {:?}", order);

    // 動的計画法で最小経路を求める (計算量: O(e_num))
    // dp[i] i番目以前の頂点における、重みの総和の最小値
    let INF = 1 << 60; // usizeが取りうる値は0~2^64。
    let mut dp = vec![INF; v_num];
    dp[order[0]] = 0;
    for i in order.iter() {
        // 「貰う型」ではなく、「配る型」の動的計画法
        for edge in graph[*i].iter() {
            dp[edge.neighbor] = min(dp[edge.neighbor], dp[*i] + edge.weight);
            if *i == 0 {
                println!("{}: dp{}", edge.neighbor, dp[edge.neighbor]) 
            }

        }
    }

    // for i in order {
    //     println!("{}: {}", i, dp[i]);
    // }

    for i in 0..v_num {
        if dp[i] == INF {
            println!("INF");
        } else {
            println!("{:?}", dp[i]);
        }
    }



}


// トポロジカルソートをする
// ※引数graphは可変参照&mutは駄目。不変参照&にする
fn dfs(v: usize, graph: &Vec<Vec<Edge>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;
    for next_edge in graph[v].iter() {
        if seen[next_edge.neighbor] {continue}
        dfs(next_edge.neighbor, graph, seen, order);
    }
    order.push(v);
}