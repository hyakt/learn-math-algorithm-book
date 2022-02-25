use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ay

fn main() {
    input! {
        n: usize,
    };

    if n % 4 == 0 {
        println!("6")
    }
    if n % 4 == 1 {
        println!("2")
    }
    if n % 4 == 2 {
        println!("4")
    }
    if n % 4 == 3 {
        println!("8")
    }
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c059";

    #[test]
    fn sample_c059_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"10")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "4\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn sample_c059_2() {
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
    fn sample_c059_3() {
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
