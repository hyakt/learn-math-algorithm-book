use std::cmp::min;

use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    };

    let mut m: Vec<isize> = vec![0; n];

    for i in 0..n {
        if i == 0 {
            m[0] = 0;
        } else if i == 1 {
            m[1] = (a[0] - a[1]).abs();
        } else {
            m[i] = min(
                (a[i] - a[i - 1]).abs() + m[i - 1],
                (a[i] - a[i - 2]).abs() + m[i - 2],
            );
        }
    }

    println!("{}", m[n - 1]);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c028";

    #[test]
    fn sample_c028_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"4
10 30 40 20
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"30
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c028_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"2
    10 10
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"0
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c028_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6
    30 10 60 10 60 50",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"40
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
