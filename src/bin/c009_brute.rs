use proconio::input;

fn main() {
    input! {
        n: u32,
        s: u32,
        mut a: [u32; n],
    };

    for i in 0..(1 << n) {
        let mut sum: u32 = 0;
        // print!("{:4b}: ", i);
        for j in 0..n {
            if i & (1 << j) != 0 {
                // print!("{}, ", a[j as usize]);
                sum += a[j as usize];
            }
        }

        if sum == s {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c008";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3 11
2 5 9",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"4 11
3 1 4 5",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "No\n");
        assert!(output.stderr_str().is_empty());
    }
}
