// use proconio::input;
use std::io;

fn main() {
    // 標準入力: AOJではproconioが使えない
    // input! {
    //     n: usize,
    //     a: [usize; n],
    //     q: usize,
    //     m: [usize; q],
    // }

    // 標準入力: proconio以外でやる
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("入力エラー");
    let n: usize = n.trim().parse().expect("変換エラー");
    // println!("n is {}", n);

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("入力エラー");
    let a: Vec<&str> = a.split_whitespace().collect();
    let a: Vec<usize> = a.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    // println!("a is {:?}", a);

    let mut q = String::new();
    io::stdin().read_line(&mut q).expect("入力エラー");
    let q: usize = q.trim().parse().expect("変換エラー");
    // println!("q is {}", q);

    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("入力エラー");
    let m: Vec<&str> = m.split_whitespace().collect();
    let m: Vec<usize> = m.into_iter().map(|i| (i.parse().expect("変換エラー"))).collect();
    // println!("m is {:?}", m);

    // 答えを格納したベクトル
    let mut ans = vec!("no"; q);

    // 2 ** n 通り
    for bit in 0..(1_usize << n) {
        let mut sum = 0;
        for i in 0..n {
            // ビット演算
            if (bit & (1_usize << i) == (1_usize << i)) {
                sum += a[i]; 
            }
        }
        for j in 0..q {
            if m[j] == sum {
                ans[j] = "yes";
            }
        }
    }
    for j in 0..q {
        println!("{}", ans[j]);
    }
}