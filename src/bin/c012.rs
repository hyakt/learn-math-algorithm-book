use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l

fn is_prime(num: f64) -> bool {
    let r = num.sqrt().floor() as usize;
    for i in 2..=r {
        if (num as usize) % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        n: f64,
    };

    println!("{}", if is_prime(n) { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c012";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"53")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"77")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "No\n");
        assert!(output.stderr_str().is_empty());
    }
}
