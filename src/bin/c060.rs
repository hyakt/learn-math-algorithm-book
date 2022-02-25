use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_az

fn main() {
    input! {
        n: usize,
    };

    if n % 4 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c060";

    #[test]
    fn sample_c060_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"4")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Second\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn sample_c060_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn sample_c060_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "\n");
        assert!(output.stderr_str().is_empty());
    }
}
