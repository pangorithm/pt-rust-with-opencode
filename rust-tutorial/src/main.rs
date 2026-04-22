// # Rust 튜토리얼 - JavaScript 개발자를 위한 Rust 문법 가이드
//
// 이 프로젝트는 JavaScript (ES6+) 개발자들이 Rust의 핵심 문법과 개념을 빠르게
// 익힐 수 있도록 도와줍니다.
// 각 섹션은 main()에서 순차적으로 호출되는 독립적인 함수입니다.
//
// JavaScript 개발자를 위한 가장 중요한 차이점:
// - Rust는 **컴파일타임**에 타입과 메모리 관리를 결정합니다 (JavaScript는 런타임 사용)
// - Rust는 가비지 컬렉션 없이 **소유권 시스템**을 통해 메모리 안전성을 보장합니다
// - Rust는 **모든 값이 정의된 타입**을 가지지만, 강력한 타입 추론을 제공합니다

// ============================================================
// Section 1: 변수 선언
// ============================================================

// Rust의 `let`/`mut`은 JavaScript의 `let`/`const`에 해당합니다
// - JavaScript: `const`는 재할당 불가, `let`은 재할당 가능
// - Rust: `let`은 기본적으로 불변, `mut`을 추가해야 변경 가능
// - Rust의 타입 추론은 강력합니다 - 타입을 생략하면 자동으로 추론됩니다

fn section_1_variable_declarations() {
    // JavaScript의 `const`와 유사: 재할당 불가
    let immutable_value: i32 = 42;
    // immutable_value = 100; // ERROR! 재할당 불가

    // JavaScript의 `let`과 같은 재할당 (mut 사용)
    let mut mutable_value: i32 = 42;
    mutable_value = 100; // OK! 재할당 가능
    println!(
        "Section 1 - Variable Declarations: mutable = {}",
        mutable_value
    );

    // 타입 추론 - Rust는 타입을 명시하지 않으면 자동으로 추론
    let inferred_number = 42; // 자동으로 i32 추론
    let inferred_float = 3.14_f64; // 명시적으로 f64
    let inferred_string = "hello"; // 자동으로 &str (문자열 슬라이스) 추론
    println!(
        "   Type Inference: {} (i32), {} (f64), {} (&str)",
        inferred_number, inferred_float, inferred_string
    );

    // JavaScript의 `const`처럼 항상 불변으로 유지하고 싶다면 `mut`을 추가하지 마세요
    // 이것이 Rust의 철학입니다: 기본적으로 불변, 변경하려면 명시적으로 `mut` 선언
}

// ============================================================
// Section 2: 기본 타입
// ============================================================

// JavaScript에는 `number`(모든 숫자는 float64), `string`, `boolean`, `null`, `undefined` 등이 있습니다.
// Rust는 훨씬 더 명확한 타입 시스템을 제공합니다:
// - 정수: i8, i16, i32, i64, i128, isize (부호 있는)
// - 부호 없는 정수: u8, u16, u32, u64, u128, usize
// - 부동 소수점: f32, f64
// - 부울: bool
// - 문자: char (유니코드 스칼라 값, 1 문자)

fn section_2_primitive_types() {
    // 정수 타입 - JavaScript의 `number`와 달리 크기를 명시합니다
    let signed: i32 = -100; // -2^31 to 2^31-1 (JavaScript의 number와 유사한 범위)
    let unsigned: u32 = 100; // 0 to 2^32-1 (음수 불가)
    let big: i128 = 999_999_999_999; // 읽기 쉬움을 위한 언더스코어 (JavaScript에서도 가능)
    println!(
        "Section 2 - Integers: {} (i32), {} (u32), {} (i128)",
        signed, unsigned, big
    );

    // 부동 소수점 - JavaScript의 `number`는 모두 float64이지만, Rust에서는 선택 가능
    let float32: f32 = 3.14; // 32-bit 부동 소수점
    let float64: f64 = 3.141592653589793; // 64-bit (JavaScript의 number와 동일)
    println!("   Float: {} (f32), {} (f64)", float32, float64);

    // 부울 - JavaScript의 `true`/`false`와 동일
    let is_rust_great: bool = true; // JavaScript의 `true`와 동일
    let is_javascript_cool: bool = false;
    println!(
        "   Boolean: {} (Is Rust great? {})",
        is_javascript_cool, is_rust_great
    );

    // 문자 - JavaScript의 문자는 String이지만, Rust는 별도의 char 타입을 가집니다
    // JavaScript: `'a'`과 `"a"` 모두 String (차이 없음)
    // Rust: `'a'`은 char (유니코드 스칼라, 4바이트)
    let first_letter: char = 'R';
    let emoji: char = '🦀'; // Rust의 게 마스코트!
    println!("   Char: '{}' (char), '{}' (emoji)", first_letter, emoji);
}

// ============================================================
// Section 3: 문자열
// ============================================================

// JavaScript의 String과 Rust의 String은 근본적으로 다릅니다:
// - JavaScript: String은 불변이며, 메서드 호출은 항상 새로운 String을 반환
// - Rust: `String`은 변경 가능 (힙 할당), `&str`은 불변 (불변 슬라이스)
// - Rust는 메모리 효율을 위해 두 타입을 명확히 구분

fn section_3_strings() {
    // &str: 컴파일 타임에 알려진 불변 문자열 슬라이스 (문자열 참조)
    // JavaScript의 String에 가장 가까운 개념이지만, "슬라이스" 구분이 중요합니다.
    // 슬라이스(&str)는 기존 문자열(String)의 일부 또는 전체를 "참조"만 하는 타입으로,
    // 데이터 자체를 복사하지 않고 포인터(메모리 주소)와 길이만 보유합니다.
    // String::substring()이 새 객체를 생성하는 반면, &str[0..5]는 참조만 생성하므로 성능이 우수합니다.
    let string_slice: &str = "Hello, Rust!";
    println!("Section 3 - &str: {}", string_slice);

    // String: 힙의 변경 가능한 String
    // JavaScript의 String은 불변이지만, Rust의 String은 변경 가능
    let mut heap_string = String::from("Hello, ");
    heap_string.push_str("World!"); // push_str로 문자열 추가 (JavaScript의 `+=`에 해당)
    println!("   String: {}", heap_string);

    // JavaScript의 `+`로 문자열 연결은 비효율적 (새 객체 생성)
    // Rust의 `format!` 매크로는 JavaScript의 템플릿 리터럴과 유사
    let name = "Rust";
    let combined = format!("Hello, {}!", name); // JavaScript의 ``Hello, ${name}!``과 유사
    println!("   format!: {}", combined);

    // 문자열 연결 (JavaScript의 `+`에 해당)
    let mut string_concat = String::from("Hello");
    string_concat.push('!'); // OK! 문자 추가
    println!("   push!: {}", string_concat);
}

// ============================================================
// Section 4: 함수
// ============================================================

// JavaScript 함수와 Rust 함수의 주요 차이점:
// - JavaScript: 모든 값이 암시적 반환 (명시적 return 필요)
// - Rust: 마지막 표현식이 암시적 반환, `return`은 명시적 사용
// - JavaScript: `=>` (화살표 함수)로 익명 함수 정의
// - Rust: `|args| { ... }`로 클로저 사용 (별도 섹션에서 설명)

fn section_4_functions() {
    // JavaScript의 `function add(a, b)`에 해당
    // JavaScript와 달리 파라미터와 반환 타입을 명시해야 합니다
    fn add(a: i32, b: i32) -> i32 {
        a + b // ← 마지막 표현식이 암시적 반환 값 (JavaScript의 `return a + b`와 동일)
    }
    println!("   add(2, 3) = {}", add(2, 3));

    // JavaScript처럼 명시적 `return`도 사용 가능
    fn greet(name: &str) -> String {
        return format!("Hello, {}!", name); // 명시적 return
    }
    println!("   greet: {}", greet("JavaScript developer"));

    // JavaScript의 화살표 함수 `const double = (x) => x * 2;`에 해당
    // Rust에서도 같은 형태로 함수 정의 가능 (반환 타입 생략 시 추론)
    let double_fn = |x: i32| -> i32 { x * 2 }; // 이것은 클로저입니다 (별도 섹션에서 상세 설명)
    println!("   Closure double(21) = {}", double_fn(21));

    // JavaScript의 `void` 함수에 해당 - 반환 타입은 `()` (유니트 타입)
    fn log_message(message: &str) {
        // -> () 생략 가능 (유니트 타입)
        println!("   [LOG] {}", message);
    }
    log_message("Function section example");

    // JavaScript의 `default parameters`에 해당 - Rust는 기본값이 없지만
    // 오버로딩이 불가능하므로 다른 함수 이름이나 Option으로 처리
    fn optional_greet(name: Option<&str>) -> String {
        match name {
            Some(n) => format!("Hello, {}!", n),
            None => "Hello, World!".to_string(),
        }
    }
    println!("   Optional: {}", optional_greet(Some("Alice")));
    println!("   Optional: {}", optional_greet(None)); // JavaScript의 undefined와 유사
}

// ============================================================
// Section 5: Ownership (소유권)
// ============================================================

// **Rust에서 가장 중요한 개념** - JavaScript 개발자가 가장 어려워하는 부분
//
// JavaScript는 가비지 컬렉션(GC)으로 자동으로 메모리를 관리합니다.
// Rust는 GC가 없지만, 소유권 시스템을 통해 메모리 안전성을 보장합니다.
//
// 세 가지 규칙:
// 1. 모든 값에는 하나의 **소유자(Owner)**가 있습니다
// 2. 소유자가 범위를 벗어나면 값은 **버려집니다** (삭제됨)
// 3. 이동된 값은 사용할 수 없습니다

fn section_5_ownership() {
    // String은 힙에 데이터를 저장하는 타입
    // JavaScript: 모든 String은 힙에 저장
    let s1 = String::from("hello"); // s1이 "hello"의 소유자
    let s2 = s1; // s1의 값이 s2로 **이동**
                 // println!("{}", s1);            // ERROR! s1은 더 이상 유효하지 않음 (이동됨)
    println!("Section 5 - Ownership: s2 = {}", s2); // OK! s2가 소유자
                                                    // JavaScript 개발자를 위한 설명: JS의 GC가 해주는 일을 Rust에서는 직접 관리

    // 명시적 복사를 위한 Clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // OK! 명시적 복사 - 힙 메모리도 함께 복사
    println!("   Clone: s3 = {}, s4 = {}", s3, s4); // 둘 다 사용 가능
                                                    // JavaScript 개발자를 위한 설명: JavaScript의 `JSON.parse(JSON.stringify(obj))`와 유사하지만
                                                    // Rust에서는 성능 고려로 명시적으로 clone()을 호출해야 합니다

    // i32와 같은 기본 타입은 스택에 저장되므로, Move 대신 Copy가 발생
    let x = 42;
    let y = x; // OK! x가 복사됨 (Copy trait 구현 타입)
    println!("   Copy: x = {}, y = {}", x, y); // 둘 다 사용 가능
                                               // JavaScript 개발자를 위한 설명: JavaScript의 number는 모두 기본 타입이므로
                                               // 이것이 JavaScript의 동작과 가장 유사합니다
}

// ============================================================
// Section 6: References & Borrowing (참조 & 빌림)
// ============================================================

// 소유권의 대안으로, 소유하지 않고 값을 사용할 수 있습니다
// - `&T`: 불변 참조 (불변 빌림) - 여러 개 허용
// - `&mut T`: 변경 가능 참조 (변경 가능 빌림) - 하나만 허용 (동시에)
// JavaScript의 참조와 유사하지만, 규칙은 컴파일타임에 강제됩니다

fn section_6_references_and_borrowing() {
    // 불변 참조 - 값 복사 없이 참조만 전달
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1: 불변 참조 (빌링)
    println!(
        "Section 6 - Borrowing: Length is {}, s1 still valid: {}",
        len, s1
    );
    // JavaScript 개발자를 위한 설명: JavaScript의 객체 참조와 유사하지만
    // Rust는 컴파일타임에 "이 참조가 값을 변경하지 않을 것"을 보장

    // 변경 가능 참조 - 값을 변경할 수 있는 참조
    let mut s2 = String::from("hello");
    change_string(&mut s2); // &mut s2: 변경 가능 참조 (변경 가능 빌링)
    println!("   Mutable borrow: {}", s2);
    // JavaScript 개발자를 위한 설명: JavaScript의 객체는 기본적으로 변경 가능합니다.
    // Rust에서는 변경하려면 명시적으로 `&mut`을 요청해야 합니다

    // ✅ 동시에 여러 불변 참조 허용
    let _r1 = &s1;
    let _r2 = &s1;
    let _r3 = &s1; // 무한히 많은 불변 참조 허용

    // ❌ 변경 가능 참조와 불변 참조를 동시에 가질 수 없음
    // let _r4 = &mut s2;  // ERROR! 불변 참조가 있을 때 변경 가능 참조를 가질 수 없음
}

fn calculate_length(s: &String) -> usize {
    s.len() // JavaScript의 `string.length`에 해당
}

fn change_string(s: &mut String) {
    s.push_str(", world!"); // JavaScript의 `string += ", world!"`에 해당
}

// ============================================================
// Section 7: Structs (구조체)
// ============================================================

// JavaScript Object와 Rust Struct 비교
// - JavaScript: `{ name: "Alice", age: 30 }` - 동적, 유연
// - Rust: `struct` - 정적, 컴파일타임에 타입 결정
// Java의 Class와 유사하지만 메서드 없이 데이터만 저장

fn section_7_structs() {
    // JavaScript의 `class User { constructor(name, age) { this.name = name; this.age = age; } }`
    // Rust의 struct는 Java의 class와 매우 유사
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 인스턴스 생성 - JavaScript의 `new User("alice", ...)`에 해당
    let user1 = User {
        username: String::from("alice_dev"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("Section 7 - Struct: {} ({})", user1.username, user1.email);

    // 필드 접근 - JavaScript의 `user1.username`에 해당
    println!("   Active: {}", user1.active);

    // JavaScript의 객체는 변경 가능하지만, Rust struct 인스턴스는 기본적으로 불변
    // 변경하려면 `mut`을 추가해야 합니다
    let mut user2 = User {
        username: String::from("bob_dev"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("bob_new@example.com"); // OK! 변경 가능으로 변경
    println!("   After change: {}", user2.email);

    // struct 업데이트 문법 (spread operator과 유사)
    let user3 = User {
        ..user2 // user2의 나머지 필드 복사
    };
    // JavaScript의 `{...user2, email: "new"}`와 유사
    println!("   Spread: {}", user3.email);
}

// ============================================================
// Section 8: Enums & Match (열거형 & 패턴 매칭)
// ============================================================

// JavaScript에는 Enum이 없습니다 (Symbol 또는 문자열 상수를 사용).
// Rust의 Enum은 TypeScript의 Union Type과 유사하지만 훨씬 강력합니다.
// Java의 enum은 상수 집합에 불과하지만, Rust의 Enum은 각 variant에 데이터를 담을 수 있습니다.

fn section_8_enums_and_match() {
    // JavaScript의 `const Direction = { UP: 'up', DOWN: 'down', LEFT: 'left', RIGHT: 'right' };`
    // 하지만 Rust Enum은 데이터를 담을 수 있습니다!
    enum Message {
        Quit,                       // 데이터 없는 variant
        Move { x: i32, y: i32 },    // 객체 데이터 (JavaScript의 `{ x: 1, y: 2 }`)
        Write(String),              // 문자열 데이터
        ChangeColor(i32, i32, i32), // 튜플 데이터 (Java의 Tuple과 유사)
    }

    // JavaScript는 처리에 `switch (msg)`를 사용
    // Rust는 패턴 매칭에 `match`를 사용 - 모든 경우를 처리해야 함 (exhaustive)
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("   Quit message"),
        Message::Move { x, y } => println!("   Move: ({}, {})", x, y),
        Message::Write(text) => println!("   Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("   ChangeColor: RGB({}, {}, {})", r, g, b),
    }
    // JavaScript의 switch와 유사하지만, Rust는 모든 경우를 처리해야 함 (exhaustive match)

    // Option - JavaScript의 `null`/`undefined`를 처리하기 위한 Enum
    // Java의 Optional과 매우 유사
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    println!("Section 8 - Option: {:?}, {:?}", some_number, no_number);

    // JavaScript의 null 처리: `x !== null ? x : x.toString()`
    // Rust: match로 안전한 처리
    match some_number {
        Some(n) => println!("   Some: {}", n),
        None => println!("   None (null/undefined에 해당)"),
    }
}

// ============================================================
// Section 9: Pattern Matching (패턴 매칭)
// ============================================================

// JavaScript의 `switch`보다 훨씬 강력합니다.
// - JavaScript: `switch(value)` - 단순 값 매칭만
// - Rust: 패턴 매칭 - struct 해체, 범위, 조건부 매칭

fn section_9_pattern_matching() {
    // JavaScript의 switch에 해당
    let number = 42;
    match number {
        0 => println!("   0"),
        1 | 2 | 3 => println!("   1, 2, or 3"), // |로 여러 값 매칭
        10..=20 => println!("   10~20 range"),  // 범위 매칭 (JavaScript에서는 불가능)
        21..=100 => println!("   21~100 range"),
        _ => println!("   Other number"), // _ = 기본 케이스 (JavaScript의 switch default)
    }

    // 해체 - JavaScript의 구조 분해 할당과 유사
    let point = (3, 5); // JavaScript: const point = [3, 5];
    let (x, y) = point; // JavaScript: const [x, y] = point;
    println!("   Destructuring: ({}, {})", x, y);

    // 객체 해체 - JavaScript의 `const { name, age } = user;`와 유사
    struct Point2D {
        x: i32,
        y: i32,
    }
    let p = Point2D { x: 42, y: 99 };
    let Point2D { x: px, y: py } = p; // JavaScript: const { x: px, y: py } = p;
    println!("   Object destructuring: ({}, {})", px, py);

    // if let - JavaScript의 `if (condition)`에 해당
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("   Favorite color: {}", color);
    } else {
        println!("   No favorite color");
    }
    // JavaScript: `if (favoriteColor) { console.log(favoriteColor); }`와 유사
}

// ============================================================
// Section 10: Collections (컬렉션)
// ============================================================

// Rust 컬렉션은 JavaScript의 Array, Object(=Map)에 해당합니다
// - JavaScript의 Array와 Rust의 Vec은 유사하지만, 크기가 고정되지 않음
// - JavaScript의 Object(Map)와 Rust의 HashMap은 유사하지만, 타입 안전

fn section_10_collections() {
    // Vec(Vector)은 JavaScript의 Array에 해당
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // JavaScript: let numbers = [1, 2, 3, 4, 5];
    println!("Section 10 - Vec: {:?}", numbers);

    // JavaScript의 array.push에 해당
    numbers.push(6);
    println!("   After push: {:?}", numbers);

    // JavaScript의 array.forEach에 해당
    for num in &numbers {
        println!("   num: {}", num);
    }

    // JavaScript의 array.map에 해당
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   map(x*2): {:?}", doubled);

    // JavaScript의 array.filter에 해당
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    println!("   filter(even): {:?}", evens);

    // HashMap은 JavaScript의 Map에 해당
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Rust".to_string(), 95); // JavaScript: scores.set("Rust", 95);
    scores.insert("JavaScript".to_string(), 85);
    scores.insert("Python".to_string(), 90);

    // JavaScript: scores.get("Rust")
    if let Some(score) = scores.get("Rust") {
        println!("   Rust score: {}", score);
    }

    // JavaScript의 Object.keys와 유사
    for (language, score) in &scores {
        println!("   {} : {}", language, score);
    }
}

// ============================================================
// Section 11: Closures (클로저)
// ============================================================

// JavaScript의 화살표 함수 (`=>`)와 Rust의 클로저 (`|...|`)
// - JavaScript: `(x) => x * 2`
// - Rust: `|x| x * 2`
// - 둘 다 다른 함수의 인수로 전달할 수 있는 "1급 함수"입니다

fn section_11_closures() {
    // JavaScript의 `const double = (x) => x * 2;`에 해당
    let double_fn = |x| x * 2; // 파라미터 타입과 반환 타입은 추론
    println!("Section 11 - Closure: double(21) = {}", double_fn(21));

    // 타입을 명시적으로 지정
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    println!("   add(3, 4) = {}", add(3, 4));

    // JavaScript의 `arr.map(x => x * 2)`에 해당
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("   map(x*x): {:?}", squared);

    // JavaScript의 `arr.filter(x => x > 2)`에 해당
    let greater_than_two: Vec<i32> = numbers.iter().filter(|x| **x > 2).copied().collect();
    println!("   filter(> 2): {:?}", greater_than_two);

    // JavaScript의 `arr.reduce((acc, x) => acc + x, 0)`에 해당
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("   fold(sum): {}", sum);

    // 클로저는 주변 스코프의 변수를 캡처할 수 있습니다
    // JavaScript의 중첩 함수가 외부 변수에 접근하는 것과 유사
    let multiplier = 10;
    let multiply_by_ten = |x| x * multiplier; // multiplier를 캡처
    println!("   captured: multiply_by_ten(5) = {}", multiply_by_ten(5));
}

// ============================================================
// Section 12: Traits (트레이트)
// ============================================================

// JavaScript의 덕 타이핑("오리가처럼 울리면 그것은 오리다")과 유사하지만 타입 안전합니다.
// Java의 Interface와 매우 유사합니다.
// - Java: `interface Printable { void print(); }`
// - Rust: `trait Printable { fn print(&self); }`
// JavaScript에는 Interface가 없지만, TypeScript에는 있습니다. Rust의 Trait는 TypeScript Interface와 가장 유사합니다.

fn section_12_traits() {
    // Java의 `interface Drawable { void draw(); }`에 해당
    trait Drawable {
        fn draw(&self) -> String; // JavaScript: `draw() { return '...'; }`
    }

    // struct에 Trait 구현 (JavaScript의 class에 해당)
    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) -> String {
            format!("Circle: radius = {}", self.radius)
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) -> String {
            format!("Rectangle: {} x {}", self.width, self.height)
        }
    }

    // JavaScript의 `function drawAll(shape)`에 해당
    // 하지만 Rust는 타입 안전합니다 (구현체는 모두 Drawable이 될 수 있음)
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
    ];

    for shape in &shapes {
        println!("   {}", shape.draw());
    }

    // JavaScript에서는 `obj.toString()`을 호출하려면 `obj`가 toString 메서드를 가져야 합니다
    // Rust에서는 Trait 구현이 자동으로 `to_string()`을 추가합니다!
    let circle = Circle { radius: 5.0 };
    println!("   Circle: {:?}", circle); // Debug trait로 출력
}

// ============================================================
// Section 13: Error Handling (에러 처리)
// ============================================================

// Rust의 Result<T, E>는 JavaScript의 try/catch에 해당
// - JavaScript: try/catch/finally
// - Rust: Result<T, E> enum (성공: Ok(T), 실패: Err(E))
// - Java의 Checked Exception과 유사하지만, Result<T, E> enum을 사용합니다.

fn section_13_error_handling() {
    // JavaScript: `try { parseInt("42"); } catch(e) { console.log(e); }`
    // Rust: `Result<T, E>`로 오류를 값으로 처리
    let parsed: Result<i32, std::num::ParseIntError> = "42".parse();
    match parsed {
        Ok(number) => println!("OK - Result: {}", number),
        Err(e) => println!("Error: {:?}", e),
    }

    // Rust의 ? 연산자는 JavaScript의 `try { ... } catch(e) { return; }`에 해당
    fn safe_parse(s: &str) -> Result<i32, std::num::ParseIntError> {
        // JavaScript: try { return parseInt(s); } catch(e) { throw e; }
        // Rust: `?` 연산자는 오류가 발생하면 함수를 즉시 종료
        let num = s.parse::<i32>()?; // 이 값이 Err이면 함수가 즉시 오류 반환
        Ok(num) // OK면 Ok로 감싸서 반환
    }

    // println!("safe_parse('456'): {:?}", safe_parse("456"));

    // Option<T> - JavaScript의 null/undefined를 처리하는 타입
    // Java의 Optional과 매우 유사
    let maybe_name: Option<String> = Some("Alice".to_string());
    // JavaScript: const name = maybeName || 'Guest';
    // Rust: unwrap_or()로 기본값 제공
    let name = maybe_name.unwrap_or("Guest".to_string());
    println!("   name: {}", name);

    // ? 연산자 - None이면 즉시 None 반환
    // fn demo() { let n = maybe_name?; } // None이면 즉시 None 반환
}

// ============================================================
// Section 14: Generics (제네릭스)
// ============================================================

// JavaScript에는 제네릭이 없습니다 (TypeScript는 있음).
// Rust의 제네릭은 TypeScript의 제네릭과 매우 유사합니다.

fn section_14_generics() {
    // JavaScript: function identity(x) { return x; }  // 타입 없음
    // Rust: 제네릭으로 타입을 명시적으로 정의 가능
    fn identity<T>(value: T) -> T {
        value // 모든 타입으로 작동
    }
    println!("Section 14 - Generics: identity(42) = {}", identity(42));
    println!("   identity(\"hello\") = {}", identity("hello"));

    // JavaScript: [1, "hello", true]  // 배열 요소의 타입이 다를 수 있음
    // Rust: 제네릭 배열의 모든 요소는 같은 타입이어야 함
    let numbers: Vec<i32> = vec![1, 2, 3];
    println!("   Generic array: {:?}", numbers);

    // JavaScript의 Array.map에 해당하는 Rust의 제네릭 함수
    fn first<T>(slice: &[T]) -> &T {
        &slice[0] // 첫 번째 요소 반환
    }
    let numbers2: &[i32] = &[10, 20, 30];
    let first_num = first(numbers2);
    println!("   first([10, 20, 30]) = {}", first_num);
}

// ============================================================
// Section 15: Lifetimes (라이프타임)
// ============================================================

// Rust의 라이프타임은 JavaScript에 존재하지 않는 개념입니다.
// JavaScript는 GC로 자동으로 메모리를 정리하지만, Rust에는 GC가 없습니다.
// 라이프타임은 컴파일타임에 "이 참조가 얼마나 유효한지"를 보장합니다.
// JavaScript 개발자를 위한 설명: "JavaScript의 GC가 해주는 일을 Rust에서는 직접 관리"

fn section_15_lifetimes() {
    // 대부분의 경우, 라이프타임 없이도 컴파일러가 추론
    // JavaScript 개발자를 위한 설명: "JavaScript의 GC가 관리해주는 것을 Rust에서는 명시적"

    // 간단한 예시: 두 문자열 중 더 긴 것 반환
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let result = longest("hello", "world!");
    println!(
        "Section 15 - Lifetime: longest('hello', 'world!') = {}",
        result
    );
    // JavaScript: `const longest = (a, b) => a.length > b.length ? a : b;` 동일한 로직
    // 하지만 Rust는 컴파일타임에 "result가 s1/s2보다 오래 살아남지 않을 것"을 보장

    // 라이프타임 추론 - 대부분의 경우 명시할 필요가 없습니다
    fn get_first(s: &str) -> &str {
        // 컴파일러가 라이프타임을 자동으로 추론
        s
    }
    let word = get_first("hello lifetime");
    println!("   Auto inference: {}", word);
}

// ============================================================
// Section 16: Async/Await (비동기 처리)
// ============================================================

// Rust의 async 처리는 JavaScript의 `async/await`에 해당
// - JavaScript: `async function fetch() { const res = await fetch(url); }`
// - Rust: `async fn fetch() { let res = tokio::...await; }`
// - JavaScript는 이벤트 루프를 사용, Rust는 tokio 런타임을 async 처리에 사용

async fn section_16_async_example() {
    // JavaScript의 `const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));`
    // Rust의 `tokio::time::sleep`과 유사
    println!("Section 16 - Async: Async 처리 예제");
    // JavaScript: `setTimeout(() => console.log('after 100ms'), 100)`
    // Rust: `tokio::time::sleep(Duration::from_millis(100)).await;`
    println!("   Rust는 JavaScript와 동일한 async/await 패턴을 사용합니다!");
}

// ============================================================
// Section 17: Modules & Crates (모듈 & 크레이트)
// ============================================================

// Rust의 모듈 시스템은 JavaScript의 ES6 Module(import/export)에 해당
// - JavaScript: `import { foo } from './bar.js'` / `export function baz() {}`
// - Rust: `use crate::foo::bar;` / `pub fn baz() {}`
// - Rust의 Crate = JavaScript의 npm Package
// - Module = 코드를 구성하는 파일/폴더 구조
// - `pub` 키워드 = JavaScript의 `export` (가시성 제어)

fn section_17_modules_and_crates() {
    println!("Section 17 - Modules & Crates");

    // JavaScript의 ES6 모듈을 생각하세요:
    //   // math.js
    //   export function add(a, b) { return a + b; }
    //   export default class Calculator {}
    //
    //   // app.js
    //   import { add } from './math.js';
    //   import Calculator from './math.js';

    // Rust에서 `mod` 키워드로 모듈(네임스페이스)을 생성합니다
    // JavaScript의 `import`에 해당

    // JavaScript: `export function add(a, b) { return a + b; }`
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // JavaScript: `export function subtract(a, b) { return a - b; }`
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // `pub` 키워드로 외부에서 접근 가능하게 합니다 (JavaScript의 `export`)
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> Result<f64, String> {
        if b == 0 {
            Err("Division by zero!".to_string()) // JavaScript: throw new Error("Division by zero!")
        } else {
            Ok(a as f64 / b as f64) // JavaScript: return a / b
        }
    }

    // JavaScript의 `import`에 해당하는 것이 Rust의 `use` 키워드
    //   JavaScript: import { add, multiply } from './math.js';
    //   Rust: use module_name::{add, multiply};

    // 모듈 내의 함수 호출
    let result_add = add(5, 3);
    let result_multiply = multiply(4, 7);
    let result_divide = divide(10, 2).unwrap_or(0.0);
    println!(
        "   add(5, 3) = {}, multiply(4, 7) = {}, divide(10, 2) = {:.1}",
        result_add, result_multiply, result_divide
    );

    // JavaScript의 `export default`에 해당하는 패턴
    // Rust에는 `export default`가 없지만, pub struct로 대체 가능
    struct Calculator {
        history: Vec<String>, // JavaScript: private field처럼 사용 (pub 없으면 외부 접근 불가)
    }

    impl Calculator {
        fn new() -> Self {
            // JavaScript: `constructor() { this.history = []; }`
            Calculator {
                history: Vec::new(),
            }
        }

        // JavaScript: `add(a, b) { this.history.push(\`${a} + ${b}\`); return a + b; }`
        fn add(&mut self, a: i32, b: i32) -> i32 {
            let result = a + b;
            self.history.push(format!("{} + {} = {}", a, b, result));
            result
        }

        fn get_history(&self) -> &Vec<String> {
            &self.history
        }
    }

    let mut calc = Calculator::new();
    calc.add(10, 20);
    calc.add(30, 40);
    println!("   Calculator history: {:?}", calc.get_history());

    // 파일 구조 (JavaScript의 폴더 구조와 유사)
    //
    // JavaScript:
    //   /src
    //     /math
    //       index.js      // export { add, subtract }
    //       utils.js      // export { multiply, divide }
    //     app.js          // import { add, multiply } from './math';
    //
    // Rust:
    //   src/
    //     mod.rs          // mod math; (모듈 선언)
    //     math/
    //       mod.rs         // pub fn add() { ... } (index.js 역할)
    //       utils.rs      // pub fn multiply() { ... }
    //     main.rs         // use crate::math::{add, multiply};
    //
    // `use`로 모듈의 함수를 가져오기:
    //   `use crate::module_name::function_name;` (crate = 현재 프로젝트)
    //   `use std::collections::HashMap;` (std 라이브러리)

    // JavaScript의 npm Package = Rust의 Crate
    // package.json = Cargo.toml
    // node_modules/ = ~/.cargo/registry/
    // npm install = cargo build
    // JavaScript: `"dependencies": { "express": "^4.18.0" }`
    // Rust: `express = "4.18"` (Cargo.toml)
}

// ============================================================
// Section 18: File I/O (파일 입출력)
// ============================================================

// Rust의 File I/O는 JavaScript의 `fs` 모듈 (Node.js)에 해당
// - JavaScript: `const fs = require('fs'); fs.readFileSync('file.txt', 'utf8');`
// - Rust: `std::fs::read_to_string("file.txt")`
// - Rust의 에러 처리는 try/catch 대신 Result<T, E> 사용
// - Node.js의 `fs.promises.readFile()`과 유사하지만 Rust의 타입 안전성 추가

fn section_18_file_io() {
    println!("Section 18 - File I/O");

    // JavaScript (Node.js):
    //   const fs = require('fs');
    //   const data = fs.readFileSync('data.txt', 'utf8');
    //   fs.writeFileSync('output.txt', 'Hello World');
    //
    // Rust equivalent:
    //   let data = std::fs::read_to_string("data.txt");
    //   std::fs::write("output.txt", "Hello World");

    // JavaScript의 `fs.readFileSync()`에 해당하는 Rust 함수
    //   JavaScript: const content = fs.readFileSync('README.md', 'utf8');
    //   Rust: let content = std::fs::read_to_string("README.md");
    //
    // 읽기 실패 시 Result::Err를 반환 (try/catch 대신)
    // JavaScript: try { content = fs.readFileSync('nonexistent.txt'); } catch(e) { ... }
    // Rust: Result를 match로 처리
    let read_result = std::fs::read_to_string("Cargo.toml");
    match read_result {
        Ok(content) => {
            // 파일이 성공적으로 읽혔을 때
            // JavaScript: console.log(content);
            let line_count = content.lines().count();
            println!("   Cargo.toml 읽기 성공! ({}줄)", line_count);
            // 첫 줄만 출력
            if let Some(first_line) = content.lines().next() {
                println!("   첫 줄: {}", first_line);
            }
        }
        Err(e) => {
            // JavaScript의 catch(e) 블록에 해당
            // JavaScript: console.error('File read error:', e.message);
            println!("   파일 읽기 실패: {}", e);
        }
    }

    // JavaScript의 `fs.writeFileSync()`에 해당하는 Rust 함수
    //   JavaScript: fs.writeFileSync('hello.txt', 'Hello from Rust!');
    //   Rust: std::fs::write("hello.txt", "Hello from Rust!");
    //
    // 쓰기 성공 시 Result::Ok(()) 반환
    let write_result = std::fs::write("hello_rust.txt", "Hello from Rust!\nThis is a test file.\n");
    match write_result {
        Ok(()) => println!("   파일 쓰기 성공! (hello_rust.txt 생성)"),
        Err(e) => println!("   파일 쓰기 실패: {}", e),
    }

    // JavaScript의 `fs.existsSync()`에 해당하는 Rust 함수
    //   JavaScript: if (fs.existsSync('file.txt')) { ... }
    //   Rust: std::path::Path::exists("file.txt")
    use std::path::Path;
    let cargo_toml_exists = Path::new("Cargo.toml").exists();
    let nonexistent_exists = Path::new("nonexistent_file.txt").exists();
    println!("   Cargo.toml 존재: {}", cargo_toml_exists);
    println!("   nonexistent_file.txt 존재: {}", nonexistent_exists);

    // JavaScript의 `fs.mkdirSync()`에 해당하는 Rust 함수
    //   JavaScript: fs.mkdirSync('temp', { recursive: true });
    //   Rust: std::fs::create_dir_all("temp")
    let mkdir_result = std::fs::create_dir_all("tutorial_temp_dir");
    match mkdir_result {
        Ok(()) => println!("   디렉토리 생성 성공! (tutorial_temp_dir)"),
        Err(e) => println!("   디렉토리 생성 실패: {}", e),
    }

    // JavaScript의 `fs.readdirSync()`에 해당하는 Rust 함수
    //   JavaScript: const files = fs.readdirSync('.');
    //   Rust: std::fs::read_dir(".")
    let dir_result = std::fs::read_dir(".");
    if let Ok(entries) = dir_result {
        let mut count = 0;
        for entry in entries.take(5) {
            // 최대 5개만 표시 (JavaScript: files.slice(0, 5))
            if let Ok(e) = entry {
                if let Some(name) = e.path().file_name() {
                    println!("   디렉토리 항목: {:?}", name);
                    count += 1;
                }
            }
        }
        println!("   (총 {}개 중 5개 표시)", count);
    }

    // JavaScript의 `fs.rmSync()`에 해당하는 Rust 함수
    //   JavaScript: fs.rmSync('hello.txt');
    //   Rust: std::fs::remove_file("hello.txt") / std::fs::remove_dir_all("dir")
    // cleanup:
    let _ = std::fs::remove_file("hello_rust.txt");
    let _ = std::fs::remove_dir_all("tutorial_temp_dir");

    // JavaScript의 `fs.statSync()`에 해당하는 Rust 함수
    //   JavaScript: const stats = fs.statSync('Cargo.toml'); stats.size;
    //   Rust: std::fs::metadata("Cargo.toml").map(|m| m.len())
    if let Ok(metadata) = std::fs::metadata("Cargo.toml") {
        println!("   Cargo.toml 크기: {} bytes", metadata.len());
        println!(
            "   수정 시간: {:?}",
            metadata
                .modified()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH)
        );
    }

    // JavaScript의 `fs.appendFileSync()`에 해당하는 Rust 함수
    //   JavaScript: fs.appendFileSync('log.txt', 'new line\n');
    //   Rust: std::fs::OpenOptions (append 모드로 파일 열기)
    //   JavaScript: const stream = fs.createWriteStream('log.txt', { flags: 'a' });
    //   Rust: std::fs::OpenOptions::new().append(true).open("file.txt")
}

// ============================================================
// Section 19: Testing (테스트)
// ============================================================

// Rust의 테스트 시스템은 JavaScript의 Jest/Mocha/Vitest에 해당
// - JavaScript: `test('adds 1 + 2', () => { expect(add(1, 2)).toBe(3); })`
// - Rust: `#[test] fn test_add() { assert_eq!(add(1, 2), 3); }`
// - Rust 테스트는 `cargo test`로 실행 (JavaScript: `npm test`)
// - 동일 파일 테스트 (JavaScript: 별도 __tests__ 파일)

fn section_19_testing() {
    println!("Section 19 - Testing");

    // JavaScript (Jest):
    //   test('adds 1 + 2 to equal 3', () => {
    //     expect(add(1, 2)).toBe(3);
    //   });
    //
    //   test('returns negative result', () => {
    //     expect(add(-1, -2)).toBe(-3);
    //   });

    // Rust equivalent:
    //   #[test]
    //   fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    //   }

    // JavaScript의 `expect(actual).toBe(expected)`에 해당하는 것이 Rust의 `assert_eq!`
    // JavaScript의 `expect(actual).toBeTruthy()`에 해당하는 것이 Rust의 `assert!`
    // JavaScript의 `expect(actual).toBeFalsy()`에 해당하는 것이 Rust의 `assert!(!value)`
    // JavaScript의 `expect(actual).not.toBe()`에 해당하는 것이 Rust의 `assert_ne!`

    // 테스트할 함수 정의
    fn add_positive(a: i32, b: i32) -> i32 {
        a + b
    }

    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // JavaScript의 `describe()` 블록에 해당하는 것이 Rust의 `mod tests`
    // JavaScript:
    //   describe('math', () => {
    //     test('add', () => { ... });
    //   });
    //
    // Rust:
    //   #[cfg(test)]
    //   mod tests {
    //     use super::*;
    //     #[test]
    //     fn test_add() { ... }
    //   }

    // JavaScript의 `beforeEach()`에 해당하는 것이 Rust의 `#[before]` (없음)
    // 대신 함수 내에서 직접 초기화

    // JavaScript의 `afterEach()`에 해당하는 것이 Rust의 `#[after]` (없음)
    // 대신 함수 내에서 직접 정리

    // JavaScript의 `it('should throw', () => { expect(fn).toThrow(); })`
    // Rust: `#[should_panic]`

    // JavaScript의 `test.skip('...', ...)`에 해당하는 것이 Rust의 `#[ignore]`
    // JavaScript의 `test.only('...', ...)`에 해당하는 것이 Rust의 `--exact` 플래그

    // 실제 테스트 실행
    //   JavaScript: expect(addPositive(1, 2)).toBe(3);
    //   Rust: assert_eq!(addPositive(1, 2), 3);
    let test_value = add_for_test(3, 4);
    println!("   add_for_test(3, 4) = {} (expect: 7)", test_value);

    // JavaScript: expect(isEven(4)).toBeTruthy();
    // Rust: assert!(is_even_for_test(4));
    println!(
        "   is_even_for_test(4) = {} (expect: true)",
        is_even_for_test(4)
    );

    // JavaScript: expect(isEven(3)).toBeFalsy();
    // Rust: assert!(!is_even_for_test(3));
    println!(
        "   is_even_for_test(3) = {} (expect: false)",
        is_even_for_test(3)
    );

    // JavaScript의 `expect(value).toBeGreaterThan(5)`에 해당하는 것이 Rust의 `assert!(value > 5)`
    let result = add_for_test(10, 5);
    println!("   add(10, 5) = {} (> 5: {})", result, result > 5);

    // JavaScript의 `expect(value).toBeInstanceOf(Array)`에 해당하는 것이 Rust의 `assert!(value.is::<Vec<_>>())`
    let numbers = vec![1, 2, 3, 4, 5];
    println!("   Vec is array-like: {}", !numbers.is_empty());

    // 테스트 실행 방법:
    //   JavaScript: npm test 또는 jest
    //   Rust: cargo test
    //
    //   JavaScript: jest --testNamePattern='test_add'
    //   Rust: cargo test test_add
    //
    //   JavaScript: jest --coverage
    //   Rust: cargo test -- --show-output (또는 cargo llvm-cov for coverage)

    // Rust의 테스트는 컴파일타임에 타입 체크가 되기 때문에
    // JavaScript의 테스트보다 훨씬 안전한 편입니다
    // JavaScript: expect(add("1", 2)).toBe(3); // 런타임 에러!
    // Rust: assert_eq!(add(1, 2), 3); // 컴파일타임 에러 방지
}

fn add_for_test(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even_for_test(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // JavaScript: `test('adds 1 + 2 to equal 3', () => { expect(add(1, 2)).toBe(3); });`
    #[test]
    fn test_add() {
        assert_eq!(add_for_test(1, 2), 3);
    }

    // JavaScript: `test('returns negative result', () => { expect(add(-1, -2)).toBe(-3); });`
    #[test]
    fn test_negative_add() {
        assert_eq!(add_for_test(-1, -2), -3);
    }

    // JavaScript: `test('isEven', () => { expect(isEven(4)).toBeTruthy(); expect(isEven(3)).toBeFalsy(); });`
    #[test]
    fn test_is_even() {
        assert!(is_even_for_test(4));
        assert!(!is_even_for_test(3));
    }

    // JavaScript: `test('should throw', () => { expect(fn).toThrow(); });`
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic"); // JavaScript: throw new Error("test");
    }

    // JavaScript의 `describe('math', () => { ... });`에 해당하는 것이 Rust의 `mod`
    mod advanced_tests {
        use super::*;

        // JavaScript: `test('filter evens', () => { expect(filter([1,2,3,4,5]).filter(x => x%2==0)).toEqual([2,4]); });`
        #[test]
        fn test_filter_evens() {
            let numbers = vec![1, 2, 3, 4, 5, 6];
            let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
            assert_eq!(evens, vec![2, 4, 6]);
        }

        // JavaScript: `test('map doubles', () => { expect(map([1,2,3], x => x*2)).toEqual([2,4,6]); });`
        #[test]
        fn test_map_doubles() {
            let numbers = vec![1, 2, 3];
            let doubled: Vec<i32> = numbers.iter().map(|x| *x * 2).collect();
            assert_eq!(doubled, vec![2, 4, 6]);
        }
    }
}

// ============================================================
// Section 20: Iterators (이터레이터)
// ============================================================

// Rust의 Iterator 트레이트는 JavaScript의 배열 메서드와 제네레이터에 해당
// - JavaScript: `arr.map(x => x * 2)` / `for (const x of arr)`
// - Rust: `arr.iter().map(|x| x * 2)` / `for x in arr.iter()`
// - Rust Iterators는 Lazy합니다 (JavaScript의 Generator처럼 즉시 실행 안함)
// - JavaScript의 `for...of` = Rust의 `for x in iterator`
// - Rust의 Iterator는 성능이 매우 우수합니다 (컴파일타임 최적화)

fn section_20_iterators() {
    println!("Section 20 - Iterators");

    // JavaScript의 for...of와 Rust의 for...in 비교
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   for (const num of arr) {
    //     console.log(num);
    //   }
    // Rust:
    //   let arr = vec![1, 2, 3, 4, 5];
    //   for num in &arr {
    //     println!("{}", num);
    //   }

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // JavaScript의 Array.map() = Rust의 Iterator::map()
    // JavaScript: const doubled = arr.map(x => x * 2);
    // Rust: let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   map(x*2): {:?}", doubled);

    // JavaScript의 Array.filter() = Rust의 Iterator::filter()
    // JavaScript: const evens = arr.filter(x => x % 2 === 0);
    // Rust: let evens: Vec<i32> = arr.iter().filter(|x| x % 2 == 0).copied().collect();
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    println!("   filter(evens): {:?}", evens);

    // JavaScript의 Array.reduce() = Rust의 Iterator::fold()
    // JavaScript: const sum = arr.reduce((acc, x) => acc + x, 0);
    // Rust: let sum: i32 = arr.iter().fold(0, |acc, x| acc + x);
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("   fold(sum): {}", sum);

    // JavaScript의 Array.forEach() = Rust의 for loop (or Iterator::for_each())
    // JavaScript: arr.forEach(x => console.log(x));
    // Rust: for x in &arr { println!("{}", x); }
    //       arr.iter().for_each(|x| println!("{}", x));
    println!("   for_each:");
    numbers.iter().take(3).for_each(|x| print!("   {} ", x));
    println!(); // 줄바꿈 (JavaScript: console.log()와 동일)

    // JavaScript의 Array.find() = Rust의 Iterator::find()
    // JavaScript: const first = arr.find(x => x > 3); // 4
    // Rust: let first = arr.iter().find(|x| **x > 3); // Some(&4)
    if let Some(first) = numbers.iter().find(|x| **x > 3) {
        println!("   find(> 3): {}", first);
    }

    // JavaScript의 Array.some() = Rust의 Iterator::any()
    // JavaScript: arr.some(x => x > 5); // true
    // Rust: arr.iter().any(|x| *x > 5); // true
    let has_large = numbers.iter().any(|x| *x > 5);
    println!("   any(> 5): {}", has_large);

    // JavaScript의 Array.every() = Rust의 Iterator::all()
    // JavaScript: arr.every(x => x > 0); // true
    // Rust: arr.iter().all(|x| *x > 0); // true
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("   all(> 0): {}", all_positive);

    // JavaScript의 Array.slice(0, 3) = Rust의 Iterator::take(3)
    // JavaScript: const first3 = arr.slice(0, 3);
    // Rust: let first3: Vec<&i32> = arr.iter().take(3).copied().collect();
    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    println!("   take(3): {:?}", first_three);

    // JavaScript의 arr.slice(3) = Rust의 Iterator::skip(3)
    // JavaScript: const rest = arr.slice(3);
    // Rust: let rest: Vec<i32> = arr.iter().skip(3).copied().collect();
    let rest: Vec<i32> = numbers.iter().skip(3).copied().collect();
    println!("   skip(3): {:?}", rest);

    // JavaScript의 arr.entries() = Rust의 Iterator::enumerate()
    // JavaScript: for (const [i, val] of arr.entries()) { console.log(i, val); }
    // Rust: for (i, val) in arr.iter().enumerate() { println!("{}: {}", i, val); }
    println!("   enumerate:");
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("     [{}]: {}", i, val);
    }

    // JavaScript의 arr.concat(arr2) = Rust의 Iterator::chain()
    // JavaScript: const combined = [...arr1, ...arr2];
    // Rust: arr1.iter().chain(arr2.iter()).collect::<Vec<_>>()
    let more_numbers = vec![11, 12, 13];
    let combined: Vec<i32> = numbers.iter().chain(more_numbers.iter()).copied().collect();
    println!("   chain: {:?}", combined);

    // JavaScript의 Generator (function*):
    //   function* range(start, end) { for (let i = start; i < end; i++) yield i; }
    //   for (const n of range(1, 5)) { console.log(n); }
    // Rust의 Iterator는 이와 유사하게 Lazy하게 동작합니다
    // JavaScript: const lazy = arr.map(x => { console.log('processing x'); return x * 2; });
    //             lazy.forEach(x => console.log(x)); // map의 console.log가 즉시 실행됨
    // Rust: let lazy = arr.iter().map(|x| { println!("processing {}", x); x * 2 });
    //       // println이 즉시 실행되지 않음! Lazy evaluation!
    //       let result: Vec<i32> = lazy.collect();
    //       println!("{:?}", result);
    // JavaScript의 Generator와 달리 Rust는 컴파일타임에 최적화됨

    // Iterator의 Chaining (파이프라인 패턴)
    // JavaScript: const result = arr.filter(x => x > 2).map(x => x * 2).slice(0, 3);
    // Rust: let result: Vec<i32> = arr.iter().filter(|x| **x > 2).map(|x| x * 2).take(3).collect();
    let chained: Vec<i32> = numbers
        .iter()
        .filter(|x| **x > 2) // JavaScript: filter(x => x > 2)
        .map(|x| *x * 2) // JavaScript: map(x => x * 2)
        .take(3) // JavaScript: slice(0, 3)
        .collect();
    println!("   chain pipeline: {:?}", chained);

    // Iterator는 JavaScript의 Array method보다 더 많은 옵션을 제공합니다:
    // JavaScript: map, filter, reduce, find, some, every, forEach, includes, indexOf, etc.
    // Rust: map, filter, fold, find, any, all, for_each, contains, position, etc. + take, skip, step_by, zip, etc.
}

// ============================================================
// Section 21: Concurrency (동시성)
// ============================================================

// Rust의 동시성은 JavaScript의 Worker Threads와 Web Workers에 해당
// - JavaScript: `new Worker('worker.js')` / `postMessage()` / `onmessage`
// - Rust: `std::thread::spawn()` / `std::sync::mpsc::channel()`
// - JavaScript: `Promise.all([promise1, promise2])` = Rust: `thread.join()`
// - JavaScript의 `SharedArrayBuffer` = Rust의 `Arc<Mutex<T>>`
// - Rust는 컴파일타임에 데이터 경합(Data Race)을 방지합니다

fn section_21_concurrency() {
    println!("Section 21 - Concurrency");

    // JavaScript의 Worker:
    //   const worker = new Worker('./worker.js');
    //   worker.postMessage({ data: 42 });
    //   worker.onmessage = (e) => console.log(e.data);
    //
    // Rust의 Thread:
    //   let handle = std::thread::spawn(|| {
    //       println!("Hello from thread!");
    //   });
    //   handle.join().unwrap();

    // JavaScript의 Promise.all() = Rust의 thread::spawn + join()
    // JavaScript:
    //   const p1 = fetch('/api/users');
    //   const p2 = fetch('/api/posts');
    //   const results = await Promise.all([p1, p2]);
    //
    // Rust:
    //   let handle1 = std::thread::spawn(|| { work1(); });
    //   let handle2 = std::thread::spawn(|| { work2(); });
    //   let r1 = handle1.join().unwrap();
    //   let r2 = handle2.join().unwrap();

    // JavaScript: `const worker = new Worker(() => { ... });`
    // Rust: `let handle = std::thread::spawn(|| { ... });`
    // JavaScript의 Worker 스코프 = Rust의 `move` closure (변수를 소유권으로 가져옴)

    // 간단한 스레드 예시
    // JavaScript의 setTimeout과 유사한 개념 (하지만 Rust의 thread는 병렬 실행)
    let handle = std::thread::spawn(|| {
        // JavaScript: `setTimeout(() => { console.log('from worker'); }, 100);`
        println!("   Hello from spawned thread!");
        42 // 반환값 (JavaScript: `self.postMessage(42)`)
    });

    // JavaScript의 `worker.onmessage = (e) => console.log(e.data);`
    // Rust의 `handle.join().unwrap();`로 스레드의 결과를 받습니다
    let result = handle.join().unwrap();
    println!("   Thread returned: {}", result);

    // 여러 스레드 병렬 실행 (JavaScript의 Promise.all() 유사)
    // JavaScript:
    //   const tasks = [task1, task2, task3];
    //   const results = await Promise.all(tasks.map(t => t()));
    //
    // Rust:
    let handles: Vec<std::thread::JoinHandle<i32>> = (1..=3)
        .map(|i| {
            std::thread::spawn(move || {
                // JavaScript: `postMessage('Task ' + i);`
                println!("   Task {} running", i);
                i * 10
            })
        })
        .collect();

    // JavaScript: `for (const r of results) { console.log(r); }`
    // Rust: `for handle in handles { result.push(handle.join().unwrap()); }`
    for handle in handles {
        let r = handle.join().unwrap();
        println!("   Task result: {}", r);
    }

    // JavaScript의 MessageChannel과 유사한 것이 Rust의 Channel (mpsc = Multi-Producer, Single-Consumer)
    // JavaScript:
    //   const { port1, port2 } = new MessageChannel();
    //   port1.postMessage('Hello');
    //   port2.onmessage = (e) => console.log(e.data);
    //
    // Rust:
    //   let (tx, rx) = std::sync::mpsc::channel();
    //   tx.send('Hello');
    //   rx.recv().unwrap();

    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    // JavaScript: `worker.postMessage({ id: 1, data: [1,2,3] });`
    // Rust: `tx.send((1, vec![1, 2, 3])).unwrap();`
    std::thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap(); // JavaScript: `postMessage(i)`
                                 // JavaScript: `setTimeout(() => worker.postMessage(i), i * 100);`
        }
    });

    // JavaScript: `worker.onmessage = (e) => console.log(e.data);`
    // Rust: `rx.recv()`으로 메시지 받기
    println!("   Messages from channel:");
    for received in rx.iter() {
        println!("     Received: {}", received);
    }

    // JavaScript의 `SharedArrayBuffer`와 `Atomics` = Rust의 `Arc<Mutex<T>>`
    // JavaScript:
    //   const buffer = new SharedArrayBuffer(100);
    //   const view = new Int32Array(buffer);
    //   Atomics.add(view, 0, 1);
    //
    // Rust:
    //   let counter = Arc::new(Mutex::new(0));
    //   let counter_clone = Arc::clone(&counter);
    //   std::thread::spawn(move || {
    //       let mut num = counter_clone.lock().unwrap();
    //       *num += 1;
    //   });

    println!("   Rust guarantees no data races at compile time!");
    // JavaScript: 런타임에 데이터 경합이 발생할 수 있음
    // Rust: 컴파일타임에 데이터 경합 방지 (이것이 Rust의 가장 강력한 장점)

    // JavaScript의 Promise와 Rust의 Thread 비교:
    // JavaScript: const promise = new Promise(resolve => { setTimeout(() => resolve(42), 1000); });
    // Rust: let handle = std::thread::spawn(|| { std::thread::sleep(Duration::from_secs(1)); 42; });
    // JavaScript: const result = await promise;
    // Rust: let result = handle.join().unwrap();
}

// ============================================================
// Section 22: Cargo - Rust Package Manager (캐르고 - 패키지 매니저)
// ============================================================

// Cargo는 Rust의 내장 패키지 매니저로, npm/yarn/pnpm과 유사
// - Cargo.toml = package.json
// - cargo build = npm build / npm run build
// - cargo run = npm start / npm run dev
// - cargo test = npm test
// - cargo new = npm init
// - cargo add = npm install
// - cargo check = npm run lint (빠른 체크)
// - cargo clippy = ESLint (코드 품질 검사)
// - cargo fmt = Prettier (코드 포맷팅)

fn section_22_cargo() {
    println!("Section 22 - Cargo (Rust Package Manager)");

    // JavaScript의 package.json과 Rust의 Cargo.toml 비교
    //
    // JavaScript (package.json):
    //   {
    //     "name": "my-app",
    //     "version": "1.0.0",
    //     "dependencies": {
    //       "express": "^4.18.0"
    //     },
    //     "devDependencies": {
    //       "jest": "^29.0.0",
    //       "eslint": "^8.0.0"
    //     },
    //     "scripts": {
    //       "start": "node index.js",
    //       "test": "jest",
    //       "build": "webpack"
    //     }
    //   }
    //
    // Rust (Cargo.toml):
    //   [package]
    //   name = "my-app"
    //   version = "0.1.0"
    //   edition = "2021"
    //   [dependencies]
    //   serde = "1.0"
    //   tokio = { version = "1.0", features = ["full"] }
    //   [dev-dependencies]
    //   mockito = "1.0"

    // JavaScript의 npm 명령어와 Rust의 Cargo 명령어 비교:
    //
    // | npm/yarn/pnpm          | Cargo              | Description                  |
    // |------------------------|--------------------|------------------------------|
    // | npm init                 | cargo init         | 새 프로젝트 시작             |
    // | npm init my-app/       | cargo new my-app   | 새 프로젝트 생성 (폴더 포함)  |
    // | npm install            | cargo build        | 의존성 설치 및 빌드          |
    // | npm install express    | cargo add express  | 패키지 설치                   |
    // | npm start              | cargo run          | 프로젝트 실행                |
    // | npm test               | cargo test         | 테스트 실행                  |
    // | npm run build          | cargo build        | 빌드 (실제 실행 파일 생성)    |
    // | npm run lint           | cargo clippy       | 코드 품질 검사               |
    // | npm run format         | cargo fmt          | 코드 포맷팅                  |
    // | npm outdated           | cargo outdated     | 오래된 의존성 확인           |
    // | npm update             | cargo update       | 의존성 업데이트              |
    // | npm pack               | cargo package      | 패키지 압축 (crates.io 업로드)|
    // | npm publish            | cargo publish      | crates.io에 공개             |
    // | npm ls                 | cargo tree         | 의존성 트리가 표시           |
    // | npx                    | cargo-expand         | 패키지 실행                  |

    // Cargo.toml의 구조 설명
    //
    // [package] 섹션 (JavaScript의 package.json 최상위 필드)
    //   name = "my-app"        // 프로젝트 이름 (JavaScript: "name": "my-app")
    //   version = "0.1.0"      // 버전 (JavaScript: "version": "0.1.0")
    //   edition = "2021"       // Rust 에디션 (JavaScript: 없음, Node.js 버전)
    //
    // [dependencies] 섹션 (JavaScript의 "dependencies")
    //   serde = "1.0"          // serde crate의 1.0.x 버전 (JavaScript: "serde": "1.0")
    //   tokio = { version = "1.0", features = ["full"] }
    //                           // 옵션 지정 (JavaScript: "tokio": {"version": "1.0", ...})
    //
    // [dev-dependencies] 섹션 (JavaScript의 "devDependencies")
    //   mockito = "1.0"        // 테스트용 의존성

    // Cargo Workspace (JavaScript의 npm Workspaces / monorepo)
    //
    // JavaScript (npm workspaces):
    //   {
    //     "workspaces": ["packages/*"]
    //   }
    //   packages/
    //     ui/         // package.json
    //     utils/      // package.json
    //
    // Rust (Cargo workspace):
    //   Cargo.toml (workspace 정의)
    //   packages/
    //     ui/         // Cargo.toml
    //     utils/      // Cargo.toml
    //
    // JavaScript: `npm install --workspace packages/ui`
    // Rust: `cargo build --package ui`

    // crates.io (JavaScript의 npm registry)
    //
    // JavaScript: https://www.npmjs.com/
    // Rust: https://crates.io/
    //
    // JavaScript: `npm search express`
    // Rust: `cargo search express` (하지만 Rust에는 express가 없음, Rust의 web framework는 axum, actix-web)

    // Cargo의 빌드 프로세스 (JavaScript의 build pipeline)
    //
    // JavaScript:
    //   1. npm install (의존성 설치)
    //   2. npm run build (빌드)
    //   3. node dist/main.js (실행)
    //
    // Rust:
    //   1. cargo build (빌드 + 의존성 설치)
    //   2. cargo run (빌드 + 실행)
    //   3. cargo check (빠른 체크, 컴파일만)
    //
    // JavaScript의 TypeScript: tsc (컴파일) → node (실행)
    // Rust: cargo build (컴파일 + 링크) → 실행 파일 직접 실행
    // Rust는 컴파일러가 native 코드를 생성하므로 별도의 런타임이 필요 없음

    // Cargo의 Features (JavaScript의 npm optional dependencies / peer dependencies)
    //
    // JavaScript: "react": {"optional": true}
    // Rust: serde = { version = "1.0", features = ["derive"] }
    //
    // JavaScript의 "scripts" = Rust의 cargo subcommands
    // JavaScript: "build": "webpack --mode production"
    // Rust: cargo build --release (optimization enabled)
    // JavaScript: npm run test -- --coverage
    // Rust: cargo test -- --nocapture (출력 표시)

    // Cargo의 유용한 명령어 모음
    //
    // 빠른 명령어:
    //   cargo check    - 빠른 컴파일 체크 (JavaScript: npm run lint)
    //   cargo build    - 빌드 (JavaScript: npm run build)
    //   cargo run      - 빌드 + 실행 (JavaScript: npm start)
    //   cargo test     - 테스트 실행 (JavaScript: npm test)
    //   cargo clean    - 빌드 결과 제거 (JavaScript: npm run clean)
    //
    // 코드 품질:
    //   cargo clippy   - 정적 분석 (JavaScript: npm run lint / ESLint)
    //   cargo fmt      - 코드 포맷팅 (JavaScript: npm run format / Prettier)
    //   cargo doc      - 문서 생성 (JavaScript: npm run docs)
    //   cargo doc --open - 문서 생성 + 브라우저에서 열기
    //
    // 의존성 관리:
    //   cargo add      - 의존성 추가 (JavaScript: npm install)
    //   cargo tree     - 의존성 트리 (JavaScript: npm ls)
    //   cargo outdated - 오래된 의존성 확인 (JavaScript: npm outdated)
    //   cargo update   - 의존성 업데이트 (JavaScript: npm update)

    // JavaScript의 Node.js 실행 = Rust의 컴파일된 실행 파일
    // JavaScript: node app.js
    // Rust: ./target/debug/my-app (개발 빌드)
    // Rust: ./target/release/my-app (최적화 빌드)
    //
    // JavaScript: npm run dev (dev server)
    // Rust: cargo run (개발 서버)
    // Rust: cargo build --release (production 빌드)
}

// ============================================================
// 메인 함수 - 모든 섹션 실행
// ============================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust Tutorial - JavaScript 개발자를 위한 Rust 문법 가이드");
    println!("============================================================\n");

    // 섹션 1~15는 동기 함수이므로 직접 호출
    section_1_variable_declarations();
    section_2_primitive_types();
    section_3_strings();
    section_4_functions();
    section_5_ownership();
    section_6_references_and_borrowing();
    section_7_structs();
    section_8_enums_and_match();
    section_9_pattern_matching();
    section_10_collections();
    section_11_closures();
    section_12_traits();
    section_13_error_handling();
    section_14_generics();
    section_15_lifetimes();

    // 섹션 16은 async 함수이므로 await 필요
    section_16_async_example().await;

    // 섹션 17~22는 동기 함수이므로 직접 호출
    section_17_modules_and_crates();
    section_18_file_io();
    section_19_testing();
    section_20_iterators();
    section_21_concurrency();
    section_22_cargo();

    println!("\n============================================================");
    println!("All 22 sections complete! Rust tutorial finished.");
    println!("Hope this helps JavaScript developers understand Rust's core concepts!");

    Ok(())
}
