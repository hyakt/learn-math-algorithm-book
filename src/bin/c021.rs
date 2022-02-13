use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_u

fn combination(n: usize, r: usize) -> usize {
    let mut dividend = 1;
    let mut divisor = 1;
    for i in 0..r {
        dividend *= n - i;
        divisor *= i + 1;
    }
    dividend / divisor
}

fn main() {
    input! {
        n: usize,
        r: usize,
    };

    println!("{}", combination(n, r));
}

#[cfg(test)]
mod tests {
    use crate::*;
    use cli_test_dir::*;
    const BIN: &'static str = "./c021";

    #[test]
    fn check_combination() {
        assert_eq!(combination(6, 2), 15);
    }

    #[test]
    fn c0211() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"15
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
