use std::io;
use std::cmp::max;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let q: usize = s.trim().parse().unwrap();
    // println!("{}", q);

    let mut x_list = vec![];
    let mut y_list = vec![];
    for i in 0..q {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let x: Vec<char> = s.trim().chars().collect();
        
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let y: Vec<char> = s.trim().chars().collect();
        x_list.push(x);
        y_list.push(y);    
    }

    for (y, x) in y_list.iter().zip(x_list.iter()) {
        // dp[i][j]の定義: Yのi番目以前の文字列と、Xのj番目以前の文字列でなす最長共通部分列zの長さ
        // dpの初期化
        let mut dp = vec![vec![0; x.len()]; y.len()+1];
        // dpの1行目の計算
        for j in 0..x.len() {
            if y[0] == x[j] {
                for j_or_later in j..x.len() {
                    dp[0][j_or_later] = 1;
                }
                break
            }
        }
        // dpの1列目の計算
        for i in 0..y.len() {
            if x[0] == y[i] {
                for i_or_later in i..y.len() {
                    dp[i_or_later][0] = 1;
                }
                break
            }
        }
        // println!("dp initial state");
        // for i in 0..y.len() {
        //     println!("{:?}", dp[i]);            
        // }

        // dpの1列目と1行目以外の計算
        // 遷移は3種類で、この中で最大になる遷移を選ぶ
        // (1)dp[i-1][j-1] -> dp[i][j]: Yのi番目の文字列と、Xのj番目の文字列が一致して遷移する場合に限る
        // (2)dp[i][j-1] -> dp[i][j]
        // (3)dp[i-1][j] -> dp[i][j]
        // ここでi=0やj=0のとき、i-1やj-1を計算できないので、dpの1列目や1行目を事前に計算した
        for i in 1..y.len() {
            for j in 1..x.len() {
                if y[i] == x[j] {
                    dp[i][j] = max(dp[i-1][j-1] + 1, dp[i][j-1]);
                    dp[i][j] = max(dp[i][j], dp[i-1][j]);
                }
                else {
                    dp[i][j] = max(dp[i][j-1], dp[i-1][j]);
                }
            }
        }
        println!("{}", dp[y.len() - 1][x.len() - 1]);


        // println!("dp final state:");
        // for i in 0..y.len() {
        //     println!("{:?}", dp[i]);            
        // }

    }
    // 入力の確認
    // println!("{:?}", x_list);
    // println!("{:?}", y_list);
}