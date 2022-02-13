use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_y

fn main() {
    input! {
        n: usize,
        a: [f64; n],
        b: [f64; n],
    };

    let mut ans = 0.0;

    for i in 0..n {
        ans += (a[i] * (2.0 / 6.0)) + (b[i] * (4.0 / 6.0));
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c025";

    #[test]
    fn sample_c025_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
3 1 4 1 5
9 2 6 5 3
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"21.333333333333336
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
