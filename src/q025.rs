use std::io;

fn main() {
    let mut island_nums: Vec<usize> = vec![];
    // 各map毎のループ
    loop {
        // w h の取得
        let mut wh = String::new();
        io::stdin().read_line(&mut wh).expect("");
        let wh: Vec<usize> = wh.split_whitespace().into_iter().map(|j| j.parse().expect("")).collect();
        let w: usize = wh[0];
        let h: usize = wh[1];
        if wh.len() == 2 && w == 0 && h == 0 {
            break
        }

        // mapの取得
        let mut mesh: Vec<Vec<usize>> = vec![];
        for _ in 0..h {
            let mut row = String::new();
            io::stdin().read_line(&mut row).expect("");
            let row: Vec<usize> = row.split_whitespace().into_iter().map(|i| i.parse().expect("")).collect();
            mesh.push(row);
        }

        // 島の数え上げ
        let mut island_num = 0;
        let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
        for i in 0..w {
            for j in 0..h {
                // 海ならスキップ
                if mesh[j][i] == 0 {continue}
                // 調査済みの陸ならスキップ
                if seen[j][i] {continue}
                dfs(j, i, &mut mesh, &mut seen, h, w);
                // println!("countup: h w {} {}", j, i);
                island_num += 1;
            }
        }
        island_nums.push(island_num);

    }
    for num in island_nums {
        println!("{}", num);
    }

}

// 1つの陸続きのマスを全て探す再帰関数
fn dfs(h: usize, w: usize, mesh: &mut Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>, height: usize, width: usize) {
    // println!("h w: {} {}", h, w);
    // 0: 海, 1: 陸
    if mesh[h][w] == 0 {return}
    // 陸
    seen[h][w] = true;

    // 8方向の確認
    let y_orient: Vec<isize> = vec![-1, 0, 1];
    let x_orient: Vec<isize> = vec![-1, 0, 1];
    for i in y_orient {
        for j in x_orient.iter() {
            let y = h as isize + i;
            let x = w as isize + j;
            // map外ならスキップ
            if y < 0 || height as isize <= y || x < 0 || width as isize <= x {continue}
            let y = y as usize;
            let x = x as usize;
            
            // 海ならスキップ
            if mesh[y][x] == 0 {continue}
            // 調査済みの陸ならスキップ
            if seen[y][x] {continue}
            dfs(y, x, mesh, seen, height, width);
        }
    }

}