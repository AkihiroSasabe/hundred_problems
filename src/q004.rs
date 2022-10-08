use proconio::input;

fn main() {
    input! {
        // 生徒数 (要素にアクセスするとき、添字をusizeにキャストしないといけないので、ベクトルの要素の型はi32ではなくusizeがおすすめ。64bitのPCならu64)
        // usizeの最大値: 2 ** 64 -1 = 18446744073709551615 = 10 ** 19.26..
        n: usize,
        // 曲数
        m: usize,
        // 点数表 (m個の要素が格納されたベクトルがn個 = 縦n個, 横m個)
        a: [[usize; m]; n]
    }

    let mut point = 0;
    let mut max_point = 0;
    let mut max_i=0;
    let mut max_j=1;
    for i in 0..(m-1) {
        for j in i+1..m {
            for x in 0..n {
                if a[x][i] <= a[x][j] {
                    point += a[x][j];
                } else {
                    point += a[x][i];
                }
            }
            if max_point < point {
                max_point = point;
                max_i = i;
                max_j = j;
            }
            point = 0;
        }
    }
    println!("{}", max_point);
    // println!("{max_i}, {max_j}");
}