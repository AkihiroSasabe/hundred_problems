use std::io;
use std::cmp::{max, min};

const INF: usize = 1 << 60;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    let s: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = s[0];
    let m = s[1];

    let mut s = String::new();
    io::stdin().read_line(&mut s);
    let c: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // println!("n, m: {} {}", n, m);
    // println!("c {:?}", c);

    // dp[i][j]の定義: コインの枚数合計。インデックスi-1までのコインを使って、j円を払うとき
    let mut dp = vec![vec![INF; n+1]; m+1];
    for i in 0..m {
        dp[i][0] = 0;
    }
    for i in 0..m {
        for j in 0..n+1 {
            // c[i]円のコインを追加できないとき
            if j < c[i] {
                // 遷移は1パターン
                dp[i+1][j] = dp[i][j];
            }
            // c[i]円のコインを追加できないとき
            else {
                // 遷移は3パターン
                dp[i+1][j] = min(dp[i][j - c[i]] + 1, dp[i][j]);
                dp[i+1][j] = min(dp[i+1][j - c[i]] + 1, dp[i+1][j]);
            }
        }
    }
    // // dpテーブルの可視化
    // for i in 0..m+1 {
    //     println!("{:?}", dp[i]);        
    // }
    // i=m-1までのコイン(全種類のコイン0~m-1)で、n円払えるとき、コインの最小枚数
    println!("{}", dp[m][n]);

}