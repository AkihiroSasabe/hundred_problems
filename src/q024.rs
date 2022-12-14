use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("入力エラー");
    // trim()は改行\nや文字列頭と末尾の空白などの削除
    // parse()は文字列を数値に変換
    let n: usize = n.trim().parse().expect("入力エラー");

    // グラフの入力
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("入力エラー");
        // collect()はベクトルに変換
        let s: Vec<&str> = s.split_whitespace().collect();
        // into_iter()は所有権を移動してイテレータを返す。for文のデフォルト
        let s: Vec<usize> = s.into_iter().map(|j| ((j.parse::<isize>().expect("変換エラー") - 1) as usize)).collect();
        // スライスの前には&をつける。to_veC()でスライスをベクトルに変換可能
        graph[s[0]] = (&s[2..]).to_vec();
    }
    // println!("{:?}", graph);

    // 発見時刻(discovery)
    let mut d = vec![0_usize; n];
    // 調査完了時刻(finish)
    let mut f = vec![1_usize; n];
    // 現在時刻
    let mut time = 1_usize;
    
    for v in 0..n {
        if d[v] != 0 {continue}
        dfs(v, &mut time, &mut graph, &mut d, &mut f);
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, d[i], f[i]);
    }

}

// Depth-First Search
fn dfs(v: usize, time: &mut usize, graph: & Vec<Vec<usize>>, d_time: &mut Vec<usize>, f_time: &mut Vec<usize>) {
    // 発見(discovery)
    d_time[v] = *time;
    // println!("time: {}, discovery: {}", time, v+1);
    *time += 1;
    
    for adjacent in graph[v].iter() {
        if d_time[*adjacent] != 0 {continue}
        dfs(*adjacent, time, graph, d_time, f_time);
    }
    // 隣接した奴がないとき調査完了(finish)
    f_time[v] = *time;
    // println!("time: {}, finish: {}", time, v+1);
    *time += 1;

}
