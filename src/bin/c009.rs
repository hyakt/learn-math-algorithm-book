use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i

fn main() {
    input! {
        n: usize,
        s: usize,
        mut a: [usize; n]
    };

    let mut dp: Vec<Vec<usize>> = vec![vec![0; s + 1]; n + 1];
    a.insert(0, 0);

    let mut yes = false;

    for i in 1..=n {
        for j in 1..=s {
            if a[i] == j
                || dp[i - 1][j] == j
                || j as isize - a[i] as isize > 0 && dp[i - 1][j - a[i]] > 0
            {
                if s == j {
                    yes = true;
                    break;
                }
                dp[i][j] = j;
            }
        }
    }

    if yes {
        println!("Yes");
    } else {
        println!("No");
    }

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
    const BIN: &'static str = "./c009";

    #[test]
    fn sample_c009_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3 11
    2 5 9
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"Yes
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c009_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"4 11
    3 1 4 5
    ",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"No
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
