use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_z

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0.0;

    for i in 1..=n {
        ans += n as f64 / i as f64;
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c026";

    #[test]
    fn sample_c026_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"11.416666666666666
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
