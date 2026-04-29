use std::sync::mpsc; // mpsc: 여러 송신자, 한 명의 수신자
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // tx: 송신자, rx: 수신자

    thread::spawn(move || {
        tx.send("양파 다 깠어요!").unwrap(); // 데이터 던지기
    });

    let message = rx.recv().unwrap(); // 데이터 받기
    println!("메시지 도착: {}", message);
}
