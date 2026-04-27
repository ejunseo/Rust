fn main() {
    let nums = vec![1, 2, 3];

    let result: i32 = nums.into_iter()
        .filter(|x| *x > 0)     // filter: 0보다 큰 것만 (전부 통과)
        .map(|x| x + 1)         // map: 각각 1씩 더하기 (2, 3, 4)
        .fold(0, |acc, x| acc + x); // fold: 모두 더하기 (2 + 3 + 4)

    println!("결과: {}", result);
}
