use proconio::input;

fn main() {
    input! {
        // 人
        n: usize,
        // 関係
        m: usize,
        pairs: [[usize; 2]; m]
    }

    // n人の議員が、それぞれ誰と友達かをまとめたグラフ
    let mut friend_graph = vec![vec![0; n]; n];
    for pair in pairs {
        friend_graph[pair[0] - 1][pair[1] -1] = 1;
        friend_graph[pair[1] - 1][pair[0] - 1] = 1;
    }
    // println!("friend_graph: {:?}", friend_graph);


    // 派閥のパターンはn人が属しているか、属していなかの2^n通り
    // そのパターンを全探索して、そのパターンを満たす友人の組み合わせをチェック
    let mut member_list: Vec<usize> = vec![];
    let mut pattern_exist = true;
    let mut member_num = 0;
    let mut member_num_max = 0;
    for pattern in 0..(1<<n) {
        // println!("{:05b}", pattern);
        member_num = 0;
        member_list = vec![];
        for i in 0..n {
            // i桁目のビットが1なら、i番目のメンバーをmember_listに加入
            if pattern & 1 << i != 0 {
                member_list.push(i);
                member_num += 1;
            }
        }
        // println!("{:?}", member_list);

        // member_listが実現可能かチェック
        pattern_exist = true;
        for i in 0..member_list.len() {
            if !pattern_exist {
                break;
            }
            for j in i+1..member_list.len() {
                if friend_graph[member_list[i]][member_list[j]] != 1 {
                    pattern_exist = false;
                    break;
                }
            }
        }
        if !pattern_exist {
            // println!("The pattern {:05b} cannot exist.", pattern);
            continue
        }

        // 最大メンバー数を更新するか
        if member_num_max < member_num {
            member_num_max = member_num;
        }
    }

    println!("{}", member_num_max);
        
}