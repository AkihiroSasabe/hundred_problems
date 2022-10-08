use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut count: i32 = 0;

    // 105未満で条件「奇数かつ約数が 8 個」を満たす整数は0個
    if n < 105 {
        println!("{}", count);
        return
    }

    // 105は条件を満たす
    count += 1;

    // 106から数える
    for i in 106..n+1 {
        // 偶数は飛ばす
        if i % 2 == 0 {
            continue;
        }
        
        // 約数の数を数える
        let mut dev_num: i32 = 0;
        for devisor in 1..(i+1) {
            if i % devisor == 0 {
                dev_num += 1;
            }
        }
        if dev_num == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}