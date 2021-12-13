fn main() {
    // 구조체 정의
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 정의한 필드의 순서와 같을 필요x
    // 구조체 User의 인스턴스 생성
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someUsername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherUsername123"),
        ..user1
    };

    user1.sign_in_count = 2;
    // 불러올때는 . 찍어서
    println!(
        "\nUsername: {}\nEmail: {}\nSign in count: {}\nActive: {}\n",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    println!(
        "\nUsername: {}\nEmail: {}\nSign in count: {}\nActive: {}\n",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );
}
