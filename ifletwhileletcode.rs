fn main() {
    // 1. '3'이라는 숫자가 들어있는 상자(Option)를 만듭니다.
    let some_value = Some(3);

    // 2. [if let] "만약 상자 안에 든 게 Some(3)이면 실행해!"라는 뜻입니다.
    // match처럼 다른 경우(_ => )를 일일이 적어줄 필요가 없어 간결합니다.
    if let Some(3) = some_value {
        println!("값이 3이 맞네요!");
    }

    // 3. 1, 2, 3이 들어있는 리스트(vec)를 만듭니다.
    let mut stack = vec![1, 2, 3];

    // 4. [while let] "리스트에서 값을 꺼냈을 때(pop), 값이 있는 동안(Some) 계속 반복해!"
    // 리스트가 비어서 꺼낼 게 없으면(None) 알아서 반복문이 종료됩니다.
    while let Some(top) = stack.pop() {
        println!("꺼내온 값: {}", top);
    }
}
