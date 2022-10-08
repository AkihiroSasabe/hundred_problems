use proconio::input;


fn main() {
    input! {
        // 全長
        d: usize,

        // 店舗の個数
        n: usize,

        // 注文の個数
        m: usize,

        // 本店以外の店舗の位置
        mut di: [usize; n-1],

        // 宅配先の位置
        mut k: [usize; m]
    }

    di.sort();
    k.sort();

    di.insert(0, 0);
    di.push(d);

    // // 入力の確認
    // println!("d: {}", d);
    // println!("n: {}", n);
    // println!("m: {}", m);
    // println!("di: {:?}", di);
    // println!("k: {:?}", k);


    let mut total_distance = 0;
    for dist in k {
        let mut min = 0;
        let mut max = n;
        while (max as isize - min as isize > 1) {
            let mid = (min + max) / 2;
            // println!("min: {}, max: {}, mid:{}, di[mid]: {}, dist: {}", min, max, mid, di[mid], dist);
            
            if dist < di[mid] {
                max = mid;
            }
            else {
                min = mid;
            }
        }
        if (di[max] - dist) > (dist - di[min]) {
            total_distance += dist - di[min];
        } else {
            total_distance += di[max] - dist;
        }
    }
    println!("{}", total_distance);
}