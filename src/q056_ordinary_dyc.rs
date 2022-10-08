// 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
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


    // ダイクストラ法
    let INF = 1 << 60; // usizeが取りうる値は0~2^64。
    let mut converged_list = vec![false; v_num];

    // 単純なダイクストラ法 計算量: O(|V|^2)
    // let mut distance = vec![INF; v_num];
    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    // To be implemented..
    distance[r] = 0;
    for _ in 0..v_num {
        // 収束していない集合の中で最小距離の頂点を探す
        let mut min_dist = INF;
        let mut min_v = INF;
        for v in 0..v_num {
            if !converged_list[v] && distance[v] < min_dist {
                min_dist = distance[v];
                min_v = v;
            }
        }
        // 新たに収束リストに加わるものが無ければ完了なので打ち切り(全頂点が収束済み)
        if min_v == INF {break}
        converged_list[min_v] = true;

        // min_vを始点とした辺を緩和
        for edge in graph[min_v].iter() {
            // 「貰う型」ではなく、「配る型」の動的計画法
            distance[edge.neighbor] = min(distance[edge.neighbor], distance[min_v] + edge.weight);
        }
    }


    for i in 0..v_num {
        if distance[i] == INF {
            println!("INF");
        } else {
            println!("{:?}", distance[i]);
        }
    }
}
