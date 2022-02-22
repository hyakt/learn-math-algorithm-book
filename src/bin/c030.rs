use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_d

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [(usize, isize); n]
    };

    let mut weights: Vec<usize> = a.iter().map(|x| x.0).collect();
    weights.insert(0, 0);
    let mut values: Vec<isize> = a.iter().map(|x| x.1).collect();
    values.insert(0, 0);

    let mut m: Vec<Vec<isize>> = vec![vec![-1000000; w + 1]; n + 1];
    m[0][0] = 0;

    for i in 1..=n {
        for j in 0..=w {
            if j < weights[i] {
                m[i][j] = m[i - 1][j];
            }
            if j >= weights[i] {
                // println!("m[i]: {:?}", m[i]);
                // println!("i:j {:?}:{:?}", i, j);
                // println!("m[i - 1][j]: {:?}", m[i - 1][j]);
                // println!(
                //     "m[i - 1][j - weights[i]] + values[i]: {:?}",
                //     m[i - 1][j - weights[i]] + values[i]
                // );
                m[i][j] = std::cmp::max(m[i - 1][j], m[i - 1][j - weights[i]] + values[i]);
            }
        }
    }

    // // debug
    // for i in m {
    //     for j in i {
    //         if j < 0 {
    //             print!("   ",)
    //         } else {
    //             print!("{:02} ", j)
    //         }
    //     }
    //     println!("");
    // }

    println!("{}", m.into_iter().flatten().max().unwrap());
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c030";

    #[test]
    fn sample_c030_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3 8
3 30
4 50
5 60
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"90
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c030_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5 5
    1 1000000000
    1 1000000000
    1 1000000000
    1 1000000000
    1 1000000000
    ",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"5000000000
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c030_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6 15
6 5
5 6
6 4
6 6
3 5
7 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"17
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
