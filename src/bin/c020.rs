use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c020";

    #[test]
    fn c0201() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"13
243 156 104 280 142 286 196 132 128 195 265 300 130
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"4
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
