use proconio::input;

fn main() {
    input! {
        // スイッチの数
        n: usize,
        // 電球の数
        m: usize,
    }

    // 角電球毎に接続されている全スイッチを格納したリスト
    let mut s = vec![];
    for i in 0..m {
        input! {
            k_i: usize,
            s_i: [usize; k_i]
        }
        s.push(s_i);
    }

    // 角電球が光る偶奇の条件
    input! {
        p: [usize; m]
    }
    // println!("s: {:?}", s);
    // println!("p: {:?}", p);

    let mut pattern_num = 0;
    let mut succsess = true;
    // n個あるスイッチのon/offを全探索する(2^n通り)
    for pattern in 0..(1 << n) {
        // println!("pattern: {:04b}", pattern);
        let mut succsess = true;
        // 各電球が全て光るか判定(=success)し、全て光る場合のパターンの総数pattern_numを数える
        for bulb in 0..m {
            let mut on_num = 0;
            // 各電球に繋がれたスイッチの点灯している個数を確認
            for bulb_switch in &s[bulb] {
                // ビット演算ANDと左シフト
                // patternはnビットの数。2ビットの各桁の値(0/1)が電球の状態(OFF/ON)に対応
                if (pattern & (1 << (bulb_switch - 1))) == (1 << (bulb_switch - 1)) {
                    on_num += 1;
                }       
            }
            if on_num % 2 != p[bulb] {
                succsess = false;
                break;
            }
        }

        if succsess {
            // println!("succsess pattern: {:04b}", pattern);
            pattern_num += 1;
        }
        
    }
    println!("{}", pattern_num);
}