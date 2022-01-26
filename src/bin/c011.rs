use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k

fn main() {
    input! {
        n: usize,
    };

    let mut primes: Vec<usize> = Vec::new();

    for i in 2..=n {
        let mut prime = true;
        for j in 2..i {
            if i % j == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(i);
        }
    }

    println!(
        "{}",
        primes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c011";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"10")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "2 3 5 7\n");
        assert!(output.stderr_str().is_empty());
    }
}
