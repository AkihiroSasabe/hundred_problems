use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    // println!("p is {:?}", p);
    // println!("q is {:?}", q);
    let mut count = 0;
    let mut a = 0;
    let mut b = 0;
    // permには1~nの数字を並べ替えてできるn個の数字の組のいずれかが格納されている
    for perm in (1..n+1).permutations(n) {
        // println!("{:?}", perm);
        count += 1;
        if perm == p {
            a = count;
            // println!("a");
        }
        if perm == q {
            b = count;
            // println!("b");
        }
    }

    let answer: isize;
    if (a >= b) {
        answer = a as isize - b as isize;
    } else {
        answer = b as isize - a as isize;
    }

    println!("{}", answer);   
}