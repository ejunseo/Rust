fn main() {
    let mut s = String::from("hello");

    // 1. 불변 빌림 (여러 개 가능)
    let r1 = &s; 
    let r2 = &s;
    println!("{} and {}", r1, r2); // 여기까지는 문제 없음

    // 2. 가변 빌림 (규칙 위반 시 시나리오)
    // let r3 = &mut s; // 에러! r1, r2가 살아있는 동안에는 가변으로 빌릴 수 없음
    
    // 3. 빌림의 끝
    // r1, r2의 사용이 여기서 끝난다면...
    
    let r3 = &mut s; // 이제는 가능!
    r3.push_str(", world");
    println!("{}", r3);
}
