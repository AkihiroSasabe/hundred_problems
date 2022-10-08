use std::io;

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
            if j < ws[i] {
                // i番目の荷物を追加しない。
                dp[i+1][j] = dp[i][j];
                continue
            }
            // 重さがj未満のとき、i番目の荷物を追加するか否か
            match dp[i][j-ws[i]] + vs[i] < dp[i][j] {
                true => dp[i+1][j] = dp[i][j],
                false => dp[i+1][j] = dp[i][j-ws[i]] + vs[i],
            }
        }
    }
    // // dpテーブルの可視化
    // for i in 0..n+1 {
    //     println!("{:?}", dp[i]);        
    // }
    println!("{:?}", dp[n][w]);        
}