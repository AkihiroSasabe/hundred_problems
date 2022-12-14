use std::io;

fn main() {

    // 標準入力: proconio以外でやる
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("入力エラー");
    let n: usize = n.trim().parse().expect("変換エラー");

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("入力エラー");
    let s: Vec<&str> = s.split_whitespace().collect();
    let s: Vec<usize> = s.into_iter().map(|j| (j.parse().expect("変換エラー"))).collect();


    let mut q = String::new();
    io::stdin().read_line(&mut q).expect("入力エラー");
    let q: usize = q.trim().parse().expect("変換エラー");

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("入力エラー");
    let t: Vec<&str> = t.split_whitespace().collect();
    let t: Vec<usize> = t.into_iter().map(|j| (j.parse().expect("変換エラー"))).collect();

    // println!("{}", n);
    // println!("{:?}", s);
    // println!("{}", q);
    // println!("{:?}", t);

    let mut count = 0;
    for val in t {
        let mut min = 0;
        let mut max = s.len() - 1;
        while (min <= max) {
            let mut mid = (min + max) / 2;
            // println!("min: {}, max: {}, mid:{}, s[mid]: {}", min, max, mid, s[mid]);

            if val == s[mid]{
                count += 1;
                // println!("exist val: {}", val);
                break
            }
            else if val < s[mid] {
                max = mid - 1;
            } 
            else {
                min = mid + 1;
            }

        }
    }

    println!("{}", count);

    // println!("{}", ((1 + 2) / 2) as usize);

}


// fn sort(list: &Vec<usize>) -> Vec<usize>{
//     let mut sorted_list: Vec<usize> = vec![];

//     // for i in list {
//     for i in list {
//         for j in 0..sorted_list.len() {
//             if *i < sorted_list[j] {
//                 sorted_list.insert(j, i.clone());
//                 println!("insert 1");
//                 break;
//             }

//             if j == sorted_list.len() - 1 {
//                 sorted_list.push(i.clone());
//                 println!("insert 2");
//             }
//         }

//         if sorted_list.len() == 0 {
//             sorted_list.push(i.clone());
//             println!("insert 3");
//         }
//     }
//     return sorted_list
// }