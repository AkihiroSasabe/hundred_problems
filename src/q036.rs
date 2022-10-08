use std::io;
use std::cmp::max;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<usize> = s.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    // println!("{:?}", s);
    let n = s[0];
    let w = s[1];
    // println!("{} {}", n, w);

    let mut vs = vec![];
    let mut ws = vec![];
    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<usize> = s.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        vs.push(s[0]);
        ws.push(s[1]);
    }


    // dp[i][W]は、w以下の重さの制約の下、0 ~ i-1のモノを複数選択したときの、価値の総和の最大値
    let mut dp = vec![vec![0; w+1]; n+1];
    for i in 0..n {
        for j in 0..w+1 {
            // i番目の荷物を追加できない場合
            if j < ws[i] {
                dp[i+1][j] = dp[i][j];
            }
            // i番目の荷物を追加できる場合
            else{
                // 3パターンの遷移がある
                // 1: i番目を追加しない
                // 2: i-1番目までで価値をj-ws[i]とし、i番目を最後に1つ追加する
                // 3: i番目までで価値をj-ws[i]とし、i番目を最後に1つ追加する
                dp[i+1][j] = max(dp[i][j-ws[i]] + vs[i], dp[i][j]);
                dp[i+1][j] = max(dp[i+1][j-ws[i]] + vs[i], dp[i+1][j]);
            }

        }
    }
    // dpテーブルの可視化
    // for i in 0..n+1 {
    //     println!("{:?}", dp[i]);        
    // }
    println!("{:?}", dp[n][w]);        
}