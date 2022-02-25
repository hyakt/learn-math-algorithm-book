use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ax

fn main() {
    input! {
        n: isize,
        x: isize,
        y: isize,
    };

    if (x.abs() + y.abs()) <= n && n % 2 == 0 && (x + y) % 2 == 0 || n % 2 == 1 && (x + y) % 2 == 1
    {
        println!("Yes");
        return;
    }

    println!("No");
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c058";

    #[test]
    fn sample_c058_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"10 2 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c058_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"
9 3 1
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "No\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c058_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"101 100 1")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty());
    }
}
