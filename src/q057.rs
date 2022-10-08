use std::cmp::Ordering;

fn main() {

    // 比較時にはLess, Greater, Equalのいずれかが返される
    let x = 1;
    let y = 2;
    let z = 1;
    println!("{:?}", x.cmp(&y)); // Less
    println!("{:?}", y.cmp(&x)); // Greater
    println!("{:?}", x.cmp(&z)); // Equal


    // then_with()メソッドの定義
    // then_with()の直前がOrdering::Equalでない場合は、then_with()の直前のものを返す。
    // then_with()の直前がOrdering::Equalである場合は、then_with()の中の処理を返す。

    // then_with()の直前が、Ordering::Equalなので、then_with()の中のOrdering::Lessを返す
    let result = Ordering::Equal.then_with(|| Ordering::Less);
    println!("{:?}", result); // Less

    // then_withの直前が、Ordering::Equalではない(Ordering::Less)なので、then_with()の直前のOrdering::Lessを返す
    let result = Ordering::Less.then_with(|| Ordering::Equal);
    println!("{:?}", result); // Less


    // 下記のタプルの大小比較を考える
    let x = (1, 2, 7);
    let y = (1, 5, 3);

    // 0個目の要素を比較する場合: 
    // x.0 = 1, y.0 = 1 => x.0 = y.1なのでOrdering::Equalを返す
    let result = x.0.cmp(&y.0);
    println!("{:?}", result); // Equal

    // 1個目の要素を比較する場合: 
    // x.1 = 2, y.1 = 5 => x.1 < y.1なのでOrdering::Lessを返す
    let result = x.0.cmp(&y.0);
    println!("{:?}", result); // Less

    // タプルの比較をする。
    // 0個目の要素から比較を開始し、もし0個目の要素同士が等しければ1個目の要素を比較し、1個目の要素も等しければまた次の要素を比較していく
    // 今回は0個目の要素が等しく、1個目の要素が比較対象より小さいので、Lessを返す
    let result = x.0.cmp(&y.0).then_with(|| x.1.cmp(&y.1)).then_with(|| x.2.cmp(&y.2));
    println!("{:?}", result); // Less

}