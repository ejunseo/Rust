fn main() {
    // --- [1. 스칼라 타입 (Scalar Types)] ---

    // 1-1. 정수형 (Integer)
    let age: i32 = 30;           // i32: 32비트 부호 있는 정수
    let score: u8 = 100;         // u8: 8비트 부호 없는 정수 (0~255)
    let index: usize = 0;        // usize: 컴퓨터 환경(64비트 등)에 맞춘 주소값 타입 (중요!)

    // 1-2. 부동 소수점 (Floating-Point)
    let height: f64 = 175.5;     // f64: 64비트 실수

    // 1-3. 불리언 (Boolean)
    let is_rust_fun: bool = true;

    // 1-4. 문자형 (Character)
    let heart: char = '❤';       // 4바이트 유니코드

    // 1-5. 타입 추론 (Type Inference) - 추가!
    let implicit_x = 5;          // 타입을 안 적어도 컴파일러가 i32로 자동 결정!


    // --- [2. 컴파운드 타입 (Compound Types)] ---

    // 2-1. 튜플 (Tuple)
    let person: (&str, i32, f64) = ("Alice", 20, 160.0);
    let name = person.0; 

    // 2-2. 배열 (Array)
    let numbers: [i32; 3] = [10, 20, 30];
    let first_number = numbers[index]; // 위에서 선언한 usize 타입 index 사용


    // --- [출력 확인] ---
    println!("이름: {}, 이모지: {}, 자동추론: {}", name, heart, implicit_x);
    println!("나이: {}, 점수: {}, 키: {}", age, score, height);
    println!("배열 첫번째: {}, 러스트 재미있나? {}", first_number, is_rust_fun);
}
