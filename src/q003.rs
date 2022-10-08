use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        // Vec<char>で受け取る(ベクトルに1文字ずつ格納されている)
        s: Chars,
    }

    // ACGTなものの長さを格納していく
    let mut acgt_lens = vec![];

    // 部分文字列の全パターンを網羅する
    for start in 0..s.len(){
        for j in 0..s.len(){
            let end = s.len() - j - 1;
            if start > end {continue}
            // スライスするときは、&でやる
            let s_slice = &s[start..end+1];
            // into_iter()は値を返すイテレータを返す。所有権を移動する
            let s_sub: String = s_slice.into_iter().collect();

            // ACGTか、1文字ずつ確認
            let mut flag_acgt: bool = true;
            // as_str()でString -> &str型に変換、chars()で1文字ずつのイテレータに変換
            for c in s_sub.as_str().chars(){
                if !(c == 'A' || c == 'C' || c == 'G' || c == 'T') {
                    flag_acgt = false
                }
            }

            if flag_acgt {
                acgt_lens.push(s_sub.len());
                // デバッグ用に確認
                // println!("start: {}, end: {}, s_sub: {}, lenght: {}", start, end, s_sub, s_sub.len());
            }
        }
    }

    // 長さが最大のものを求める
    // let max_len = acgt_lens.iter().max().unwrap();
    let max_len = match acgt_lens.iter().max() {
        Some(n) => *n, // 参照外し
        None => 0,
    };
    println!("{}", max_len);
}