use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_v

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;

    // O(n^2)
    // for i in 0..n {
    //     for j in (i + 1)..n {
    //         if a[i] + a[j] == 100000 {
    //             ans += 1;
    //         }
    //     }
    // }

    // O(n)
    let mut arr: [usize; 100000] = [0; 100000];

    for i in a {
        arr[i] += 1;
    }

    for i in 1..=49999 {
        ans += arr[i] * arr[100000 - i];
    }

    // 50000はnCr(n = 50000の数、 r = 2)になる
    ans += (arr[50000] * (arr[50000] - 1)) / 2;

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c022";

    #[test]
    fn c0221() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6
40000 50000 20000 80000 50000 30000
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"2
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
