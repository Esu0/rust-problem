use rust_problem::problems::{problem1::Problem1, Problem};

fn main() {
    Problem1::test(answer);
}

/// 問題1: イテレータのコレクト
///
/// `iter`が返す要素のうち、`n`個目までの要素のコレクションと、それ以降の要素のコレクションをタプルにして
/// 返す関数を実装せよ。
///
/// 制約:
/// * `iter`が持つ要素の数は`n`個以上でかつ、有限である。
/// * `n` >= 0
///
/// 目標コード量: 60文字以下(空白、改行除く)
#[allow(unused_variables, unused_mut)]
fn answer(mut iter: Box<dyn Iterator<Item = i32>>, mut n: usize) -> (Vec<i32>, Vec<i32>) {
    unimplemented!("implement answer function")
}
