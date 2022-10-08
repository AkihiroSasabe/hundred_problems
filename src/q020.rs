use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n],
        mut c: [usize; n],
    }

    // 2分探索するときは、必ずソートする
    a.sort();
    c.sort();

    // GitHubに落ちていた、lower_bound()メソッドを使った方法(楽) 
    // 参考: https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs
    // let mut total = 0;
    // for bi in b {
    //     let a_num = a.lower_bound(&bi);
    //     let c_num = n - c.upper_bound(&bi);
    //     total += a_num * c_num;
    // }
    // println!("{}", total);
    

    let mut total_num = 0;
    for bi in b {
        let mut a_min = 0;
        let mut a_max = n;
        while (a_max != a_min) {
            let mid = (a_min + a_max) / 2;
            if a[mid] < bi {
                a_min = mid + 1;
            } 
            else if bi <= a[mid] {
                a_max = mid;
            }
        }

        let mut c_min = 0;
        let mut c_max = n;
        while (c_max != c_min) {
            let mid = (c_min + c_max) / 2;
            if c[mid] <= bi {
                c_min = mid + 1;
            } 
            else if bi < c[mid] {
                c_max = mid;
            }
        }
        let a_num = a_min;
        let c_num = (n as isize - c_min as isize) as usize;
        total_num += a_num * c_num;
    }
    println!("{}", total_num);
}

// lower_bound=Key★以★上★のインデックス、
// upper_bound=Key★よ★り★大きいインデックス
// sorted_list.lower_bound(&x)は、x以上となる最小のインデックスを返すが、x超えがリスト内に無いときは、sorted_list.len()を返すので注意
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
