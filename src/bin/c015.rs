use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    };

    let ans = loop {
        if a == 0 || b == 0 {
            if a > 0 {
                break a;
            }
            break b;
        }

        if a >= b {
            a %= b
        } else {
            b %= a
        }
    };

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c015";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"33 88")
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"11
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"123 777
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"3
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"123 1
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"1
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
