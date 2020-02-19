use std::panic;
use std::panic::RefUnwindSafe;
use std::panic::UnwindSafe;

pub fn build_test_object(
    correct: Vec<bool>,
    points: f32,
    name: &str,
    ordinals: Vec<i32>,
    output: &str,
    tags: Vec<&str>,
    visibility: Visibility,
) -> TestReport {
    let len = correct.len();
    let correct: i32 = correct.into_iter().map(i32::from).sum();
    let score: f32 = points * (correct as f32 / len as f32);
    let number: String = ordinals
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(".");
    TestReport {
        score: score.to_string(),
        max_score: points,
        name: name.to_string(),
        number: number,
        output: Some(output.to_string()),
        tags: Some(tags.into_iter().map(String::from).collect()),
        visibility: Some(visibility),
    }
}

#[allow(dead_code)]
pub fn example_report() -> Report {
    Report {
        score: 44.0.to_string(),
        execution_time: 136.0,
        output: String::from("Text relevant to the entire submission"),
        stdout_visibility: Some(Visibility::Visible),
        tests: vec![build_test_object(
            vec![true, false],
            2.0,
            "Ethan",
            vec![1, 1],
            "Giant multiline string that will be placed in a <pre> tag and collapsed by default",
            vec!["tag1", "tag2"],
            Visibility::Visible,
        )],
    }
}

#[allow(dead_code)]
pub fn compare_outputs<F, I: Copy, O: Eq>(f1: F, f2: F, x: I) -> bool
where
    F: Fn(I) -> O,
{
    f1(x) == f2(x)
}

#[allow(dead_code)]
pub fn compare_results<F, I: Copy, O: Eq, E: Eq>(f1: F, f2: F, x: I) -> bool
where
    F: Fn(I) -> Result<O, E>,
{
    match (f1(x), f2(x)) {
        (Ok(y1), Ok(y2)) => y1 == y2,
        (Err(y1), Err(y2)) => y1 == y2,
        _ => false,
    }
}

#[allow(dead_code)]
pub fn compare_outputs_unwind<
    F: RefUnwindSafe,
    I: Copy + RefUnwindSafe,
    O: Eq + UnwindSafe,
    E: Eq,
>(
    f1: F,
    f2: F,
    x: I,
) -> bool
where
    F: Fn(I) -> Result<O, E>,
{
    let g1 = || panic::catch_unwind(|| f1(x));
    let g2 = || panic::catch_unwind(|| f2(x));
    match (g1(), g2()) {
        (Ok(y1), Ok(y2)) => y1 == y2,
        (Err(_), Err(_)) => true,
        _ => false,
    }
}
#[allow(dead_code)]
struct TestResult {
    name: String,
    passed: bool,
    points: f32,
}

trait UnitTest<X>
where
    Standard: Distribution<X>,
{
    fn inputs(num_samples: usize) -> Vec<X> {
        return Standard
            .sample_iter(&mut thread_rng())
            .take(num_samples)
            .collect();
    }
    fn number_string(ordinals: Vec<i32>) -> String {
        return ordinals
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(".");
    }
    fn total_correct<O: Eq>(
        &self,
        num_samples: usize,
        f1: &dyn TestFunction<X, O>,
        f2: &dyn TestFunction<X, O>,
    ) -> i32 {
        return Self::inputs(num_samples)
            .into_iter()
            .map(|x| (f1.apply(&x) == f2.apply(&x)) as i32)
            .sum();
    }

    fn report<O: Eq>(
        &self,
        f1: &dyn TestFunction<X, O>,
        f2: &dyn TestFunction<X, O>,
        num_samples: usize,
        max_score: f32,
        name: String,                   // how do we get this?
        number: i32,                    // how do we build his?
        description: Option<String>,    // make optional
        tags: Option<Vec<String>>,      // make optional
        visibility: Option<Visibility>, // make optional
    ) -> TestReport {
        let total_correct = self.total_correct(num_samples, f1, f2);
        let score = max_score * (total_correct as f32 / num_samples as f32);
        return TestReport {
            score: score.to_string(),
            max_score: max_score,
            name: name,
            number: number.to_string(),
            output: description,
            tags: tags,
            visibility: visibility,
        };
    }
}
