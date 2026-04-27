fn main() {
    // 1. Fn (읽기): 구경만 하기
    let s1 = String::from("Fn");
    let c1 = || println!("{}", s1); 
    c1(); c1(); // 여러 번 호출 가능

    // 2. FnMut (수정): 내용 바꾸기
    let mut s2 = String::from("FnMut");
    let mut c2 = || s2.push_str("!"); 
    c2(); c2(); // 여러 번 호출 가능
    println!("{}", s2);

    // 3. FnOnce (소유권): 아예 가져가기
    let s3 = String::from("FnOnce");
    let c3 = || drop(s3); 
    c3(); 
    // c3(); // 주석을 풀면 에러! (이미 써버림)
}
