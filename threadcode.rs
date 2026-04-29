use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("새로운 스레드: 양파 까는 중...");
    });

    handle.join().unwrap(); // 일꾼이 끝날 때까지 대기
    println!("메인 스레드: 요리 시작!");
}
