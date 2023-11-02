use super::Problem;

/// イテレータのコレクト
pub struct Problem1;

impl Problem for Problem1 {
    type TestCase = TestCase;
    type AnswerFn = fn(Box<dyn Iterator<Item = i32>>, usize) -> (Vec<i32>, Vec<i32>);
    fn test_case(case: Self::TestCase, answer: Self::AnswerFn) -> bool {
        answer(case.iter, case.n) == case.answer
    }
    fn make_cases() -> Vec<Self::TestCase> {
        vec![
            TestCase {
                iter: Box::new(1..=10),
                n: 5,
                answer: (vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]),
            },
            TestCase {
                iter: Box::new([917, 325, 113, 576, 812, 367].into_iter()),
                n: 1,
                answer: (vec![917], vec![325, 113, 576, 812, 367]),
            },
            TestCase {
                iter: Box::new((-6..=6).cycle().take(99)),
                n: 49,
                answer: (
                    (-6..=6).cycle().take(49).collect(),
                    (4..=6).chain((-6..=6).cycle().take(47)).collect(),
                ),
            },
            TestCase {
                iter: Box::new(std::iter::once(1000000000)),
                n: 1,
                answer: (vec![1000000000], vec![]),
            },
        ]
    }
}

pub struct TestCase {
    iter: Box<dyn Iterator<Item = i32>>,
    n: usize,
    answer: (Vec<i32>, Vec<i32>),
}
