use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_p

fn euclidean(mut a: usize, mut b: usize) -> usize {
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

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = a[0];
    for i in &a[1..] {
        ans = euclidean(ans, *i);
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c016";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3
12 18 24
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"6
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
