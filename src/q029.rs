use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;


fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        // Vec<Vec<char>>型
        mut maze: [Chars; r]
    }
    maze[gy - 1][gx - 1] = 'g';
    // for m in &maze {
    //     println!("{:?}", m);
    // }
    // println!("{:?}", maze);    
    let mut todo = VecDeque::<Vec<usize>>::new();
    todo.push_back(vec![sy - 1, sx - 1]);
    let mut distance = vec![vec![-1;c];r];
    distance[sy - 1][sx - 1] = 0;
    
    while todo.len() != 0 {
        let pixel = todo.pop_front().unwrap();
        let y = pixel[0];
        let x = pixel[1];
        // 移動方向(下右上左)
        let orients: [[isize; 2];4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        
        // for (next_y, next_x) in &maze[y, x] {
        for orient in &orients {
            let next_y = (y as isize + orient[0]) as usize;
            let next_x = (x as isize + orient[1]) as usize;
            // println!("y, x, {} {}", y, x);
            // println!("next_y, next_x, {} {}", next_y, next_x);
            // 既に通っていたらスキップ
            if distance[next_y][next_x] != -1 {continue};
            // 壁だったらスキップ
            if maze[next_y][next_x] == '#' {continue};
            // キューに追加
            todo.push_back(vec![next_y, next_x]);
            distance[next_y][next_x] = distance[y][x] + 1;
            // ゴールだったらスキップ
            if maze[next_y][next_x] == 'g' {break};
        }

    }
    println!("{}", distance[gy - 1][gx - 1]);

}