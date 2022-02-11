use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_q

fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
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
    }
}

fn lcm(a: usize, b: usize) -> usize {
    // a * b / gcd(a, b); この計算順序だとなぜか通らない
    (a / gcd(a, b)) * b
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = a[0];

    for i in &a[1..] {
        ans = lcm(ans, *i);
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c017";

    #[test]
    fn sample0() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
111 222 333 444 91
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"121212
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3
12 18 14
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"252
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
