fn solve(l: usize, r: usize, arr: &Vec<i32>) -> i32 {
    if r - l == 1 {
        return arr[l as usize];
    }

    let m = (l + r) / 2;
    let s1 = solve(l, m, arr);
    let s2 = solve(m, r, arr);
    return s1 + s2;
}

fn main() {
    let arr = vec![10, 10, 10, 10, 10, 10];

    let ans = solve(0, arr.len(), &arr);

    println!("{:?}", ans);
}
