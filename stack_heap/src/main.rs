// fn main() {
//     {
//         let mut s = String::from("hello");
//         s.push_str(", world!");
//         println!("{}", s);
//     }
// }

//
//
//

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1: {}, s2: {}", s1, s2);
// }

//
//
//

// fn main() {
//     let s = String::from("hello"); // s가 스코프 안으로 들어왔습니다.

//     takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...
//                         // ... 그리고 이제 더이상 유효하지 않습니다.
//     let x = 5; // x가 스코프 안으로 들어왔습니다.

//     makes_copy(x); // x가 함수 안으로 이동했습니다만,
//                    // i32는 Copy가 되므로, x를 이후에 계속
//                    // 사용해도 됩니다.
// } // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
//   // 별다른 일이 발생하지 않습니다.

// fn takes_ownership(some_string: String) {
//     // some_string이 스코프 안으로 들어왔습니다.
//     println!("{}", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
//   // 해제되었습니다.

// fn makes_copy(some_integer: i32) {
//     // some_integer이 스코프 안으로 들어왔습니다.
//     println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

//
//
//

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership은 반환값을 s1에게
//                                 // 이동시킵니다.

//     let s2 = String::from("hello"); // s2가 스코프 안에 들어왔습니다.

//     let s3 = takes_and_gives_back(s2); // s2는 takes_and_gives_back 안으로
//                                        // 이동되었고, 이 함수가 반환값을 s3으로도
//                                        // 이동시켰습니다.
// } // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
//   // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
//   // 벗어나서 drop이 호출됩니다.

// fn gives_ownership() -> String {
//     // gives_ownership 함수가 반환 값을
//     // 호출한 쪽으로 이동시킵니다.

//     let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

//     some_string // some_string이 반환되고, 호출한 쪽의
//                 // 함수로 이동됩니다.
// }

// // takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string이 스코프
//     // 안으로 들어왔습니다.

//     a_string // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
// }

//
//
//

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len()함수는 문자열의 길이를 반환합니다.

//     (s, length)
// }

//
// 참조자, 엠퍼센드(&) 기호가 참조자, 이는 어떤 값을 소유권을 넘기지 않고 참조할 수 있도록 함.
//

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// } // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
//   // 때문에, 아무런 일도 발생하지 않습니다.

//
//
//
// 가변 참조자, 가변 참조자는 딱 한가지 큰 제한이 있습니다: 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다는 겁니다.
//
// ex)
//  let mut s = String::from("hello");
//  let r1 = &mut s;
//  let r2 = &mut s;
//  이는 안됨.
//
// 또한 우리는 불변 참조자를 가지고 있을 동안에도 역시 가변 참조자를 만들 수 없습니다.
//
//
//

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

//
//
//

// fn main() {
//     let my_string = String::from("he llo");
//     let my_first_word = first_word(&my_string);
//     println!("my string: {}, first word: {}", my_string, my_first_word);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

//
//
//

// fn main() {
//     let my_string = String::from("hello world");
//     let my_word = first_word(&my_string);

//     println!("{}", my_word);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     println!("dd");
//     &s[..]
// }

fn main() {
    let mut s = String::from("Hello~!");
    {
        let q = String::from(", world~!");
        s.push_str(&q);
    }
    println!("{}", s);
}
