use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        // ラッキーナンバーの桁数
        n: usize,
        // ラッキーナンバー
        // Vec<char>で受け取る(ベクトルに1文字ずつ格納されている)
        s: Chars,
    }
 
    // Vec<char> -> Vec<usize>の変換(48を引くことに注意)
    let s_usize: Vec<usize> = s.into_iter().map(|i| (i as usize - 48)).collect();

    let mut count = 0;
    for i in 0..10{
        for j in 0..10 {
            for k in 0..10 {
                let num: Vec<usize> = vec![i, j, k];
                let mut index = 0;
                for x in 0..n {
                    if num[index] == s_usize[x] {
                        index+=1;
                        if index == 3 {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}