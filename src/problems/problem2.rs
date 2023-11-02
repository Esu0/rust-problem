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
                    (-95..-92).collect(),
                    (-91..-86).collect(),
                    (-85..-80).collect(),
                    (-79..-74).collect(),
                    (-73..-64).collect(),
                    (-63..-50).collect(),
                    (-49..-14).collect(),
                    (-13..-12).collect(),
                    (-11..-7).collect(),
                    (-6..=9).collect(),
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
