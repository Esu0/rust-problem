pub mod problem1;
pub mod problem2;

pub trait Problem {
    type TestCase;
    type AnswerFn: Copy;
    fn test_case(case: Self::TestCase, answer: Self::AnswerFn) -> bool;
    fn make_cases() -> Vec<Self::TestCase>;
    fn test(answer: Self::AnswerFn) {
        for (i, case) in Self::make_cases().into_iter().enumerate() {
            if Self::test_case(case, answer) {
                println!("test case {} passed", i + 1);
            } else {
                panic!("test case {} failed", i + 1);
            }
        }
    }
}
