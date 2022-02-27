use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/abc167_d

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n]
    };
    a.insert(0, 0);
    let mut m = vec![0; n + 1];

    let mut next = 1;
    let mut v = Vec::<usize>::new();

    let mut f = true;

    for i in 1..n * 3 {
        m[next] += 1;

        next = a[next];

        if k == i {
            println!("{}", next);
            f = false;
        }

        if m[next] == 2 {
            v.push(next);
        }
    }

    if f {
        for i in m {
            if i == 1 {
                k -= 1;
            }
        }
        println!("{}", v[k % v.len()]);
    }
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c062";

    #[test]
    fn sample_c062_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"4 5
3 2 4 1
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "4\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c062_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6 727202214173249351
6 5 2 5 3 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "2\n");
        assert!(output.stderr_str().is_empty());
    }
}
