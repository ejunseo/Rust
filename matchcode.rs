// 1. Direction이라는 이름의 '선택지 보따리(Enum)'를 만듭니다.
enum Direction {
    Up,     // 위
    Down,   // 아래
    Left,   // 왼쪽
    Right,  // 오른쪽
}

fn main() {
    // 2. dir이라는 변수를 만들고 '위(Up)'를 담습니다.
    let dir = Direction::Up;

    // 3. dir 안에 뭐가 들었는지 검사를 시작합니다.
    match dir {
        // 4. 만약 Up이 들어있다면? => 오른쪽 문장을 실행합니다.
        Direction::Up => println!("위로!"),

        // 5. 만약 Down이 들어있다면? => 오른쪽 문장을 실행합니다.
        Direction::Down => println!("아래로!"),

        // 6. [중요] _ (언더바)는 "나머지 전부"라는 뜻입니다.
        // 여기선 위에서 안 적은 'Left'와 'Right'를 한꺼번에 처리합니다.
        // 이게 없으면 "모든 경우를 다 안 적었다"며 에러가 납니다(완전 검사).
        _ => println!("나머지 방향(좌/우)"),
    } // 매칭 종료
}
