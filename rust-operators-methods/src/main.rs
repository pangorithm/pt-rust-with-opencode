// # Rust 연산자 & 메서드 완전 정복
//
// 이 프로젝트는 Rust에서 가장 자주 사용되는 연산자(operator)와 메서드(method)를
// 한 곳에서 정리합니다. JavaScript/Java 개발자가 Rust의 핵심 연산/메서드를
// 빠르게 익힐 수 있도록 실습 예제와 비교표를 포함합니다.
//
// ============================================================
// Rust 연산자 & 메서드 가이드
// JavaScript/Java 배경 지식을 가진 개발자를 위한 참고서
// ============================================================

// ============================================================
// Section 1: 산술 연산자 (Arithmetic Operators)
// ============================================================
//
// Rust의 산술 연산자는 JavaScript/Java와 동일하지만,
// unsigned 타입에서의 overflow 행동이 다릅니다.
//
// JavaScript/Java          Rust                  설명
// ──────────────────────   ───────────────────   ─────────────────────────
// 10 + 3                   10 + 3                더하기
// 10 - 3                   10 - 3                빼기
// 10 * 3                   10 * 3                곱하기
// 10 / 3                   10 / 3                나누기 (Integer Division → 3)
// 10 % 3                   10 % 3                나머지 (mod)
// -x                       -x                    음수 (negation)
//
// debug_assertions 활성화 시 overflow 체크:
//   debug 빌드: overflow 시 패닉 (에러)
//   release 빌드: wrap-around (모듈로 연산)
//   → cargo build --release 로 컴파일하면 wrap-around

fn section_1_arithmetic_operators() {
    println!("Section 1 - Arithmetic Operators");

    let a: i32 = 10;
    let b: i32 = 3;

    // 더하기
    println!("   {} + {} = {}", a, b, a + b);

    // 빼기
    println!("   {} - {} = {}", a, b, a - b);

    // 곱하기
    println!("   {} * {} = {}", a, b, a * b);

    // 나누기 (Integer Division → 정수 결과)
    println!("   {} / {} = {}", a, b, a / b); // 10 / 3 = 3

    // 나머지 (modulo)
    println!("   {} % {} = {}", a, b, a % b); // 10 % 3 = 1

    // 음수 연산
    println!("   -{} = {}", a, -a);

    // 부동 소수점 나누기
    let x: f64 = 10.0;
    let y: f64 = 3.0;
    println!("   {:.1} / {:.1} = {:.4}", x, y, x / y); // 3.3333

    // unsigned 타입 overflow (debug 빌드 패닉 / release 빌드 wrap-around)
    // wrap-around demonstrat: wrapping_add 사용
    // JavaScript: (255 + 1) % 256 = 0
    let u: u8 = 255;
    println!("   u8: 255.wrapping_add(1) = {} (wrap-around)", u.wrapping_add(1));
    // 주의: debug 빌드에서 u + 1은 패닉, wrapping_add는 항상 안전
}

// ============================================================
// Section 2: 비교 연산자 (Comparison Operators)
// ============================================================
//
// JavaScript와 Rust의 근본적 차이:
//   JavaScript: == 로 타입 변환 후 비교 (falsy truthy!)
//     0 == false → true (위험!)
//     "" == 0 → true (위험!)
//   Rust: == 은 타입과 값 모두 비교, 다른 타입 비교 자체가 불가능
//     0 == false → 컴파일 에러! (타입이 다름)
//     0 == 0 → true (정확한 비교)
//
// JavaScript          Rust                  설명
// ──────────────────  ───────────────────   ─────────────────────────
// ==                   ==                    같다 (타입 엄격)
// !==                  !=                    같지 않다
// <                    <                    작다
// >                    >                    크다
// <=                   <=                    작거나 같다
// >=                   >=                    크거나 같다

fn section_2_comparison_operators() {
    println!("Section 2 - Comparison Operators");

    let x: i32 = 10;
    let y: i32 = 20;

    // 같다 (==)
    println!("   10 == 10: {}", 10 == 10);

    // 같지 않다 (!=)
    println!("   10 == 20: {}", x != y);

    // 작다 (<)
    println!("   10 < 20: {}", x < y);

    // 크다 (>)
    println!("   20 > 10: {}", y > x);

    // 작거나 같다 (<=)
    println!("   10 <= 20: {}", x <= y);

    // 크거나 같다 (>=)
    println!("   20 >= 20: {}", y >= y);

    // JavaScript의 == vs Rust의 == 비교
    // JavaScript: 0 == false → true (!!! 위험!)
    // Rust: 0 == false → 컴파일 에러 (i32 vs bool, 타입 다름!)
    // Rust는 타입이 다르면 비교 자체를 허용하지 않습니다!
}

// ============================================================
// Section 3: 논리 연산자 (Logical Operators)
// ============================================================
//
// JavaScript와 유사하지만, JavaScript의 "truthy/falsy" 개념이 Rust에는 없습니다.
//
// JavaScript          Rust                  설명
// ──────────────────  ───────────────────   ─────────────────────────
// &&                   &&                    AND (단락 평가)
// ||                   ||                    OR (단락 평가)
// !                    !                     NOT
//
// 단락 평가 (Short-circuit evaluation):
//   false && ... → 두 번째 피연산자 평가 안 함
//   true || ... → 두 번째 피연산자 평가 안 함
//   → 성능 최적화 + 에러 방지

fn section_3_logical_operators() {
    println!("Section 3 - Logical Operators");

    let is_admin: bool = true;
    let is_logged_in: bool = true;
    let is_expired: bool = false;

    // AND (&&) - 두 조건이 모두 true일 때 true
    println!("   is_admin && is_logged_in: {}", is_admin && is_logged_in);

    // OR (||) - 하나라도 true면 true
    println!("   is_admin || is_expired: {}", is_admin || is_expired);

    // NOT (!)
    println!("   !is_expired: {}", !is_expired);

    // AND/OR 조합 (JavaScript와 동일한 우선순위)
    println!("   (2 > 1) && (3 > 2): {}", (2 > 1) && (3 > 2));
    println!("   (2 > 3) || (3 > 2): {}", (2 > 3) || (3 > 2));
    println!("   !(2 > 3): {}", !(2 > 3));

    // 단락 평가 예시
    let x = 5;
    // false && 무조건 실패 → 두 번째 평가 안 함
    if x > 10 && x < 100 {
        println!("   (이곳은 실행 안 됨)");
    } else {
        println!("   단락 평가: 첫 조건 실패 → 두 번째 평가 skipped");
    }

    // JavaScript의 truthy/falsy vs Rust
    // JavaScript: if (0) {} // 실행 안 됨 (falsy!)
    // JavaScript: if ("") {} // 실행 안 됨 (falsy!)
    // Rust: if 0 {} // 컴파일 에러! bool만 허용
    //   Rust에는 truthy/falsy 개념이 없습니다.
    //   모든 조건식은 명시적으로 bool이어야 합니다.
}

// ============================================================
// Section 4: 비트 연산자 (Bitwise Operators)
// ============================================================
//
// JavaScript와 Rust 모두 정수 타입에만 사용됩니다.
// JavaScript는 BigInt를 제외하면 모든 number가 float64이므로
// 비트 연산 시 자동으로 32비트 정수로 변환됩니다.
// Rust는 명시적인 정수 타입에서만 사용됩니다.
//
// JavaScript          Rust                  설명
// ──────────────────  ───────────────────   ─────────────────────────
// a & b                a & b                 AND (비트 단위)
// a | b                a | b                 OR (비트 단위)
// a ^ b                a ^ b                 XOR (비트 단위)
// a << b               a << b                왼쪽 시프트
// a >> b               a >> b                오른쪽 시프트 (부호 유지)
// a >>> b              (Rust에 없음)        오른쪽 시프트 (부호 무시)
// ~a                   (!a 아님!)           NOT (비트 단위, Rust에 없음)

fn section_4_bitwise_operators() {
    println!("Section 4 - Bitwise Operators");

    let a: u32 = 0b1010; // 10 (이진: 1010)
    let b: u32 = 0b0110; // 6  (이진: 0110)

    // AND (&) - 양쪽 1인 비트만 1
    println!("   {} & {} = {} (AND: 1010 & 0110 = {:04b})",
        a, b, a & b, a & b);

    // OR (|) - 하나라도 1인 비트 1
    println!("   {} | {} = {} (OR: 1010 | 0110 = {:04b})",
        a, b, a | b, a | b);

    // XOR (^) - 서로 다른 비트만 1
    println!("   {} ^ {} = {} (XOR: 1010 ^ 0110 = {:04b})",
        a, b, a ^ b, a ^ b);

    // NOT - 모든 비트 반전 (&! 사용)
    println!("   !{} = {} (NOT: ~{:04b} = {:04b})",
        a, !a, a, !a);

    // 왼쪽 시프트 - 2의 거듭제곱 곱하기
    // 1 << 3 = 8 (2^3)
    println!("   1 << 3 = {} (1 * 2^3 = 8)", 1 << 3);
    println!("   5 << 2 = {} (5 * 2^2 = 20)", 5 << 2);

    // 오른쪽 시프트 - 2의 거듭제곱 나누기
    // 8 >> 1 = 4 (8 / 2)
    println!("   8 >> 1 = {} (8 / 2^1 = 4)", 8 >> 1);
    println!("   20 >> 2 = {} (20 / 2^2 = 5)", 20 >> 2);

    // 실제 활용 예시: 플래그 비트
    let READ: u32 = 0b0001;
    let WRITE: u32 = 0b0010;
    let EXECUTE: u32 = 0b0100;
    let READ_WRITE: u32 = READ | WRITE;
    println!("   READ: {:04b}, WRITE: {:04b}, EXECUTE: {:04b}",
        READ, WRITE, EXECUTE);
    println!("   READ_WRITE: {:04b} (READ | WRITE)", READ_WRITE);
    println!("   READ_WRITE & READ = {} (READ 포함 체크)", READ_WRITE & READ);
}

// ============================================================
// Section 5: 대입 연산자 (Assignment Operators)
// ============================================================
//
// JavaScript와 동일하지만, Rust에서는 연산 결과를 다시 변수에
// 대입할 때 mut 키워드가 필요합니다.
//
// JavaScript          Rust                  설명
// ──────────────────  ───────────────────   ─────────────────────────
// x = a                x = a                기본 대입
// x += a               x += a                더하고 대입
// x -= a               x -= a                빼고 대입
// x *= a               x *= a                곱하고 대입
// x /= a               x /= a                나누고 대입
// x %= a               x %= a                나머지와 대입
// x &= a               x &= a                AND 후 대입
// x |= a               x |= a                OR 후 대입
// x ^= a               x ^= a                XOR 후 대입
// x <<= a              x <<= a               왼쪽 시프트 후 대입
// x >>= a              x >>= a               오른쪽 시프트 후 대입

fn section_5_assignment_operators() {
    println!("Section 5 - Assignment Operators");

    // Rust에서는 mut 없이는 재할당 불가
    let mut x: i32 = 10;
    x += 5;
    println!("   x += 5  → x = {}", x); // 15

    let mut y: i32 = 20;
    y -= 3;
    println!("   y -= 3  → y = {}", y); // 17

    let mut z: i32 = 4;
    z *= 3;
    println!("   z *= 3  → z = {}", z); // 12

    let mut w: i32 = 10;
    w /= 3;
    println!("   w /= 3  → w = {}", w); // 3

    let mut m: i32 = 10;
    m %= 3;
    println!("   m %= 3  → m = {}", m); // 1

    // 비트wise 대입
    let mut a: u32 = 0b1010;
    a &= 0b0111;
    println!("   a &= 0b0111 → {:04b}", a);

    let mut b: u32 = 0b1010;
    b |= 0b0100;
    println!("   b |= 0b0100 → {:04b}", b);

    let mut c: u32 = 0b1010;
    c ^= 0b0110;
    println!("   c ^= 0b0110 → {:04b}", c);

    let mut d: u32 = 5;
    d <<= 2;
    println!("   d <<= 2   → {} (5 * 4 = 20)", d);

    let mut e: u32 = 20;
    e >>= 2;
    println!("   e >>= 2   → {} (20 / 4 = 5)", e);
}

// ============================================================
// Section 6: 범위 연산자 (Range Operators)
// ============================================================
//
// Rust의 범위는 JavaScript의 배열 slice와 유사하지만,
// 더 강력하고 타입 안전합니다.
//
// JavaScript                  Rust                  설명
// ──────────────────────────  ───────────────────   ─────────────────────────
// arr.slice(1, 3)              1..3                 [1, 3) - 3은 포함 안 됨
// arr.slice(1, 4)              1..4                 [1, 4) - 4는 포함 안 됨
// 1 <= x <= 10                1..=10              [1, 10] - 양쪽 포함
// for (let i=1; i<10; i++)     1..10              for_each(1..10)
//
// 범위 타입:
//   1..3    → Range<i32>       [1, 3)
//   1..=3   → RangeInclusive<&i32>  [1, 3]
//   ..3     → RangeTo<i32>     (-inf, 3)
//   3..     → RangeFrom<i32>   [3, +inf)
//   ..      → RangeFull        모든 범위

fn section_6_range_operators() {
    println!("Section 6 - Range Operators");

    // 닫힌 범위 [start, end] - 양쪽 포함
    // JavaScript: for (let i = 1; i <= 5; i++)
    // Rust: for i in 1..=5
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!(); // 1 2 3 4 5

    // 열린 범위 [start, end) - end 미포함
    // JavaScript: for (let i = 0; i < 5; i++)
    // Rust: for i in 0..5
    for i in 0..5 {
        print!("{} ", i);
    }
    println!(); // 0 1 2 3 4

    // range를 Vec으로 변환
    let numbers: Vec<i32> = (1..=10).collect();
    println!("   1..=10 as Vec: {:?}", numbers);

    // range를 사용하여 배열 인덱스 접근
    let arr = ["zero", "one", "two", "three", "four"];
    for i in 0..arr.len() {
        println!("   arr[{}] = {}", i, arr[i]);
    }

    // range로 문자 생성
    for c in 'a'..='z' {
        print!("{} ", c);
    }
    println!();

    // range로 짝수만 추출
    let evens: Vec<i32> = (0..=20).filter(|x| x % 2 == 0).collect();
    println!("   0~20 짝수: {:?}", evens);

    // range로 홀수만 추출 (2씩 건너뛰기)
    let odds: Vec<i32> = (1..20).step_by(2).collect();
    println!("   1~20 홀수 (2skip): {:?}", odds);
}

// ============================================================
// Section 7: 특수 연산자 (Special Operators)
// ============================================================
//
// Rust만의 고유 연산자와 자주 사용하는 연산자를 다룹니다.
//
// 연산자          설명                          JS/Java 비교
// ─────          ─────                          ─────────────────────────
// .              멤버 접근                      obj.property, obj.method()
// ::             네임스페이스 접근              Class.staticMethod()
// ?              try operator                  try/catch 대안
// as             타입 캐스트                    (int) value (Java)
// ->             화살표                         함수 반환/화살표 함수
// ||             클로저                         () => {} (JS)
// ..             구조체 스프레드                {...obj} (JS)
// _              와일드카드                     N/A
//
// ? 연산자 (Try Operator):
//   Result<T, E> 또는 Option<T>에서 에러/None이면 즉시 반환
//   JavaScript의 try/catch를 간결하게 대체
//
// as (캐스트):
//   Rust는 암시적 타입 변환이 거의 없음
//   명시적 as 필요: 10 as f64, 255u8 as u16

fn section_7_special_operators() {
    println!("Section 7 - Special Operators");

    // . 연산자: 멤버 접근
    let s = String::from("Hello, Rust!");
    println!("   s.len() = {}", s.len()); // 12
    println!("   s.to_uppercase() = {}", s.to_uppercase());

    // :: 연산자: 네임스페이스/정적 메서드 접근
    // JavaScript: Math.sqrt(16)
    // Java: Math.sqrt(16)
    // Rust: f64::sqrt(16.0)
    let sqrt = f64::sqrt(16.0);
    println!("   f64::sqrt(16.0) = {}", sqrt);

    // parseInt(42) vs "42".parse::<i32>()
    // JavaScript: parseInt("42") → 42
    // Rust: "42".parse::<i32>() → Ok(42)
    let parsed: Result<i32, _> = "42".parse();
    println!("   \"42\".parse() = {:?}", parsed);

    // ? 연산자 (Try Operator)
    // Result에서 Err이면 즉시 반환
    // JavaScript: try { return a / b; } catch(e) { return e; }
    // Rust: a / b? → Err이면 즉시 Err(e) 반환
    // section_7에서 ?를 쓰려면 함수가 Result를 반환해야 하므로
    // match로 대신 보여줍니다:
    fn divide_result(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("0으로 나누기 불가".to_string())
        } else {
            Ok(a / b)
        }
    }

    let result = match divide_result(10, 3) {
        Ok(v) => v,
        Err(e) => {
            println!("       Error: {}", e);
            0
        }
    };
    println!("   divide_result(10, 3) = {}", result);

    // as 캐스트
    let x: i32 = 42;
    let y: f64 = x as f64; // i32 → f64
    println!("   42 as f64 = {}", y);

    let z: u16 = 255u8 as u16; // u8 → u16
    println!("   255u8 as u16 = {}", z);

    let f: i32 = 42.7 as i32; // f64 → i32 (버림)
    println!("   42.7 as i32 = {} (버림!)", f);

    // -> 화살표 (함수 반환 타입)
    // JavaScript: const add = (a, b) => a + b;
    // Rust: let add = |a: i32, b: i32| -> i32 { a + b };
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("   add(5, 3) = {}", add(5, 3));

    // || 클로저
    let double = |x: i32| x * 2;
    println!("   double(21) = {}", double(21));

    // .. 구조체 스프레드
    // JavaScript: const copy = {...original};
    // Rust: Struct { ..original }
    struct Point { x: i32, y: i32, label: String }
    let p1 = Point { x: 1, y: 2, label: String::from("origin") };
    let p2 = Point { y: 10, ..p1 }; // x=1, label="origin" 유지
    println!("   p2: x={}, y={}, label={}", p2.x, p2.y, p2.label);

    // _ 와일드카드 (버린 값)
    // JavaScript: const [a, , c] = [1, 2, 3]; // 2 버림
    // Rust: let (a, _, c) = (1, 2, 3); // 2 버림
    let (a, _, c) = (1, 2, 3);
    println!("   tuple unpacking: a={}, c={} (middle ignored)", a, c);
}

// ============================================================
// Section 8: String 메서드 (&str / String)
// ============================================================
//
// Rust의 문자열은 &str (불변 슬라이스)와 String (가변 힙 소유) 두 타입이 있습니다.
// JavaScript의 String과 유사하지만, &str는 "참조만 함"이라는 점이 다릅니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// str.length              str.len()                 길이 (바이트 수)
// str.toUpperCase()       str.to_uppercase()        대문자 변환
// str.toLowerCase()       str.to_lowercase()        소문자 변환
// str.trim()              str.trim()                양쪽 공백 제거
// str.includes(x)         str.contains(x)           포함 여부
// str.startsWith(x)       str.starts_with(x)        시작 문자 확인
// str.endsWith(x)         str.ends_with(x)          끝 문자 확인
// str.split(x)            str.split(x)              분할 (Iterator 반환)
// str.replace(x, y)       str.replace(x, y)         교체
// str.substring(a, b)     &str[a..b]                슬라이스
// str.concat() / +        format!()                 결합
// str.repeat(n)           str.repeat(n)             반복

fn section_8_string_methods() {
    println!("Section 8 - String Methods");

    let greeting = "  Hello, Rust!  ";

    // 길이 (len)
    println!("   len(): {} 바이트", greeting.len());

    // trim (공백 제거)
    let trimmed = greeting.trim();
    println!("   trim(): \"{}\"", trimmed);

    // to_uppercase / to_lowercase
    println!("   to_uppercase(): {}", trimmed.to_uppercase());
    println!("   to_lowercase(): {}", trimmed.to_lowercase());

    // contains (포함 여부) - JS: str.includes()
    println!("   contains('Rust'): {}", trimmed.contains("Rust"));
    println!("   contains('Python'): {}", trimmed.contains("Python"));

    // starts_with / ends_with - JS: str.startsWith(), str.endsWith()
    println!("   starts_with('Hello'): {}", trimmed.starts_with("Hello"));
    println!("   ends_with('!'): {}", trimmed.ends_with("!"));

    // split (분할) - JS: str.split()
    let sentence = "Rust is fast, Rust is safe";
    let words: Vec<&str> = sentence.split(',').collect();
    println!("   split(','): {:?}", words);

    // replace (교체) - JS: str.replace()
    let replaced = trimmed.replace("Rust", "JavaScript");
    println!("   replace('Rust', 'JavaScript'): \"{}\"", replaced);

    // 슬라이싱 (substring) - JS: str.substring()
    let text = "Hello, Rust!";
    println!("   text[0..5]: \"{}\"", &text[0..5]);
    println!("   text[7..11]: \"{}\"", &text[7..11]);

    // chars (문자 단위 순회) - JS: [...str]
    for ch in "Rust".chars() {
        print!("{} ", ch);
    }
    println!();

    // repeat (반복) - JS: str.repeat()
    println!("   'Ha!'.repeat(3): {}", "Ha!".repeat(3));

    // format! (문자열 결합) - JS: `${a} ${b}`
    let first = "Hello";
    let second = "World";
    println!("   format: \"{}\"", format!("{} {}", first, second));
}

// ============================================================
// Section 9: 정수 & 부동 소수점 메서드
// ============================================================
//
// JavaScript의 number는 모두 float64이므로, 정수 전용 메서드가 없습니다.
// Rust는 타입별로 구체적인 메서드를 제공합니다.
//
// JavaScript          Rust                      설명
// ──────────────────  ────────────────────────  ─────────────────────────
// (없음)              x.to_string()             문자열 변환
// (없음)              x.to_binary()             2진수 문자열
// (없음)              x.to_octal()              8진수 문자열
// (없음)              x.to_hex()                16진수 문자열
// Math.floor()        x.floor()                 내림
// Math.ceil()         x.ceil()                  올림
// Math.round()        x.round()                 반올림
// Math.abs()          x.abs()                   절대값
// NaN                 x.is_nan()                NaN 체크
// Infinity            x.is_infinite()           무한대 체크
// Number.MAX_SAFE_INTEGER (없음)    정수 타입의 최대값

fn section_9_number_methods() {
    println!("Section 9 - Number Methods");

    // 부동 소수점 메서드 (f64)
    let pi: f64 = 3.14159265;

    // 내림 (floor) - JS: Math.floor()
    println!("   3.7.floor() = {}", 3.7_f64.floor());    // 3.0
    println!("   pi.floor() = {}", pi.floor());          // 3.0

    // 올림 (ceil) - JS: Math.ceil()
    println!("   3.1.ceil() = {}", 3.1_f64.ceil());    // 4.0
    println!("   pi.ceil() = {}", pi.ceil());          // 4.0

    // 반올림 (round) - JS: Math.round()
    println!("   3.4.round() = {}", 3.4_f64.round());  // 3.0
    println!("   3.5.round() = {}", 3.5_f64.round());  // 4.0
    println!("   pi.round() = {}", pi.round());        // 3.0

    // 절대값 (abs) - JS: Math.abs()
    println!("   -5.0.abs() = {}", (-5.0_f64).abs());  // 5.0
    println!("   pi.abs() = {}", pi.abs());            // 3.14159265

    // NaN 체크 - JS: Number.isNaN()
    let nan = f64::NAN;
    println!("   NaN.is_nan() = {}", nan.is_nan());    // true
    println!("   5.0.is_nan() = {}", 5.0_f64.is_nan()); // false

    // 무한대 체크 - JS: Number.isInfinity()
    let inf = f64::INFINITY;
    println!("   Infinity.is_infinite() = {}", inf.is_infinite()); // true

    // 정수 메서드 (i32)
    let x: i32 = 42;
    println!("   42.to_string() = \"{}\"", x.to_string());

    // 2진수 표현 - JS: (42).toString(2)
    println!("   42.to_binary() = {:b}", x);   // 101010

    // 8진수 표현 - JS: (42).toString(8)
    println!("   42.to_octal() = {:o}", x);   // 52

    // 16진수 표현 - JS: (42).toString(16)
    println!("   42.to_hex() = {:x}", x);    // 2a

    // 부호 체크
    println!("   42.is_positive() = {}", 42_i32.is_positive());
    println!("   -42.is_negative() = {}", (-42_i32).is_negative());
}

// ============================================================
// Section 10: Boolean 메서드
// ============================================================
//
// JavaScript의 boolean은 메서드가 거의 없지만,
// Rust의 bool은 유용한 메서드를 제공합니다.
//
// JavaScript          Rust                      설명
// ──────────────────  ────────────────────────  ─────────────────────────
// if (x) return f()  bool.then(f)              true일 때만 Some(f())
// if (x) return f()   if x { Some(f()) }        lazy 평가 (Rust 표준 패턴)
// x ? a : b           if x { a } else { b }     조건부 값 선택

fn section_10_boolean_methods() {
    println!("Section 10 - Boolean Methods");

    let is_rust_great = true;
    let is_python_cool = false;

    // then() - true일 때만 Some(value) 반환
    // JavaScript: if (isRustGreat) { return value; }
    let result = is_rust_great.then(|| "Rust is great!");
    println!("   true.then('Rust is great!') = {:?}", result);

    let result2 = is_python_cool.then(|| "Python is great!");
    println!("   false.then('...') = {:?}", result2);

    // lazy 평가 (값 생성 비용이 높을 때 유용) - JS: condition ? expensiveFn() : undefined
    // Rust: if-let 패턴으로 lazy 평가 구현
    let expensive = if is_rust_great {
        println!("       [비용이 큰 작업 실행!]");
        Some("computed")
    } else {
        None
    };
    println!("   if condition {{ Some(expensive()) }} = {:?}", expensive);

    let not_expensive = if is_python_cool {
        println!("       [이곳은 실행 안 됨]");
        Some("computed")
    } else {
        None
    };
    println!("   if !condition → None = {:?}", not_expensive);

    // 조건부 값 선택 - JS: x ? a : b
    let message: &str = if is_rust_great { "Rust is great!" } else { "N/A" };
    println!("   if condition ? \"a\" : \"b\" = \"{}\"", message);
}

// ============================================================
// Section 11: Option 메서드
// ============================================================
//
// JavaScript의 null/undefined를 안전하게 처리합니다.
// Java의 Optional과 유사하지만, 패턴 매칭과 결합되어 더 강력합니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// x ?? default            x.unwrap_or(default)      null 체크 + 기본값
// x ?? default            x.unwrap_or_else(f)       null 체크 + lazy 기본값
// x ? x.toString() : d    x.map_or(default, f)      Some일 때 변환
// (없음)                  x.flatten()               Option<Option<T>> → Option<T>
// (없음)                  x.transpose()             Option<Result<T,E>> → Result<Option<T>,E>
// !!x                     x.is_some()               값 존재 여부
// (없음)                  x.is_none()               None 여부
// (없음)                  x.expect(msg)             None이면 에러 메시지 표시

fn section_11_option_methods() {
    println!("Section 11 - Option Methods");

  let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    let _opt_lazy: Option<i32> = None;
    let opt_flat1: Option<Option<i32>> = Some(Some(42));
    let opt_flat2: Option<Option<i32>> = Some(None);
    let opt_trans1: Result<Option<i32>, String> = Ok(Some(42));
    let opt_trans2: Result<Option<i32>, String> = Err("failed".to_string());

    // unwrap() - Some에서 값 추출 (None이면 패닉)
    // JavaScript: x (null이면 에러!)
    println!("   Some(42).unwrap() = {}", some_value.unwrap());

    // unwrap_or() - 기본값 설정 (JS의 x ?? default와 동일)
    // JavaScript: const name = maybeName ?? 'Guest';
    // Rust: maybe_name.unwrap_or("Guest")
    println!("   Some(\"Alice\").unwrap_or(\"Guest\") = {}",
        Some("Alice".to_string()).unwrap_or("Guest".to_string()));
    println!("   None.unwrap_or(\"Guest\") = {}",
        None::<String>.unwrap_or("Guest".to_string()));

    // unwrap_or_else() - lazy 기본값 (값 생성 비용이 높을 때 유용)
    // JavaScript: const name = maybeName ?? (() => fetchDefault());
    let lazy_result = _opt_lazy.unwrap_or_else(|| {
        println!("       [lazy: 기본값 생성!]");
        999
    });
    println!("   None.unwrap_or_else(|| 999) = {}", lazy_result);

    // map() - Some일 때 변환
    let doubled = some_value.map(|x| x * 2);
    println!("   Some(42).map(x * 2) = {:?}", doubled);
    println!("   None.map(x * 2) = {:?}", none_value.map(|x| x * 2));

    // filter() - 조건 만족하는 Some만 유지
    let filtered = some_value.filter(|x| *x > 20);
    println!("   Some(42).filter(x > 20) = {:?}", filtered);
    let filtered2 = some_value.filter(|x| *x < 20);
    println!("   Some(42).filter(x < 20) = {:?}", filtered2);

    // is_some() / is_none()
    println!("   Some(42).is_some() = {}", some_value.is_some());
    println!("   None.is_some() = {}", none_value.is_some());
    println!("   None.is_none() = {}", none_value.is_none());

    // map_or() - 기본값 + 변환
    println!("   Some(42).map_or(0, |x| x * 2) = {}",
        some_value.map_or(0, |x| x * 2));
    println!("   None.map_or(0, |x| x * 2) = {}",
        none_value.map_or(0, |x| x * 2));

    // expect() - None이면 커스텀 에러
    // JavaScript: if (x === null) throw new Error("Not found");
    println!("   Some(42).expect(\"found!\") = {}",
        some_value.expect("found!"));

    // flatten() - Option<Option<T>> → Option<T>
    println!("   Some(Some(42)).flatten() = {:?}", opt_flat1.flatten());
    println!("   Some(None).flatten() = {:?}", opt_flat2.flatten());

    // transpose() - Option<Result<T, E>> → Result<Option<T>, E>
    // Option<Result>를 Result<Option>으로 뒤집습니다
    println!("   Ok(Some(42)).transpose() = {:?}", opt_trans1.transpose());
    println!("   Err(\"..\").transpose() = {:?}", opt_trans2.transpose());
}

// ============================================================
// Section 12: Result 메서드
// ============================================================
//
// Rust의 Result는 JavaScript의 try/catch를 대체합니다.
// 컴파일타임에 에러 처리를 강제하여 "에러를 잊는" 상황을 방지합니다.
//
// JavaScript                  Rust                      설명
// ──────────────────────────  ────────────────────────  ─────────────────────────
// try { ... } catch (e) { ... }  x.unwrap()            Ok에서 값 추출
// (없음)                  x.unwrap_or(default)          Ok면 값, Err면 기본값
// (없음)                  x.unwrap_or_else(f)         Err이면 함수 호출
// (없음)                  x.map(f)                    Ok일 때 값 변환
// (없음)                  x.map_err(f)                Err일 때 에러 변환
// x instanceof Error        x.is_err()                실패 여부
// (없음)                  x.is_ok()                   성공 여부
// (없음)                  x.ok()                      Result → Option

fn section_12_result_methods() {
    println!("Section 12 - Result Methods");

    // 각 메서드를 독립적으로 보여주기 위해 별도 변수 생성
    let ok1: Result<i32, String> = Ok(42);
    let _err1: Result<i32, String> = Err("parse failed".to_string());
    let ok2: Result<i32, String> = Ok(42);
    let err2: Result<i32, String> = Err("parse failed".to_string());
    let _ok3: Result<i32, String> = Ok(42);
    let err3: Result<i32, String> = Err("parse failed".to_string());
    let ok4: Result<i32, String> = Ok(42);
    let err4: Result<i32, String> = Err("parse failed".to_string());
    let _ok5: Result<i32, String> = Ok(42);
    let err5: Result<i32, String> = Err("parse failed".to_string());
    let ok6: Result<i32, String> = Ok(42);
    let err6: Result<i32, String> = Err("parse failed".to_string());

    // unwrap() - Ok에서 값 추출 (Err이면 패닉)
    // JavaScript: try { return value; } catch (e) { throw e; }
    println!("   Ok(42).unwrap() = {}", ok1.unwrap());

    // unwrap_or() - Err이면 기본값
    // JavaScript: value ?? default
    println!("   Ok(42).unwrap_or(0) = {}", ok2.unwrap_or(0));
    println!("   Err(...).unwrap_or(0) = {}", err2.unwrap_or(0));

    // unwrap_or_else() - Err이면 함수 호출 (lazy)
    // JavaScript: value ?? (() => compute())
    let r = err3.unwrap_or_else(|e| {
        println!("       [에러 처리: {}]", e);
        -1
    });
    println!("   Err.unwrap_or_else = {}", r);

    // map() - Ok일 때 값 변환
    // JavaScript: try { return fn(value); } catch (e) { return e; }
    let mapped = ok4.map(|x| x * 10);
    println!("   Ok(42).map(x * 10) = {:?}", mapped);
    let mapped_err = err4.map(|x| x * 10);
    println!("   Err(...).map(x * 10) = {:?}", mapped_err);

    // map_err() - Err일 때 에러 변환
    let mapped_err2 = err5.map_err(|e| e.len());
    println!("   Err(\"...\".map_err(len) = {:?}", mapped_err2);

    // is_ok() / is_err()
    // JavaScript: (result instanceof Error)
    println!("   Ok(42).is_ok() = {}", ok6.is_ok());
    let err5b: Result<i32, String> = Err("parse failed".to_string());
    println!("   Err(...).is_ok() = {}", err5b.is_ok());
    println!("   Err(...).is_err() = {}", err5b.is_err());

    // ok() - Result<T, E> → Option<T>
    let opt = ok6.ok();
    println!("   Ok(42).ok() = {:?}", opt);
    let opt2 = err6.ok();
    println!("   Err(...).ok() = {:?}", opt2);
}

// ============================================================
// Section 13: Vec / Array 메서드
// ============================================================
//
// JavaScript Array와 Rust Vec을 비교합니다.
// 가장 자주 사용하는 메서드를 엄선했습니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// arr.length              vec.len()               길이
// arr.push(x)             vec.push(x)             끝에 추가
// arr.pop()               vec.pop()               끝에서 제거
// arr.unshift(x)          vec.insert(0, x)        시작에 추가
// arr.shift()             vec.remove(0)           시작에서 제거
// arr.splice(a, b)            vec.splice(a, b, ...)   영역 제거/교체
// arr.includes(x)         vec.contains(&x)        포함 여부
// arr.sort()              vec.sort()              정렬
// arr.sort((a,b) => ...)  vec.sort_by(...)        커스텀 정렬
// arr.reverse()           vec.reverse()           역순
// arr.slice(a, b)         &vec[a..b]              슬라이스
// [...arr]                vec.clone()             복사
// arr[0]                  vec[0] / vec.first()    첫 요소
// arr[arr.length-1]       vec.last()              마지막 요소
// arr.find(x => ...)      vec.iter().find(...)    조건 만족 요소

fn section_13_vec_array_methods() {
    println!("Section 13 - Vec / Array Methods");

    let mut numbers: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let _arr = ["apple", "banana", "cherry"];
    // JS: arr.forEach(x => console.log(x));
    // Rust: for x in &arr { println!("{}", x); }
    for item in _arr {
        print!("{} ", item);
    }
    println!();

    // len() - JS: arr.length
    println!("   numbers.len() = {}", numbers.len());

    // push() - JS: arr.push()
    numbers.push(10);
    println!("   push(10): {:?}", numbers);

    // pop() - JS: arr.pop()
    if let Some(last) = numbers.pop() {
        println!("   pop() = {} → {:?}", last, numbers);
    }

    // insert() - JS: arr.unshift() / arr.splice()
    numbers.insert(0, 0);
    println!("   insert(0, 0): {:?}", numbers);

    // remove() - JS: arr.splice(i, 1)
    let removed = numbers.remove(1);
    println!("   remove(1) = {} → {:?}", removed, numbers);

    // contains() - JS: arr.includes()
    println!("   contains(5): {}", numbers.contains(&5));
    println!("   contains(99): {}", numbers.contains(&99));

    // sort() - JS: arr.sort()
    numbers.sort();
    println!("   sort(): {:?}", numbers);

    // sort_by() - JS: arr.sort((a, b) => ...)
    let mut words = vec!["banana", "Apple", "cherry"];
    words.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("   sort_by lowercase: {:?}", words);

    // reverse() - JS: arr.reverse()
    let mut rev = vec![1, 2, 3, 4, 5];
    rev.reverse();
    println!("   reverse(): {:?}", rev);

    // slice(&vec[a..b]) - JS: arr.slice(a, b)
    let sliced = &numbers[2..5];
    println!("   numbers[2..5]: {:?}", sliced);

    // clone() - JS: [...arr]
    let cloned = numbers.clone();
    println!("   clone(): {:?}", cloned);

    // first() / last() - JS: arr[0] / arr[arr.length-1]
    println!("   first(): {:?}", numbers.first());
    println!("   last(): {:?}", numbers.last());

    // find() - JS: arr.find(x => x > 5)
    if let Some(found) = numbers.iter().find(|x| **x > 5) {
        println!("   find(x > 5): {}", found);
    }
}

// ============================================================
// Section 14: HashMap 메서드
// ============================================================
//
// JavaScript Map과 Rust HashMap을 비교합니다.
// HashMap은 키-값 쌍을 O(1)로 탐색하는 해시 테이블입니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// map.size                map.len()                 크기
// map.set(k, v)           map.insert(k, v)          키-값 추가
// map.get(k)              map.get(k)                값 조회 (Option)
// map.delete(k)           map.remove(k)             키-값 제거
// map.has(k)              map.contains_key(k)       키 존재 여부
// map.keys()              map.keys()                키 Iterator
// map.values()            map.values()              값 Iterator
// for ([k,v] of map)      for (k, v) in &map        키-값 순회
// map.clear()             map.clear()               비우기

fn section_14_hashmap_methods() {
    println!("Section 14 - HashMap Methods");

    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();

    // insert() - JS: map.set()
    scores.insert("Rust".to_string(), 95);
    scores.insert("JavaScript".to_string(), 85);
    scores.insert("Python".to_string(), 90);
    println!("   insert 3 entries: {:?}", scores);

    // len() - JS: map.size
    println!("   len() = {}", scores.len());

    // get() - JS: map.get() → 값 또는 undefined
    // Rust: Option<&T> 반환
    if let Some(score) = scores.get("Rust") {
        println!("   get('Rust') = {}", score);
    }
    println!("   get('Go') = {:?}", scores.get("Go")); // None

    // contains_key() - JS: map.has()
    println!("   contains_key('Rust'): {}", scores.contains_key("Rust"));
    println!("   contains_key('Go'): {}", scores.contains_key("Go"));

    // remove() - JS: map.delete()
    let removed = scores.remove("JavaScript");
    println!("   remove('JavaScript') = {:?}", removed);
    println!("   after remove: {:?}", scores);

    // keys() - JS: map.keys()
    println!("   keys:");
    for key in scores.keys() {
        print!("   {} ", key);
    }
    println!();

    // values() - JS: map.values()
    println!("   values: {:?}", scores.values().collect::<Vec<_>>());

    // iter() - JS: for ([k, v] of map)
    println!("   all entries:");
    for (language, score) in &scores {
        println!("     {} = {}", language, score);
    }

    // clear() - JS: map.clear()
    scores.clear();
    println!("   clear() → {:?}", scores);
}

// ============================================================
// Section 15: Iterator 메서드
// ============================================================
//
// Rust의 Iterator는 JavaScript의 배열 메서드보다 훨씬 강력합니다.
// Lazy (지연 평가)로, collect()에서만 실제로 실행됩니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// arr.map(f)              iter().map(f).collect()   변환
// arr.filter(f)           iter().filter(f).collect()  필터링
// arr.reduce(f, init)     iter().fold(init, f)      축적
// arr.forEach(f)          iter().for_each(f)        반복 실행
// arr.find(f)             iter().find(f)            조건 만족 첫 요소
// arr.some(f)             iter().any(f)             하나라도 만족
// arr.every(f)            iter().all(f)             모두 만족
// arr.slice(a, b)         iter().skip(a).take(b)    슬라이스
// arr.concat(arr2)        iter1.chain(iter2)        연결
// [...arr1, ...arr2]      iter1.chain(iter2)        연결
// arr.entries()           iter().enumerate()        인덱스+값
// arr.indexOf(x)          iter().position(|&x|...)  인덱스 찾기

fn section_15_iterator_methods() {
    println!("Section 15 - Iterator Methods");

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map() - JS: arr.map(x => x * 2)
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   map(x*2): {:?}", doubled);

    // filter() - JS: arr.filter(x => x > 5)
    let large: Vec<&i32> = numbers.iter().filter(|x| **x > 5).collect();
    println!("   filter(x > 5): {:?}", large);

    // fold() - JS: arr.reduce((acc, x) => acc + x, 0)
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("   fold(sum) = {}", sum);

    // sum() - JS: arr.reduce((a,b) => a+b, 0)
    let sum_short = numbers.iter().sum::<i32>();
    println!("   sum() = {}", sum_short);

    // product() - JS: 없음 (reduce로 구현)
    let product: i32 = (1..=5).product();
    println!("   1~5 product() = {}", product);

    // count() - JS: arr.filter().length
    let count_above_5 = numbers.iter().filter(|x| **x > 5).count();
    println!("   filter(> 5).count() = {}", count_above_5);

    // find() - JS: arr.find(x => x > 7)
    if let Some(found) = numbers.iter().find(|x| **x > 7) {
        println!("  find(x > 7) = {}", found);
    }

    // any() - JS: arr.some(x => x > 7)
    println!("   any(x > 7): {}", numbers.iter().any(|x| *x > 7));

    // all() - JS: arr.every(x => x > 0)
    println!("   all(x > 0): {}", numbers.iter().all(|x| *x > 0));

    // for_each() - JS: arr.forEach(x => console.log(x))
    print!("   for_each(first 3): ");
    numbers.iter().take(3).for_each(|x| print!("{} ", x));
    println!();

    // skip() / take() - JS: arr.slice(a, b)
    let skipped: Vec<&i32> = numbers.iter().skip(3).take(4).collect();
    println!("   skip(3).take(4): {:?}", skipped);

    // chain() - JS: [...arr1, ...arr2]
    let more = vec![11, 12, 13];
    let chained: Vec<&i32> = numbers.iter().chain(more.iter()).collect();
    println!("   chain: {:?}", chained);

    // enumerate() - JS: arr.entries()
    print!("   enumerate(first 3): ");
    for (i, val) in numbers.iter().enumerate().take(3) {
        print!("[{}]:{} ", i, val);
    }
    println!();

    // position() - JS: arr.indexOf(5)
    if let Some(pos) = numbers.iter().position(|x| *x == 5) {
        println!("   position(5) = {}", pos);
    }

    // next() - 수동으로 Iterator 소비
    let mut iter = numbers.iter();
    println!("   next() = {:?}", iter.next());
    println!("   next() = {:?}", iter.next());
}

// ============================================================
// Section 16: char 메서드
// ============================================================
//
// Rust의 char은 유니코드 스칼라 (4바이트)로,
// JavaScript의 String[charAt()]보다 정확하고 안전합니다.
//
// JavaScript              Rust                      설명
// ──────────────────────  ────────────────────────  ─────────────────────────
// str.charAt(i)           str.chars().nth(i)        문자 추출
// str.charCodeAt(i)       char as u32               코드 포인트
// isNaN                   char.is_alphabetic()      알파벳 여부
// (없음)                  char.is_digit(10)         10진수Digit
// (없음)                  char.is_numeric()         숫자
// (없음)                  char.is_lowercase()       소문자
// (없음)                  char.is_uppercase()       대문자
// (없음)                  char.to_uppercase()       대문자 변환
// (없음)                  char.to_lowercase()       소문자 변환

fn section_16_char_methods() {
    println!("Section 16 - char Methods");

    let c = 'a';
    let d = 'Z';
    let korean = '한';
    let emoji = '🦀';

    // is_digit() - JS: /\d/.test(char)
    println!("   '7'.is_digit(10): {}", '7'.is_digit(10));
    println!("   '7'.is_digit(16): {}", '7'.is_digit(16)); // 16진수도 true
    println!("   'A'.is_digit(10): {}", 'A'.is_digit(10)); // 10진수는 false

    // is_alphabetic() - JS: /[a-zA-Z]/.test(char)
    println!("   'a'.is_alphabetic(): {}", c.is_alphabetic());
    println!("   '한'.is_alphabetic(): {}", korean.is_alphabetic());
    println!("   '7'.is_alphabetic(): {}", '7'.is_alphabetic());
    println!("   '!'.is_alphabetic(): {}", '!'.is_alphabetic());

    // is_numeric() - JS: /[0-9]/.test(char)
    println!("   '7'.is_numeric(): {}", '7'.is_numeric());
    println!("   '한'.is_numeric(): {}", korean.is_numeric());

    // is_lowercase() / is_uppercase()
    println!("   'a'.is_lowercase(): {}", c.is_lowercase());
    println!("   'Z'.is_uppercase(): {}", d.is_uppercase());

    // to_uppercase() / to_lowercase()
    println!("   'a'.to_uppercase() = \"{}\"", c.to_uppercase().collect::<String>());
    println!("   'Z'.to_lowercase() = \"{}\"", d.to_lowercase().collect::<String>());
    println!("   '한'.to_uppercase() = \"{}\"", korean.to_uppercase().collect::<String>());

    // 문자열에서 char 순회
    print!("   'Rust'.chars: ");
    for ch in "Rust".chars() {
        print!("{}({:04x}) ", ch, ch as u32);
    }
    println!();

    // ASCII 여부
    println!("   'R'.is_ascii(): {}", 'R'.is_ascii());
    println!("   '한'.is_ascii(): {}", korean.is_ascii());
    println!("   '🦀'.is_ascii(): {}", emoji.is_ascii());
}

// ============================================================
// 메인 함수 - 모든 섹션 실행
// ============================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================");
    println!("  Rust 연산자 & 메서드 완전 정복");
    println!("============================================\n");

    section_1_arithmetic_operators();
    println!();

    section_2_comparison_operators();
    println!();

    section_3_logical_operators();
    println!();

    section_4_bitwise_operators();
    println!();

    section_5_assignment_operators();
    println!();

    section_6_range_operators();
    println!();

    section_7_special_operators();
    println!();

    section_8_string_methods();
    println!();

    section_9_number_methods();
    println!();

    section_10_boolean_methods();
    println!();

    section_11_option_methods();
    println!();

    section_12_result_methods();
    println!();

    section_13_vec_array_methods();
    println!();

    section_14_hashmap_methods();
    println!();

    section_15_iterator_methods();
    println!();

    section_16_char_methods();

    println!("\n============================================");
    println!("All 16 sections complete!");
    println!("============================================");

    Ok(())
}
