use super::Problem;

/// イテレータのコレクト2
pub struct Problem2;

impl Problem for Problem2 {
    type TestCase = TestCase;
    type AnswerFn = fn(Box<dyn Iterator<Item = i32>>, Vec<usize>) -> Vec<Vec<i32>>;
    fn test_case(case: Self::TestCase, answer: Self::AnswerFn) -> bool {
        answer(case.iter, case.n) == case.answer
    }

    fn make_cases() -> Vec<Self::TestCase> {
        vec![
            TestCase {
                iter: Box::new(1..=10),
                n: vec![3, 5, 2],
                answer: vec![vec![1, 2, 3], vec![4, 5, 6, 7, 8], vec![9, 10]],
            },
            TestCase {
                iter: Box::new(-100..100),
                n: vec![4, 3, 6, 5, 5, 10, 13, 35, 1, 4, 15],
                answer: vec![
                    (-100..-96).collect(),
                    (-96..-93).collect(),
                    (-93..-87).collect(),
                    (-87..-82).collect(),
                    (-82..-77).collect(),
                    (-77..-67).collect(),
                    (-67..-54).collect(),
                    (-54..-19).collect(),
                    (-19..-18).collect(),
                    (-18..-14).collect(),
                    (-14..1).collect(),
                ],
            },
            TestCase {
                iter: Box::new(std::iter::once(-387912235)),
                n: vec![0, 0, 0, 1, 0, 0, 0],
                answer: vec![
                    vec![],
                    vec![],
                    vec![],
                    vec![-387912235],
                    vec![],
                    vec![],
                    vec![],
                ],
            },
        ]
    }
}

pub struct TestCase {
    iter: Box<dyn Iterator<Item = i32>>,
    n: Vec<usize>,
    answer: Vec<Vec<i32>>,
}
