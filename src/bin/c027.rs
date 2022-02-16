use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aa

fn merge(l: usize, r: usize, a: &mut Vec<i32>) {
    if r - l == 1 {
        return;
    }

    let m = (l + r) / 2;

    merge(l, m, a);
    merge(m, r, a);

    let mut c = Vec::<i32>::new();

    let mut c1 = l;
    let mut c2 = m;

    while c1 != m || c2 != r {
        if c1 == m {
            c.push(a[c2]);
            c2 += 1;
        } else if c2 == r {
            c.push(a[c1]);
            c1 += 1;
        } else {
            if a[c1] < a[c2] {
                c.push(a[c1]);
                c1 += 1;
            } else {
                c.push(a[c2]);
                c2 += 1;
            }
        }
    }

    for i in 0..c.len() {
        a[l + i] = c[i];
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    };

    merge(0, a.len(), &mut a);

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c027";

    #[test]
    fn sample_c027_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3
3 1 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"1 2 3
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c027_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"10
658 299 47 507 122 969 449 68 513 800
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"47 68 122 299 449 507 513 658 800 969
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
