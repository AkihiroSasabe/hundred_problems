// use proconio::input;  // AIZU ONLINE JUDGEでは使えない
// use itertools::Itertools; // AIZU ONLINE JUDGEでは使えない
use std::io;

// 0-nの順列の組み合わせを全て求める関数
fn permutation(n: usize, digit: usize, used_list: &mut Vec<usize>, perm: &mut Vec<isize>, perms: &mut Vec<Vec<isize>>) {
    // digit桁目に格納する値を決める
    for i in 0..n {
        // 既に使われている値はdigit桁目に入れない
        if used_list.contains(&i) {
            continue
        }
        perm[digit] = i as isize;

        // n桁全て揃ったら組み合わせを格納して終了
        if digit == n-1 {
            perms.push(perm.clone());
        }

        // digit桁目(digit !=n-1)であれば、(digit+1)桁目に格納する値を決める
        else {
            used_list.insert(digit, i);
            permutation(n, digit + 1, used_list, perm, perms);
            used_list.remove(digit);
        }
    }
}

// 置いた点の斜め方向の座標を全て取得する関数
fn get_cross_yx(y: usize, x: usize) -> Vec<Vec<isize>> {
    // 行
    let mut i = y as isize;

    // 列
    let mut j = x as isize;
    let mut cross = vec![];
    
    // 左上
    loop {
        i -= 1;
        j -= 1;
        if (j < 0) || (i < 0) || ((7 < j) || (7 < i)) {
            break;
        }
        cross.push(vec![i, j]);
    }
    i = y as isize;
    j = x as isize;
    // 左下
    loop {
        i += 1;
        j -= 1;
        if (j < 0) || (i < 0) || ((7 < j) || (7 < i)) {
            break;
        }
        cross.push(vec![i, j]);
    }

    i = y as isize;
    j = x as isize;
    // 右上
    loop {
        i -= 1;
        j += 1;
        if (j < 0) || (i < 0) || ((7 < j) || (7 < i)) {
            break;
        }
        cross.push(vec![i, j]);

    }

    i = y as isize;
    j = x as isize;
    // 右下
    loop {
        i += 1;
        j += 1;
        if (j < 0) || (i < 0) || ((7 < j) || (7 < i)) {
            break;
        }
        cross.push(vec![i, j]);
    }

    return cross;
}

fn main() {
    // 標準入力: AOJではproconioが使えない
    // input! {
    //     k: usize,
    //     rc: [[usize; 2]; k]
    // }

    // 標準入力: proconio以外でやる
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("入力エラー");
    let k: usize = k.trim().parse().expect("変換エラー");

    let mut rc = vec![];
    for i in 0..k {
        let mut rc_i = String::new();
        io::stdin().read_line(&mut rc_i).expect("入力エラー");
        let rc_i: Vec<&str> = rc_i.split_whitespace().collect();
        let rc_i: Vec<usize> = rc_i.into_iter().map(|j| (j.parse().expect("変換エラー"))).collect();
        rc.push(rc_i)
    }

    
    // 0-7のうち、既に確定しているk個の列番号を抜いた、残り8-k個の集まり(=left)を求める
    let mut left = vec![];
    let mut add_flag = true;
    for i in 0..8 {
        for pair in rc.iter() {
            if i == pair[1] {
                add_flag = false;
            }
        }
        if add_flag {
            left.push(i)
        }
        add_flag = true;
    }


    // AOJはpermutationsが使えないので下記7行を追加
    let mut perm_index: Vec<isize> = vec![-1; left.len()];
    let mut perm_indexes: Vec<Vec<isize>> = vec![];
    let mut used_list = vec![];
    permutation(left.len(), 0, &mut used_list, &mut perm_index, &mut perm_indexes);
    if k == 8 {
        perm_indexes = vec![vec![0,1,2,3,4,5,6,7]];
    }

    // leftの順列を求める
    // AOJはpermutationsが使えないので下記1行をコメントアウトし、さらに下に7行を追加
    // for perm in left.iter().permutations(8-k) {
    for index in perm_indexes {
        let mut perm = vec![0; left.len()];
        if left.len() != 0 {
            for (e, i) in index.iter().enumerate() {
                perm[*i as usize] = left[e as usize].clone();
            }
        }

        let mut queen_rc_list = vec![];
        let mut j = 0;
        // 各行(i)毎に、どの列にQueenがいるかのベクトルを作成
        for i in 0..8 {
            let mut flag = false;
            // 標準入力で与えられたQueen
            for pair in rc.iter() {
                // 標準入力で与えられたQueenがi行目にいる場合
                if i == pair[0] {
                    queen_rc_list.push(vec![pair[0], pair[1]]);
                    flag = true;
                    break;
                } 
            }
            // 標準入力で与えられたQueenがi行目にいない場合
            if !flag {
                queen_rc_list.push(vec![i, perm[j].clone()]);
                j += 1;
            }
        }

        // クイーンの配置パターンが、条件（どのクイーンも他のクイーンを襲撃できない）を満たしているか確認
        let mut check_result = true;
        for q in 0..queen_rc_list.len() {
            let cross_yx = get_cross_yx(queen_rc_list[q][0], queen_rc_list[q][1]);
            for r in (q+1)..(queen_rc_list.len()) {
                for cross_yx_one in cross_yx.iter() {
                    if *cross_yx_one == vec![queen_rc_list[r][0] as isize, queen_rc_list[r][1] as isize] {
                        check_result = false;
                        break;
                    }
                }
            }

            if !check_result {
                break;
            }             
        }
        if !check_result {
            continue;
        }
        
        // 条件を満たしているときだけ下記をprint
        // 各行についてのループ
        for queen in queen_rc_list {
            // 各列(i列目)についてのループ
            for i in 0..8 {
                if i == queen[1] {
                    print!("Q");
                } else {
                    print!(".");
                }
                if i == 7 {
                    println!("");
                }
            }
        }
    }
}

