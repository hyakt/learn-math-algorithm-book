use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_n

fn main() {
    input! {
        n: usize,
    };

    let r = (n as f64).sqrt().floor() as usize;

    let mut count = n;
    let mut ans = Vec::new();

    for i in 2..=r {
        if count % i == 0 {
            loop {
                if count % i == 0 {
                    ans.push(i);
                    count /= i;
                } else {
                    break;
                }
            }
        }
    }

    if count > 1 {
        ans.push(count)
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c014";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"10")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "2 5\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"36")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "2 2 3 3\n");
        assert!(output.stderr_str().is_empty());
    }
}
