use proconio::input;

fn main() {
    input! {
        // A, B, Cの値段とAとBの必要な枚数
        a: i64,
        b: i64,
        c: i64,
        x: i64,
        y: i64,
    }

    // ABの最大購入可能枚数
    let cn_max;

    if x <= y {
        cn_max = y * 2;
    } else {
        cn_max = x * 2;
    }

    let mut min_price: i64 = 2_i64.pow(62) - 1;
    let mut an_min = 0;
    let mut bn_min = 0;
    let mut cn_min = 0;
    for cn in 0..(cn_max+1) {
        if cn % 2 == 1 {continue}
        // 購入枚数        
        let mut an: i64 = x - cn / 2;
        let mut bn: i64 = y - cn / 2;
        if an < 0 {
            an = 0;
        }
        if bn < 0 {
            bn = 0;
        }
        let price = a * an + b * bn + c * cn;
        if price < min_price {
            min_price = price;
            an_min = an;
            bn_min = bn;
            cn_min = cn;
        }
    }
    println!("{}", min_price);
}
