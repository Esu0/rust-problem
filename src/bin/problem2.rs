use rust_problem::problems::{problem2::Problem2, Problem};
fn main() {
    Problem2::test(answer);
}

/// 問題2: イテレータのコレクト2 
/// 
/// イテレータと要素数の列N[[0]], N[[1]], ... N[[m-1]]が与えられる。
/// 0 <= i <= m - 1について、
/// 添え字iの要素が、`iter`が(N[[0]]+N[[1]]+...+N[[i-1]])番目から始めてN[[i]]個の要素を集めたコレクションとなるような
/// コレクションを返す関数を実装せよ。 
/// 
/// 制約:
/// * `iter`が持つ要素の数はN[[0]]+N[[1]]+...+N[[m-1]]個以上でかつ、有限である。
/// 
/// 目標コード量: 65文字(空白、改行除く)
#[allow(unused_variables, unused_mut)]
fn answer(mut iter: Box<dyn Iterator<Item = i32>>, mut n: Vec<usize>) -> Vec<Vec<i32>> {
    unimplemented!("implement answer function")
}
