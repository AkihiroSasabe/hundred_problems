use proconio::input;
use itertools::Itertools;

// 階乗(n!)を返す関数
fn factorial(n: isize) -> isize {
    if n == 0 {
        return 1
    } else {
        return n * factorial(n-1)
    }
}

// 街の距離を計算する関数
fn calc_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    (((x1 as isize - x2 as isize).pow(2) +  (y1 as isize - y2 as isize).pow(2)) as f64).powf(0.5)
}

fn main() {
    input! {
        n: usize,
        towns: [[isize; 2]; n]
    }

    // println!("{}", n);
    // println!("{:?}", towns);
    
    let mut total_distance: f64 = 0.0;
    // permの中には0..nを並び替えた組み合わせのベクターが1つ格納されている。
    for perm in (0..n).permutations(n) {
        for i in 0..(n-1) {
            total_distance += calc_distance(towns[perm[i]][0], towns[perm[i]][1], towns[perm[i+1]][0], towns[perm[i+1]][1]);
        }
    }

    let mut average_distance = total_distance / factorial(n as isize) as f64;
    println!("{}", average_distance);

}