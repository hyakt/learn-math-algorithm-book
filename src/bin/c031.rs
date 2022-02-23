use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ac

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.insert(0, 0);
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        if i == 1 {
            dp[i] = a[i];
            continue;
        }

        if dp[i - 1] > (a[i] + dp[i - 2]) {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = a[i] + dp[i - 2];
        }
    }

    // println!("dp: {:?}", dp);
    println!("{}", dp.iter().max().unwrap());

    // // debug
    // for i in dp {
    //     for j in i {
    //         if j == 0 {
    //             print!("   ")
    //         } else {
    //             print!("{:02} ", j)
    //         }
    //     }
    //     println!("");
    // }
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c031";

    #[test]
    fn sample_c031_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
2 5 3 3 1
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "8\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c031_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
1 2 3 4 5",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "9\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn sample_c031_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "\n");
        assert!(output.stderr_str().is_empty());
    }
}
