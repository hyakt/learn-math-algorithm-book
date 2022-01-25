use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j

fn main() {
    input! {
        n: usize,
    };

    let mut count: usize = 1;

    for i in 2..=n {
        count *= i
    }

    println!("{}", count)
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c010";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"20")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "2432902008176640000\n");
        assert!(output.stderr_str().is_empty());
    }
}
