use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ab

fn main() {
    input! {
        n: usize,
    };

    let mut m: Vec<isize> = vec![0; n + 1];

    m[0] = 1;
    m[1] = 1;

    for i in 2..=n {
        m[i] = m[i - 1] + m[i - 2];
    }

    println!("{}", m[n]);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c029";

    #[test]
    fn sample_c029_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"4
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"5
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c029_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"45
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"1836311903
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
