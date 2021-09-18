# khaiii.rs
Rust wrapper for khaiii (Kakao Hangul Analyzer III)

## 주의사항
이 라이브러리는 C++로 작성된 라이브러리를 Rust에서 사용할 수 있게 해주는 Wrapper이며, unsafe 코드가 많이 사용되었습니다.
따라서 의도치 않은 메모리 문제(Memory Leak, Double Free 등)가 발생할 수 있습니다.

## 설치 방법
1. Khaiii 라이브러리를 설치합니다. (설치 방법은 [이곳](https://github.com/kakao/khaiii/wiki/%EB%B9%8C%EB%93%9C-%EB%B0%8F-%EC%84%A4%EC%B9%98)을 확인하세요.)
2. Cargo Dependency에 khaiii.rs를 추가합니다.  
`khaiii = { git = "https://github.com/sangwon090/khaiii.rs" }`

## 예제

라이브러리 버전 출력
```rust
use khaiii::*;

fn main() {
    let mut api = KhaiiiApi::new();
    api.open("".into(), "".into()).unwrap();

    println!("{}", api.version());
}
```

사용자가 입력만 문장의 형태소 분석
```rust
use khaiii::*;
use std::io;

fn main() {
    let mut api = KhaiiiApi::new();
    api.open("".into(), "".into()).unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let words = api.analyze(input, "".into()).unwrap();
        
        for word in words {
            println!("{:?}", word);
        }
    }
}
```