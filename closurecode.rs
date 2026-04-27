fn main() {
    let x = 4;
    let equal_to_x = |z| z == x; // 외부 변수 x를 '캡처'

    println!("4와 같은가요? {}", equal_to_x(4)); // true
    println!("7과 같은가요? {}", equal_to_x(7)); // false
}
