// 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
// 単純なダイクストラ法 計算量: O(|V|^2)
// ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
//     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
//     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース
use std::io;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// BinaryHeapの根を最大値ではなく最小値にするために構造体を書き換える
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
// impl トレイト名 for 構造体名
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
// impl トレイト名 for 構造体名
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    let mut distance = vec![INF; v_num];
    distance[r] = 0;

    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
    let mut heap = BinaryHeap::new();
    heap.push(State {cost: distance[r], position: r});
    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let mut min_v = state.position;
        let mut min_dist = state.cost;

        // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
        if min_dist > distance[min_v] {continue}

        // min_vを始点とした辺の緩和
        for edge in graph[min_v].iter() {
            // 緩和できる場合
            if distance[edge.neighbor] > distance[min_v] + edge.weight {
                // 緩和
                distance[edge.neighbor] = min(distance[edge.neighbor], distance[min_v] + edge.weight);
                // 到達可能で最短距離が未確定な頂点リストに追加
                heap.push( State {cost: distance[edge.neighbor], position: edge.neighbor});
            }
        }
    }

    // // 単純なダイクストラ法 計算量: O(|V|^2)
    // for _ in 0..v_num {
    //     // 収束していない集合の中で最小距離の頂点を探す
    //     let mut min_dist = INF;
    //     let mut min_v = INF;
    //     for v in 0..v_num {
    //         if !converged_list[v] && distance[v] < min_dist {
    //             min_dist = distance[v];
    //             min_v = v;
    //         }
    //     }
    //     // 新たに収束リストに加わるものが無ければ完了なので打ち切り(全頂点が収束済み)
    //     if min_v == INF {break}
    //     converged_list[min_v] = true;

    //     // min_vを始点とした辺を緩和
    //     for edge in graph[min_v].iter() {
    //         // 「貰う型」ではなく、「配る型」の動的計画法
    //         distance[edge.neighbor] = min(distance[edge.neighbor], distance[min_v] + edge.weight);
    //     }
    // }


    for i in 0..v_num {
        if distance[i] == INF {
            println!("INF");
        } else {
            println!("{:?}", distance[i]);
        }
    }
}
