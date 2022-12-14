use std::io;


fn main() {
    let MAX_N: usize = 45;
    let mut F: Vec<usize> = vec![0; MAX_N];
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("");
    let n: usize = n.trim().parse().expect("");
    F[0] = 1;
    F[1] = 1;
    let answer = fib(n, &mut F);
    println!("{}", answer);
}


fn fib(num: usize, F: &mut Vec<usize>) -> usize{
    if F[num] == 0 {F[num] = fib(num - 1, F) + fib(num - 2, F);}
    return F[num]
}