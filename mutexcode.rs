use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5); // 5가 든 금고

    {
        let mut num = m.lock().unwrap(); // 열쇠 얻기
        *num = 6; // 데이터 수정
    } // 범위를 벗어나면 열쇠를 자동으로 반납 (자동 잠금)

    println!("결과: {:?}", m);
}
