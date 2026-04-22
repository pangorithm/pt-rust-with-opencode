// # Rust 튜토리얼 - JavaScript 개발자를 위한 Rust 문법 가이드
//
// 이 프로젝트는 JavaScript (ES6+) 개발자들이 Rust의 핵심 문법과 개념을 빠르게
// 익힐 수 있도록 도와줍니다.
// 각 섹션은 main()에서 순차적으로 호출되는 독립적인 함수입니다.
//
// ============================================================
// JavaScript 개발자를 위한 핵심 비교표
// ============================================================
//
// JavaScript                      Rust
// ─────────────────────────────   ─────────────────────────────────
// 가비지 컬렉션 (GC)              소유권 시스템 (Ownership)
// 런타임 타입                     정적 타입 (컴파일타임)
// 힙/스택 구분 없음               스택/힙 명확히 구분
// Promise/async-await             async/await + tokio 런타임
// npm (package.json)              Cargo (Cargo.toml)
// module.exports / import-export  mod / pub / use
// try/catch                       Result<T, E> + ? 연산자
// undefined / null                Option<T> + Result<T, E>
//
// Rust의 가장 중요한 철학 세 가지:
// 1. 메모리 안전성: 가비지 컬렉션 없이 소유권으로 메모리 안전성 보장
// 2. 동시성 안전성: 컴파일타임에 데이터 경합(Data Race) 방지
// 3. 제로-cost 추상화: 고수준 추상화가 낮은 수준 성능을 해치지 않음

// ============================================================
// Section 1: 변수 선언 (Variable Declarations)
// ============================================================
//
// Rust의 `let`/`mut`은 JavaScript의 `let`/`const`에 해당합니다.
//
// JavaScript의 변수 선언과 Rust의 비교:
//   JavaScript                Rust                      설명
//   ──────────────────────    ──────────────────────    ────────────────────────
//   const x = 42;             let x: i32 = 42;          재할당 불가 (기본 불변)
//   let y = 42;                let mut y: i32 = 42;      재할 assignment 가능 (mut 필요)
//   let z;                    ERROR!                    Rust는 선언만 허용하지 않음
//
// 핵심 차이점:
// - JavaScript의 `const`는 재할당 불가지만, 객체/배열은 변경 가능 (얕은 불변)
//   JavaScript: `const arr = [1,2,3]; arr.push(4); // 가능!`
// - Rust의 `let`은 완전한 불변 (deep immutability)
//   Rust: `let arr = vec![1,2,3]; arr.push(4); // ERROR! mut 필요`
// - JavaScript는 타입이 없습니다 (dynamic typing)
// - Rust는 타입을 명시하거나 강력하게 추론합니다 (static typing)
// - Rust에서 변수는 반드시 초기화되어야 합니다 (선언 후 할당 불가)
//
// 메모리 레이아웃 (스택):
//   i32, f64, bool, char 같은 기본 타입은 스택에 직접 저장됩니다.
//   - i32: 4바이트, f64: 8바이트, bool: 1바이트, char: 4바이트 (유니코드)
//   스택 할당은 매우 빠릅니다 (포인터터 조작만 하면 됨)

fn section_1_variable_declarations() {
    // JavaScript의 `const`와 유사: 재할당 불가
    let immutable_value: i32 = 42;
    // immutable_value = 100; // ERROR! 재할당 불가
    // JavaScript: const x = 42; x = 100; // TypeError!
    // Rust의 불변은 JavaScript의 const보다 엄격합니다.
    // JavaScript의 const는 객체의 내용까지 불변이 아니지만,
    // Rust의 let은 완전히 불변입니다.
    //
    // Rust의 변수 섀도잉 (Shadowing):
    //   JavaScript: let x = 1; let x = 2; // x는 2 (섀도잉 허용)
    //   Rust:      let x = 1; let x = 2; // x는 2 (섀도잉 허용!)
    //   Rust는 JavaScript와 마찬가지로 같은 스코프 내에서 변수 이름 재사용(섀도잉)이 가능합니다.
    //   하지만 let immutable_value = 100;은 새로운 변수를 생성하는 것이지,
    //   immutable_value에 재할당하는 것이 아닙니다 (재할당 불가!).
    //
    // JavaScript의 const vs Rust의 let 불변:
    //   JavaScript: const obj = { x: 1 }; obj.x = 2; // 가능 (얕은 불변)
    //   Rust:       let obj = MyStruct { x: 1 }; obj.x = 2; // ERROR (완전한 불변)
    //   JavaScript의 const는 "레퍼런스 재할당"만 막고, 객체 내용은 변경 가능하지만,
    //   Rust의 let은 "값 자체의 변경"을 완전히 막습니다.
    //   따라서 Rust에서 "변경 가능한 데이터"를 만들고 싶으면 String, Vec 등 Move 타입을 사용하고,
    //   변경하려면 mut 키워드를 명시해야 합니다.

    // JavaScript의 `let`과 같은 재할당 (mut 사용)
    let mut mutable_value: i32 = 42;
    mutable_value = 100; // OK! 재할당 가능
    // JavaScript: let x = 42; x = 100; // 가능
    // Rust의 mut은 JavaScript의 let과 유사한 "변경 가능" semantics을 가집니다.
    // 다만 JavaScript에서는 모든 변수가 기본적으로 변경 가능하지만,
    // Rust에서는 mut을 명시해야 변경 가능합니다.
    // 이 차이는 Rust가 불변성을 기본으로 하여 실수를 방지하기 위한 설계입니다.
    println!(
        "Section 1 - Variable Declarations: mutable = {}",
        mutable_value
    );

    // 타입 추론 - Rust는 타입을 명시하지 않으면 자동으로 추론
    let inferred_number = 42; // 자동으로 i32 추론
    // JavaScript: let x = 42; // number (모든 숫자는 float64)
    // Rust: let x = 42; // i32 (부호 있는 32비트 정수)
    // Rust의 타입 추론은 JavaScript와 다릅니다:
    // - JavaScript는 런타임에 타입을 결정 (number, string, boolean 등)
    // - Rust는 컴파일타임에 타입을 결정 (i32, i64, f32, f64 등)
    // - Rust는 "어떤 타입일까요?"를 컴파일러가 추론
    // - JavaScript는 "어떤 타입인가요?"를 개발자가 신경 써야 함 (런타임)
    //
    // JavaScript의 number는 모든 숫자를 다루지만 (정수, 부동소수점, bigint),
    // Rust에서는 정수와 부동소수점이 완전히 분리되어 있습니다.
    // 이 분리는 메모리 효율과 정밀도를 보장합니다:
    //   let x = 42;       // Rust: i32 (4바이트), JS: number (8바이트 float64)
    //   let x = 42.0;     // Rust: f64 (8바이트), JS: number (8바이트 float64)
    //   let x = 42_i64;   // Rust: i64 (8바이트, 명시적 타입 지정)
    //   let x = 42_f32;   // Rust: f32 (4바이트, 명시적 타입 지정)
    let inferred_float = 3.14_f64; // 명시적으로 f64
    // _f64는 "타입 어노테이션" 구문입니다.
    // 타입 어노테이션은 컴파일러에게 "이 값의 타입이 f64입니다"라고 알려줍니다.
    // JavaScript: let x = 3.14; // 자동으로 float64
    let inferred_string = "hello"; // 자동으로 &str (문자열 슬라이스) 추론
    // &str은 "문자열 슬라이스" 타입입니다.
    // 이는 메모리 어딘가에 있는 문자열의 일부를 "참조"만 하는 타입입니다.
    // 데이터 자체를 포함하지 않고, (포인터, 길이)만 포함합니다.
    // JavaScript의 String과 비슷하지만, "슬라이스"라는 점이 다릅니다.
    //
    // &str vs JavaScript String의 근본적 차이:
    //   JavaScript String: 데이터 자체를 보유 (힙 또는 인라인)
    //     const s = "hello"; // s가 "hello" 데이터를 직접 보유
    //   Rust &str: 데이터를 "참조"만 함 (소유자 아님)
    //     let s: &str = "hello"; // s는 "hello"가 저장된 위치를 가리킴
    //   JavaScript에서 substring()은 새 String을 생성하지만,
    //   Rust에서 &str[0..3]은 기존 데이터의 일부만 가리킴 (복사 없음).
    //
    // "hello" 리터럴의 메모리 위치:
    //   프로그램의 읽기 전용 데이터 섹션 (.rodata)에 저장됨
    //   이 데이터는 프로그램 전체 수명(static lifetime)을 가짐
    //   따라서 "hello"에 대한 &str은 어디서든 안전하게 사용 가능
    println!(
        "   Type Inference: {} (i32), {} (f64), {} (&str)",
        inferred_number, inferred_float, inferred_string
    );

    // JavaScript의 `const`처럼 항상 불변으로 유지하고 싶다면 `mut`을 추가하지 마세요
    // 이것이 Rust의 철학입니다: 기본적으로 불변, 변경하려면 명시적으로 `mut` 선언
    //
    // JavaScript 개발자를 위한 설명:
    // JavaScript는 기본적으로 모든 변수가 변경 가능합니다 (let).
    // const는 재할당만 막을 뿐 객체 내용은 변경 가능합니다.
    // Rust는 기본적으로 모든 변수가 불변입니다 (let).
    // mut을 추가해야 변경 가능합니다.
    // 이 차이는 Rust가 불변성을 기본으로 하여 버그를 줄이기 때문입니다.
}

// ============================================================
// Section 2: 기본 타입 (Primitive Types)
// ============================================================
//
// JavaScript의 타입 시스템은 간단합니다:
//   number (모든 숫자 = float64), string, boolean, null, undefined, symbol, bigint
// Rust의 타입 시스템은 명확하고 세분화되어 있습니다:
//
// JavaScript의 number는 "모든 숫자"를 다루지만, Rust는 정확한 타입을 사용합니다:
//
// JavaScript          Rust              메모리          범위
// ──────────────────  ────────────────  ──────────────  ─────────────────────
// number              i32               4 bytes         -2^31 ~ 2^31-1
// number              i64               8 bytes         -2^63 ~ 2^63-1
// number              u32               4 bytes         0 ~ 2^32-1
// number              u64               8 bytes         0 ~ 2^64-1
// number              f64               8 bytes         IEEE 754 double
// number              f32               4 bytes         IEEE 754 float
// "hello" (1 char)    char               4 bytes         유니코드 코드 포인트
// NaN                 NaN               8 bytes         f64만 가짐
// Infinity            Infinite          8 bytes         f64만 가짐
//
// JavaScript는 런타임에 타입을 결정 (dynamic typing)하므로:
//   typeof 42        → "number"
//   typeof "42"      → "string"
//   typeof undefined → "undefined"
// Rust는 컴파일타임에 타입을 결정 (static typing)하므로:
//   let x: i32 = 42;  // 컴파일러가 100% 타입을 알고 있음
//   let y: f64 = 3.14; // i32와 f64를 혼동할 수 없음
//
// JavaScript의 number는 float64이므로 정수 연산에 오차가 발생할 수 있습니다:
//   0.1 + 0.2 === 0.3  → false! (0.30000000000000004)
// Rust의 f64도 같은 IEEE 754 표준을 사용하므로 같은 오차가 있습니다.
// 정수 연산은 항상 정확합니다 (i32, u32 등).

fn section_2_primitive_types() {
    // 정수 타입 - JavaScript의 `number`와 달리 크기를 명시합니다
    let signed: i32 = -100; // -2^31 to 2^31-1 (JavaScript의 number와 유사한 범위)
    // i32는 JavaScript의 number가 다루는 정수 범위와 거의 동일합니다.
    // JavaScript의 number는 float64이므로 정수 부분만 보면 2^53까지 정확합니다 (safe integer).
    // Rust의 i32는 2^31까지만 다룰 수 있지만, 메모리를 더 적게 사용합니다.
    let unsigned: u32 = 100; // 0 to 2^32-1 (음수 불가)
    // u32는 음수를 표현할 수 없습니다.
    // JavaScript에는 unsigned 타입이 없습니다 (모든 number는 부호 있음).
    // 인덱스, 카운터 등 음수가 필요 없는 곳에 u32를 사용하면 명확성이 향상됩니다.
    let big: i128 = 999_999_999_999; // 읽기 쉬움을 위한 언더스코어 (JavaScript에서도 가능)
    // 언더스코어(_)는 가독성을 위한 구문일 뿐, 값에는 영향을 주지 않습니다.
    // JavaScript ES6에서도 999_999_999_999를 사용할 수 있습니다.
    println!(
        "Section 2 - Integers: {} (i32), {} (u32), {} (i128)",
        signed, unsigned, big
    );

    // 부동 소수점 - JavaScript의 `number`는 모두 float64이지만, Rust에서는 선택 가능
    let float32: f32 = 3.14; // 32-bit 부동 소수점
    // f32는 메모리를 덜 사용하지만 정밀도가 낮습니다.
    // JavaScript의 number는 모두 f64이므로, f32는 Rust만의 옵션입니다.
    let float64: f64 = 3.141592653589793; // 64-bit (JavaScript의 number와 동일)
    // f64은 JavaScript의 number와 정확히 동일한 IEEE 754 64비트 부동 소수점입니다.
    println!("   Float: {} (f32), {} (f64)", float32, float64);

    // 부울 - JavaScript의 `true`/`false`와 동일
    let is_rust_great: bool = true; // JavaScript의 `true`와 동일
    let is_javascript_cool: bool = false;
    // JavaScript의 true/false와 완전히 동일합니다.
    // 다만, JavaScript에서는 "truthy/falsy" 개념이 있어:
    //   0, "", null, undefined, NaN, false → falsy
    //   나머지는 모두 truthy
    // Rust에는 truthy/falsy 개념이 없습니다.
    //   if 0 { ... } // ERROR! Rust는 bool만 허용
    //   if true { ... } // OK!
    // Rust의 bool는 오직 true 또는 false만 가능합니다.
    println!(
        "   Boolean: {} (Is Rust great? {})",
        is_javascript_cool, is_rust_great
    );

    // 문자 - JavaScript의 문자는 String이지만, Rust는 별도의 char 타입을 가집니다
    // JavaScript: `'a'`과 `"a"` 모두 String (차이 없음)
    //   typeof 'a' → "string"
    //   typeof "a" → "string"
    // Rust: `'a'`은 char (유니코드 스칼라, 4바이트)
    //   'a' → char 타입 (1 글자, UTF-8 코드 포인트)
    //   "a" → &str 타입 (문자열 슬라이스)
    // JavaScript에는 char 타입이 없습니다. 모든 문자가 String입니다.
    let first_letter: char = 'R';
    let emoji: char = '🦀'; // Rust의 게 마스코트!
    // '🦀'은 Rust의 공식 마스코트입니다 (crab).
    // Unicode 코드 포인트 U+1F980으로, 4바이트입니다.
    // JavaScript의 String은 UTF-16 코드 유닛을 사용하지만,
    // Rust의 char는 UTF-8 코드 포인트를 직접 저장합니다.
    println!("   Char: '{}' (char), '{}' (emoji)", first_letter, emoji);
}

// ============================================================
// Section 3: 문자열 (Strings)
// ============================================================
//
// JavaScript와 Rust의 문자열은 근본적으로 다릅니다.
// 이 차이는 Rust가 메모리를 어떻게 관리하는지와 직접 연결되어 있습니다.
//
// JavaScript의 String:
//   - 불변 (immutable) - 한 번 생성하면 변경 불가
//   - 메서드 호출 시 항상 새로운 String 객체를 반환
//   - JavaScript: `'hello'.toUpperCase() → 'HELLO'` (새 객체)
//   - 모든 문자열이 힙에 저장됨 (엔진에 따라 다름)
//   - UTF-16 코드 유닛으로 인코딩됨
//
// Rust의 두 문자열 타입:
//   String   → 가변 (mutable), 힙에 저장, 소유권을 가짐
//   &str     → 불변 (immutable), 문자열 슬라이스, 참조만 가짐
//
// 메모리 레이아웃 비교:
//
// JavaScript String:
//   힙: [길이][UTF-16 코드 유닛 1][UTF-16 코드 유닛 2][...]
//   스택: 힙 포인터
//
// Rust String:
//   힙: [길이][용량][UTF-8 바이트 1][UTF-8 바이트 2][...]
//   스택: 힙 포인터 + 길이 + 용량 (3포인터 = 24바이트)
//
// Rust &str:
//   스택: 힙 포인터 + 길이 (2포인터 = 16바이트)
//   데이터 자체는 힙에 있음 (String이 소유)
//
// 왜 Rust는 두 타입을 구분할까요?
//   1. 성능: &str은 데이터를 복사하지 않으므로 매우 빠름
//   2. 명확성: 소유권 vs 참조를 코드에서 명확히 볼 수 있음
//   3. 메모리 효율: 함수 파라미터로 &str을 받으면 복사 없이 참조만 받음
//   4. 안전성: &str은 항상 유효한 UTF-8임을 보장

fn section_3_strings() {
    // &str: 컴파일 타임에 알려진 불변 문자열 슬라이스 (문자열 참조)
    // JavaScript의 String에 가장 가까운 개념이지만, "슬라이스" 구분이 중요합니다.
    //
    // 슬라이스(&str)는 기존 문자열(String)의 일부 또는 전체를 "참조"만 하는 타입으로,
    // 데이터 자체를 복사하지 않고 포인터(메모리 주소)와 길이만 보유합니다.
    //
    // JavaScript의 String과 &str의 차이:
    //   JavaScript: `'hello'.substring(0, 3)` → 새 String 객체 생성
    //   Rust: "hello".substring(0..3) → &str (참조만 생성, 복사 없음)
    //
    // 성능 비교:
    //   JavaScript: substring()은 새 객체를 생성하므로 메모리 할당 + 복사 발생
    //   Rust: &str은 포인터와 길이만 복사하므로 스택 작업만 발생 (나노초 단위)
    //
    // String::substring()이 새 객체를 생성하는 반면, &str[0..5]는 참조만 생성하므로 성능이 우수합니다.
    let string_slice: &str = "Hello, Rust!";
    // "Hello, Rust!"는 소스 코드에 직접 작성된 문자열 리터럴입니다.
    // 이는 컴파일 타임에 알려져 있으며, 프로그램의 읽기 전용 데이터 섹션에 저장됩니다.
    // &string_slice는 이 데이터를 가리키는 &str입니다.
    println!("Section 3 - &str: {}", string_slice);

    // String: 힙의 변경 가능한 String
    // JavaScript의 String은 불변이지만, Rust의 String은 변경 가능
    let mut heap_string = String::from("Hello, ");
    // String::from()은 문자열 리터럴(&str)을 받아 힙에 복사하여 String을 생성합니다.
    // JavaScript: let s = "Hello, "; // 불변
    // Rust: let mut s = String::from("Hello, "); // 변경 가능
    //
    // JavaScript에서 문자열 추가:
    //   s = s + "World!"; // 새 객체 생성 (메모리 할당 + 복사)
    //   s += "World!";    // 위와 동일
    // Rust에서 문자열 추가:
    //   s.push_str("World!"); // 기존 힙 메모리에 직접 추가 (복사 없음, 매우 빠름)
    //   s.push('!');          // 단일 문자 추가
    heap_string.push_str("World!"); // push_str로 문자열 추가 (JavaScript의 `+=`에 해당)
    // push_str()는 기존 String의 힙 메모리를 재할당하여 새 문자열을 추가합니다.
    // JavaScript의 +=는 새 객체를 생성하지만, Rust의 push_str()는 기존 메모리를 확장합니다.
    println!("   String: {}", heap_string);

    // JavaScript의 `+`로 문자열 연결은 비효율적 (새 객체 생성)
    // Rust의 `format!` 매크로는 JavaScript의 템플릿 리터럴과 유사
    let name = "Rust";
    // name은 &str 타입입니다 (타입 추론).
    let combined = format!("Hello, {}!", name); // JavaScript의 ``Hello, ${name}!``과 유사
    // format! 매크로는 새 String을 생성하여 반환합니다.
    // JavaScript: `Hello, ${name}!` → 템플릿 리터럴 (새 String 생성)
    // Rust: format!("Hello, {}!", name) → 새 String 생성
    // 둘 다 새 객체를 생성한다는 점은 동일하지만, Rust는 컴파일타임에 타입을 검증합니다.
    // format!에서 {}는 "플레이스홀더"이며, 여기에 name의 값이 들어갑니다.
    println!("   format!: {}", combined);

    // 문자열 연결 (JavaScript의 `+`에 해당)
    let mut string_concat = String::from("Hello");
    string_concat.push('!'); // OK! 문자 추가
    // push_str()는 문자열 전체를 추가하고, push()는 단일 문자를 추가합니다.
    // JavaScript: 'Hello'.concat('!') → 새 String
    // Rust: "Hello".push('!') → 기존 String 변경 (복사 없음)
    // push()는 char 타입을 받습니다 (1개의 유니코드 코드 포인트).
    // push_str()는 &str 타입을 받습니다 (문자열 전체).
    println!("   push!: {}", string_concat);
}

// ============================================================
// Section 4: 함수 (Functions)
// ============================================================
//
// JavaScript와 Rust의 함수는 많은 similarities와 differences가 있습니다.
//
// 비교표:
// ┌─────────────────────────┬──────────────────────────────────┐
// │ JavaScript              │ Rust                             │
// ├─────────────────────────┼──────────────────────────────────┤
// │ function add(a, b) {    │ fn add(a: i32, b: i32) -> i32 {  │
// │   return a + b;         │   a + b                          │
// │ }                       │ }                                │
// │                         │                                  │
// │ const double = (x) =>   │ let double = |x: i32| -> i32 {   │
// │   x * 2;                │   x * 2                          │
// │ }                       │ };                               │
// └─────────────────────────┴──────────────────────────────────┘
//
// 핵심 차이점:
// 1. 타입 명시: Rust는 모든 파라미터와 반환 타입을 명시해야 합니다.
//    JavaScript는 타입을 생략할 수 있지만, Rust는 컴파일타임 타입 검사를 위해 필요합니다.
// 2. 반환 값: Rust는 마지막 표현식이 암시적 반환입니다.
//    JavaScript는 명시적 `return`이 필요합니다.
// 3. 리턴 타입 생략: Rust에서는 `() -> ()`를 생략할 수 있습니다.
//    `fn foo() { println!("hi"); }` → 반환 타입이 자동으로 () (유니트 타입)
// 4. 함수 오버로딩: Rust에는 함수 오버로딩이 없습니다.
//    JavaScript는 파라미터 개수에 따라 다른 동작을 할 수 있지만, Rust는 불가능합니다.
//    대신 Option<T> 또는 trait로 처리합니다.
//
// 표현식 vs 문 (Expression vs Statement):
//   JavaScript: 모든 것이 표현식은 아님
//     if (x) { 1 } // 표현식이 아님, 문(statement)
//   Rust: if도 표현식
//     let y = if x { 1 } else { 2 }; // OK! if가 표현식
//
// 메모리 관점:
//   JavaScript 함수는 호출 시 매번 새 스코프 객체를 생성합니다.
//   Rust 함수는 스택 프레임만 할당하므로 매우 효율적입니다.
//   함수 호출 오버헤드: JavaScript > Rust (대부분의 경우)

fn section_4_functions() {
    // JavaScript의 `function add(a, b)`에 해당
    // JavaScript와 달리 파라미터와 반환 타입을 명시해야 합니다
    //
    // Rust 함수의 구조:
    //   fn 함수이름(파라미터: 타입, ...) -> 반환타입 {
    //       표현식 // 암시적 반환
    //   }
    //
    // JavaScript:
    //   function add(a, b) {
    //     return a + b;  // 명시적 return 필요
    //   }
    // Rust:
    fn add(a: i32, b: i32) -> i32 {
        a + b // ← 마지막 표현식이 암시적 반환 값 (JavaScript의 `return a + b`와 동일)
        // ;(세미콜론)이 없으므로 표현식으로 처리되어 반환됩니다.
        // ;를 추가하면 문(statement)이 되어 반환하지 않습니다.
        //   a + b;  // ERROR! ()를 반환해야 함
    }
    println!("   add(2, 3) = {}", add(2, 3));

    // JavaScript처럼 명시적 `return`도 사용 가능
    fn greet(name: &str) -> String {
        return format!("Hello, {}!", name); // 명시적 return
        // Rust에서도 명시적 return을 사용할 수 있습니다.
        // 주로 조기 반환(early return)에 사용합니다.
        //   if condition { return "early"; }
        //   "late" // 마지막 표현식 반환
    }
    println!("   greet: {}", greet("JavaScript developer"));

    // JavaScript의 화살표 함수 `const double = (x) => x * 2;`에 해당
    // Rust에서도 같은 형태로 함수 정의 가능 (반환 타입 생략 시 추론)
    //
    // 클로저의 구조:
    //   |파라미터: 타입| -> 반환타입 { 표현식 }
    //   |x: i32| -> i32 { x * 2 }
    //
    // JavaScript:
    //   const double = (x) => x * 2;
    // Rust:
    let double_fn = |x: i32| -> i32 { x * 2 }; // 이것은 클로저입니다 (별도 섹션에서 상세 설명)
    // 클로저는 변수에 할당될 수 있는 함수입니다.
    // JavaScript의 화살표 함수와 매우 유사하지만, Rust는 타입을 명시할 수 있습니다.
    // 타입을 생략하면 추론됩니다: let double_fn = |x| x * 2;
    println!("   Closure double(21) = {}", double_fn(21));

    // JavaScript의 `void` 함수에 해당 - 반환 타입은 `()` (유니트 타입)
    fn log_message(message: &str) {
        // -> () 생략 가능 (유니트 타입)
        // Rust의 유니트 타입 ()는 JavaScript의 undefined와 유사합니다.
        // 하지만 JavaScript의 undefined는 "값이 없음"을 의미하고,
        // Rust의 ()는 "값이 없음"이 아니라 "명확히 빈 값"을 의미합니다.
        println!("   [LOG] {}", message);
    }
    // 함수 호출: JavaScript: logMessage("hello"); / Rust: log_message("hello");
    log_message("Function section example");

    // JavaScript의 `default parameters`에 해당 - Rust는 기본값이 없지만
    // 오버로딩이 불가능하므로 다른 함수 이름이나 Option으로 처리
    //
    // JavaScript의 default parameter:
    //   function greet(name = "World") { ... }
    //   greet(); // "Hello, World!"
    //   greet("Alice"); // "Hello, Alice!"
    //
    // Rust의 Option<T> 패턴:
    //   fn greet(name: Option<&str>) { ... }
    //   greet(None); // "Hello, World!"
    //   greet(Some("Alice")); // "Hello, Alice!"
    //
    // Rust는 기본 parameter를 지원하지 않으므로, 이 패턴이 일반적입니다.
    fn optional_greet(name: Option<&str>) -> String {
        match name {
            Some(n) => format!("Hello, {}!", n),   // Some(값) → 값을 사용
            None => "Hello, World!".to_string(),  // None → 기본값 사용
        }
        // to_string()은 &str을 String으로 변환합니다.
        // JavaScript: "Hello, " + name → String
        // Rust: name.to_string() → String (힙 할당)
    }
    println!("   Optional: {}", optional_greet(Some("Alice")));
    println!("   Optional: {}", optional_greet(None)); // JavaScript의 undefined와 유사
    // None은 JavaScript의 null 또는 undefined에 해당합니다.
    // 하지만 Rust의 None은 컴파일타임에 타입이 결정되므로,
    // JavaScript의 null/undefined로 인한 런타임 에러가 발생하지 않습니다.
}

// ============================================================
// Section 5: Ownership (소유권)
// ============================================================
//
// **Rust에서 가장 중요한 개념** - JavaScript 개발자가 가장 어려워하는 부분
//
// JavaScript는 가비지 컬렉션(GC)으로 자동으로 메모리를 관리합니다.
// Rust는 GC가 없지만, 소유권 시스템을 통해 메모리 안전성을 보장합니다.
//
// 소유권 시스템의 세 가지 규칙:
// 1. Rust의 모든 값에는 하나의 **소유자(Owner)**가 있습니다
// 2. 소유자가 범위를 벗어나면 값은 **자동으로 버려집니다** (메모리 해제)
// 3. 한 번 이동된 값은 사용할 수 없습니다
//
// JavaScript vs Rust 메모리 관리 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (GC)          │ Rust (Ownership)                       │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ GC 스레드가 자동으로     │ 소유자가 범위 벗어나면 바로 해제        │
// │ 메모리를 정리합니다      │ (GC.pause_time = 0ms!)                 │
// │ 런타임에 동작합니다      │ 컴파일타임에 검증됩니다                 │
// │ 메모리 누수 가능성이 있음 │ 컴파일러가 메모리 누수를 방지합니다     │
// │ 데이터 경합 가능성 있음  │ 컴파일러가 데이터 경합을 방지합니다     │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 메모리 레이아웃 이해:
//   String (힙 데이터):
//     스택: [포인터 → 힙][길이][용량]   (24바이트)
//     힙:   [H][e][l][l][o][\0]         (실제 문자열 데이터)
//
//   i32 (스택 데이터):
//     스택: [42]                        (4바이트, 직접 저장)
//
// String이 Move되는 과정:
//   let s1 = String::from("hello");  // s1이 힙 데이터 소유
//   let s2 = s1;                     // s2가 힙 데이터 소유로 변경
//   // s1은 더 이상 유효하지 않음!    // JavaScript: GC가 알아서 정리
//   // Rust: s1이 가리키는 힙 데이터는 s2가 정리
//
// Copy 타입 vs Move 타입:
//   Copy 타입: i32, f64, bool, char 등 (스택에 직접 저장)
//     - 변수를 복사할 때 값이 그대로 복사됨
//     - 원본 변수도 여전히 유효함
//   Move 타입: String, Vec, Box 등 (힙 데이터를 포함)
//     - 변수를 복사할 때 포인터만 복사됨 (소유권 이동)
//     - 원본 변수는 더 이상 유효하지 않음

fn section_5_ownership() {
    // String은 힙에 데이터를 저장하는 타입
    // JavaScript: 모든 String은 힙에 저장
    //
    // String::from()은 &str을 받아 힙에 복사합니다.
    // JavaScript: let s1 = "hello"; // 문자열 리터럴
    // Rust: let s1 = String::from("hello"); // 힙에 복사
    let s1 = String::from("hello"); // s1이 "hello"의 소유자
    let s2 = s1; // s1의 값이 s2로 **이동**
                 // Move가 발생하면 s1은 더 이상 유효하지 않습니다.
                 // JavaScript: let s2 = s1; // 객체는 참조로 복사 (s1도 유효)
                 // Rust: let s2 = s1; // 소유권이 s2로 이동 (s1은 무효)
                 // println!("{}", s1);            // ERROR! s1은 더 이상 유효하지 않음 (이동됨)
    println!("Section 5 - Ownership: s2 = {}", s2); // OK! s2가 소유자
    // JavaScript 개발자를 위한 설명: JS의 GC가 해주는 일을 Rust에서는 직접 관리
    // JavaScript에서는 let s2 = s1; 후에도 s1이 유효하지만,
    // Rust에서는 s1의 소유권이 s2로 이동하므로 s1을 사용할 수 없습니다.

    // 명시적 복사를 위한 Clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // OK! 명시적 복사 - 힙 메모리도 함께 복사
    // clone()은 shallow copy가 아닌 deep copy를 수행합니다.
    // 힙에 있는 데이터 전체를 복사합니다.
    // JavaScript: let s4 = JSON.parse(JSON.stringify(s3));
    //   → JSON.stringify로 직렬화 → JSON.parse로 역직렬화
    // Rust: let s4 = s3.clone();
    //   → 힙 메모리를 직접 복사 (더 빠름)
    //
    // clone()의 성능 특징:
    //   O(n) 시간복잡도 (n = 문자열 길이)
    //   JavaScript: JSON.stringify()도 O(n)이지만, 직렬화/역직렬화 오버헤드 큼
    //   Rust: memcpy 기반의 직접 복사이므로 훨씬 빠름
    //   하지만 clone()은 명시적으로 호출해야 하므로,
    //   개발자가 "이 복사 비용을 감당할 수 있는지" 항상 고려하게 됩니다.
    //   이는 Rust의 "성능을 명시적으로 관리한다"는 철학을 반영합니다.
    //
    // clone() vs JavaScript 깊은 복사:
    //   JavaScript: 깊은 복사는 항상 명시적 (structuredClone, lodash.cloneDeep 등)
    //   Rust: clone()도 명시적 - 기본 동작이 move (소유권 이동)
    //   이 차이는 Rust가 "가벼운 동작을 기본"으로, "무거운 동작은 명시"하는 설계입니다.
    println!("   Clone: s3 = {}, s4 = {}", s3, s4); // 둘 다 사용 가능
    // JavaScript 개발자를 위한 설명: JavaScript의 `JSON.parse(JSON.stringify(obj))`와 유사하지만
    // Rust에서는 성능 고려로 명시적으로 clone()을 호출해야 합니다
    // clone()은 비용이 높은 연산 - O(n) 시간복잡도 (n = 문자열 길이)

    // i32와 같은 기본 타입은 스택에 저장되므로, Move 대신 Copy가 발생
    //
    // Copy trait를 구현한 타입은 자동으로 복사됩니다:
    //   - 모든 정수 타입: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    //   - 모든 부동 소수점 타입: f32, f64
    //   - bool
    //   - char
    //   - tuple (모든 요소가 Copy일 때): (i32, i32)
    let x = 42;
    let y = x; // OK! x가 복사됨 (Copy trait 구현 타입)
    // Copy 타입의 경우:
    //   - 스택에 직접 저장되므로 복사가 매우 빠름 (4바이트 복사)
    //   - 원본과 복사본이 모두 유효
    //   - 컴파일러가 자동으로 복사 코드를 생성
    println!("   Copy: x = {}, y = {}", x, y); // 둘 다 사용 가능
    // JavaScript 개발자를 위한 설명: JavaScript의 number는 모두 기본 타입이므로
    // 이것이 JavaScript의 동작과 가장 유사합니다
    // JavaScript: let y = x; → 값이 복사됨 (x도 유효)
    // Rust의 Copy 타입: let y = x; → 값이 복사됨 (x도 유효)
}

// ============================================================
// Section 6: References & Borrowing (참조 & 빌림)
// ============================================================
//
// 소유권을 이동하지 않고도 값을 사용할 수 있는 방법입니다.
// 함수에 값을 전달할 때 소유권을 넘기지 않고, 참조만 넘길 수 있습니다.
//
// 참조의 두 가지 형태:
//   &T     → 불변 참조 (Immutable Reference)
//            - 값을 읽을 수만 있고, 변경할 수 없음
//            - 여러 개 동시 허용 (무한히 많은 불변 참조 가능)
//   &mut T → 변경 가능 참조 (Mutable Reference)
//            - 값을 읽고 변경할 수 있음
//            - 하나만 동시 허용 (단일 변경자)
//
// JavaScript vs Rust 참조 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ 객체는 항상 참조로 전달  │ 값은 Move (소유권 이동)                 │
// │ const a = {x:1};         │ let s1 = String::from("hello");         │
// │ const b = a;             │ let s2 = s1;                            │
// │ b.x = 2;                 │ // s1은 더 이상 사용 불가!              │
// │ // a.x도 2로 변경됨      │ let len = &s2;                          │
// │ 런타임에 문제 발생       │ // &s2: 불변 참조 (소유권 이동 아님)    │
// │                          │ 컴파일타임에 데이터 경합 방지           │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 빌림(Borrowing) 규칙 (컴파일타임에 검증):
//   1. 참조는 항상 유효해야 합니다 ( dangling reference 불가)
//   2. 불변 참조는 여러 개 가능: &T, &T, &T ...
//   3. 변경 가능 참조는 하나만 가능: &mut T
//   4. 불변과 변경 가능 참조를 동시에 가질 수 없음
//      &T + &mut T → 컴파일 에러!
//
// 메모리 레이아웃:
//   &String (불변 참조):
//     스택: [포인터 → 힙] (8바이트, 64비트 시스템)
//     힙:   String 데이터 (변경 불가)
//   &mut String (변경 가능 참조):
//     스택: [포인터 → 힙] (8바이트, 64비트 시스템)
//     힙:   String 데이터 (변경 가능)
//
// JavaScript 개발자를 위한 핵심 비교:
//   JavaScript는 객체를 참조로 전달하므로:
//     function modify(obj) { obj.x = 99; }
//     const a = {x: 1}; modify(a); // a.x가 99로 변경됨!
//   Rust는 참조로 전달하므로:
//     fn modify(s: &mut String) { *s.push_str("x"); }
//     let mut a = String::from("1"); modify(&mut a); // 명시적으로 변경 가능
//   Rust의 &mut는 JavaScript의 참조 변경보다 더 명시적이고 안전합니다.

fn section_6_references_and_borrowing() {
    // 불변 참조 - 값 복사 없이 참조만 전달
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1: 불변 참조 (빌링)
    // &s1은 s1의 소유권을 넘기지 않고, s1의 메모리 주소를 전달합니다.
    // JavaScript: function length(s) { return s.length; }
    //             length(s1); // s1의 참조 전달 (s1은 여전히 유효)
    // Rust: calculate_length(&s1) // &s1은 s1의 주소만 전달 (s1은 여전히 유효)
    //
    // JavaScript vs Rust 참조 전달의 근본적 차이:
    //   JavaScript: 모든 객체는 "참조"로 전달 (레퍼런스 semantics)
    //     function modify(obj) { obj.x = 99; } // 객체 내용이 변경됨!
    //     const a = {x: 1}; modify(a); // a.x가 99로 변경됨 (의도치 않은 변경 가능!)
    //   Rust: 기본값은 "소유권 이동" (move semantics)
    //     fn take_ownership(s: String) { ... } // s의 소유권을 가져감
    //     let s1 = String::from("hello"); take_ownership(s1); // s1 사용 불가!
    //   Rust의 참조 전달은 "읽기 전용" (&T) 또는 "변경 가능"(&mut T)으로 명확히 구분
    //     fn read_only(s: &String) { ... } // s의 소유권을 넘기지 않음
    //     fn can_modify(s: &mut String) { ... } // s를 변경 가능 (mut 필요)
    println!(
        "Section 6 - Borrowing: Length is {}, s1 still valid: {}",
        len, s1
    );
    // s1이 여전히 유효합니다! 소유권을 넘기지 않았기 때문입니다.
    // JavaScript 개발자를 위한 설명: JavaScript의 객체 참조와 유사하지만
    // Rust는 컴파일타임에 "이 참조가 값을 변경하지 않을 것"을 보장
    // JavaScript에서는 함수가 객체를 변경할지 여부를 알 수 없지만,
    // Rust에서는 &T(불변 참조)를 받으면 함수가 값을 변경하지 않음을 보장합니다.

    // 변경 가능 참조 - 값을 변경할 수 있는 참조
    let mut s2 = String::from("hello");
    change_string(&mut s2); // &mut s2: 변경 가능 참조 (변경 가능 빌링)
    // &mut s2는 s2의 소유권을 넘기지 않고, s2의 변경 가능한 주소를 전달합니다.
    // JavaScript: s2 += ", world!"; // 직접 변경
    // Rust: change_string(&mut s2); // 변경 가능 참조로 전달
    println!("   Mutable borrow: {}", s2);
    // JavaScript 개발자를 위한 설명: JavaScript의 객체는 기본적으로 변경 가능합니다.
    // Rust에서는 변경하려면 명시적으로 `&mut`을 요청해야 합니다
    // JavaScript의 let obj = {}; obj.x = 1; (언제나 가능)
    // Rust의 let mut obj = MyStruct { x: 0 }; obj.x = 1; (mut 필요)

    // ✅ 동시에 여러 불변 참조 허용
    let _r1 = &s1;
    let _r2 = &s1;
    let _r3 = &s1; // 무한히 많은 불변 참조 허용
    // Rust는 여러 함수가 같은 데이터를 읽는 것을 안전하게 허용합니다.
    // JavaScript: 모든 함수가 같은 객체를 참조하므로 변경 가능성이 있습니다.
    // Rust: &T는 불변 참조이므로, 어떤 함수도 데이터를 변경할 수 없습니다.

    // ❌ 변경 가능 참조와 불변 참조를 동시에 가질 수 없음
    // let _r4 = &mut s2;  // ERROR! 불변 참조가 있을 때 변경 가능 참조를 가질 수 없음
    // 이 규칙은 데이터 경합(Data Race)을 방지합니다.
    // JavaScript: const a = {x: 1};
    //             setTimeout(() => { a.x = 2; }, 100);
    //             setInterval(() => { console.log(a.x); }, 50);
           //             // 런타임에 경합 발생 가능
    // Rust: 컴파일타임에 이런 경합을 방지합니다.
}

fn calculate_length(s: &String) -> usize {
    s.len() // JavaScript의 `string.length`에 해당
    // &String은 String의 참조입니다. JavaScript: s.length
    // 함수 내부에서 s를 변경할 수 없습니다 (불변 참조이므로).
}

fn change_string(s: &mut String) {
    s.push_str(", world!"); // JavaScript의 `string += ", world!"`에 해당
    // &mut String은 String의 변경 가능 참조입니다.
    // JavaScript: s += ", world!";
    // Rust: s.push_str(", world!"); // 기존 String에 직접 추가
}

// ============================================================
// Section 7: Structs (구조체)
// ============================================================
//
// JavaScript Object와 Rust Struct의 비교:
//
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript Object        │ Rust Struct                            │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ { name: "Alice", age: 30 } │ struct User { name: String, age: u32 } │
// │ 동적 - 필드 추가/삭제 가능 │ 정적 - 컴파일타임에 필드 결정          │
// │ 타입 없음 (dynamic)       │ 타입 있음 (static)                      │
// │ { ...obj, newField: x }   │ Struct { ..obj, field: x }             │
// │ obj.field = value (always) │ mut obj.field = value (mut 필요)       │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Java의 Class와 유사하지만 메서드가 없습니다 (데이터만 저장).
// JavaScript의 { name: "Alice", age: 30 }와 유사하지만,
// Rust는 모든 필드의 타입을 컴파일타임에 결정합니다.
//
// 메모리 레이아웃:
//   struct User {
//       username: String,    // 24바이트 (포인터+길이+용량)
//       email: String,       // 24바이트
//       sign_in_count: u64,  // 8바이트
//       active: bool,        // 1바이트 (패딩 포함 시 8바이트)
//   }
//   총: ~64바이트 (스택, 힙 데이터는 String이 관리)
//
// JavaScript vs Rust 구조체 비교:
//   JavaScript:
//     const user = { name: "Alice", age: 30 };
//     user.name = "Bob"; // 언제나 가능
//     user.email = "bob@test.com"; // 필드 추가 가능!
//   Rust:
//     let mut user = User { name: "Alice".into(), age: 30, email: "" };
//     user.name = "Bob".into(); // OK
//     user.email = "bob@test.com".into(); // email 필드가 없으면 ERROR!

fn section_7_structs() {
    // JavaScript의 `class User { constructor(name, age) { this.name = name; this.age = age; } }`
    // Rust의 struct는 Java의 class와 매우 유사
    //
    // JavaScript의 Object와 Rust의 Struct 비교:
    //   JavaScript: const user = { username: "alice", email: "a@b.com" };
    //   Rust:       let user = User { username: "alice".into(), email: "a@b.com".into() };
    //   둘 다 "이름-값" 쌍으로 데이터를 표현하지만, Rust는 타입이 정적입니다.
    //
    // JavaScript Object vs Rust Struct의 근본적 차이:
    //   JavaScript Object: 동적 (필드 추가/삭제/변경 모두 런타임에 가능)
    //     const user = { name: "Alice" };
    //     user.age = 30;          // 필드 추가 가능
    //     delete user.name;        // 필드 삭제 가능
    //   Rust Struct: 정적 (컴파일타임에 필드가 고정됨)
    //     let mut user = User { username: "alice", email: "" };
    //     user.age = 30;           // ERROR! age 필드가 없음
    //     user.username = "Bob";   // OK! (mut일 때만)
    //   JavaScript는 "무슨 필드가 있는지" 런타임에 결정되므로 타입 안전성이 낮지만,
    //   Rust는 모든 필드가 컴파일타임에 확정되므로 타입 안전성이 매우 높습니다.
    struct User {
        username: String,   // JavaScript: user.username (문자열)
        email: String,      // JavaScript: user.email (문자열)
        sign_in_count: u64, // JavaScript: user.signInCount (number, unsigned)
        active: bool,       // JavaScript: user.active (boolean)
        // struct의 필드는 기본적으로 public이 아닙니다 (Java의 private과 유사).
        // 외부 코드에서 직접 접근하려면 pub 키워드가 필요합니다.
    }

    // 인스턴스 생성 - JavaScript의 `new User("alice", ...)`에 해당
    //
    // JavaScript:
    //   class User {
    //     constructor(username, email) { this.username = username; this.email = email; }
    //   }
    //   const user1 = new User("alice_dev", "alice@example.com");
    //
    // Rust:
    let user1 = User {
        username: String::from("alice_dev"),  // JavaScript: "alice_dev"
        email: String::from("alice@example.com"), // JavaScript: "alice@example.com"
        sign_in_count: 1,                     // JavaScript: 1
        active: true,                         // JavaScript: true
    };
    println!("Section 7 - Struct: {} ({})", user1.username, user1.email);

    // 필드 접근 - JavaScript의 `user1.username`에 해당
    // JavaScript: console.log(user1.username); // "alice_dev"
    // Rust: println!("{}", user1.username); // "alice_dev"
    // 둘 다 dot notation으로 필드에 접근합니다.
    println!("   Active: {}", user1.active);

    // JavaScript의 객체는 변경 가능하지만, Rust struct 인스턴스는 기본적으로 불변
    // 변경하려면 `mut`을 추가해야 합니다
    //
    // JavaScript: const user2 = { username: "bob", email: "b@b.com" };
    //             user2.email = "new@b.com"; // 언제나 가능
    // Rust:
    let mut user2 = User {
        username: String::from("bob_dev"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("bob_new@example.com"); // OK! 변경 가능으로 변경
    // JavaScript: user2.email = "bob_new@example.com"; // 언제나 가능
    // Rust: mut이 없으면 컴파일 에러!
    println!("   After change: {}", user2.email);

    // struct 업데이트 문법 (spread operator와 유사)
    //
    // JavaScript: const user3 = { ...user2, email: "new@b.com" };
    //   → user2의 모든 필드를 복사하고 email만 변경
    // Rust:
    let user3 = User {
        ..user2 // user2의 나머지 필드 복사
    };
    // JavaScript의 `{...user2, email: "new"}`와 유사
    // Rust의 `..user2`는 user2의 모든 필드를 복사합니다.
    // JavaScript의 spread operator는 새 객체를 생성하지만,
    // Rust의 struct 업데이트는 새 인스턴스를 생성합니다.
  println!("   Spread: {}", user3.email);
}

// ============================================================
// Section 8: Enums & Match (열거형 & 패턴 매칭)
// ============================================================
//
// JavaScript에는 Enum이 없습니다 (Symbol 또는 문자열 상수를 사용).
// Rust의 Enum은 TypeScript의 Union Type과 유사하지만 훨씬 강력합니다.
// Java의 enum은 상수 집합에 불과하지만, Rust의 Enum은 각 variant에 데이터를 담을 수 있습니다.
//
// JavaScript vs Rust Enum 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ const STATUS = {          │ enum Status {                          │
// │   ACTIVE: 'active',       │   Active,                               │
// │   INACTIVE: 'inactive',   │   Inactive,                             │
// │ };                        │   Pending(u32),                         │
// │ 문자열로 비교             │   Error(String),                        │
// │ 런타임 에러 가능성         │ }                                      │
// │ `status === 'active'`     │                                          │
// │ 타입 안전성 없음           │ 컴파일타임 타입 안전성                  │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Rust Enum의 각 variant에 데이터를 담을 수 있습니다:
//   - Unit variant: Quit, Pending (데이터 없음)
//   - Tuple variant: ChangeColor(i32, i32, i32) (튜플 데이터)
//   - Struct variant: Move { x: i32, y: i32 } (명명된 필드)
//
// 메모리 레이아웃:
//   enum Message {
//       Quit,                          // 0바이트 (단일 variant)
//       Move { x: i32, y: i32 },      // 8바이트 (i32 + i32)
//       Write(String),                 // 24바이트 (String = 포인터+길이+용량)
//       ChangeColor(i32, i32, i32),   // 12바이트 (i32 * 3)
//   }
    //   실제 메모리: 가장 큰 variant 크기 + discriminant (1바이트)
    //   → 최대 25바이트 + 패딩
    //
    // 메모리 효율성:
    //   Rust Enum은 "tagged union"으로 구현되므로,
    //   한 번에 하나의 variant만 메모리에 저장됩니다.
    //   JavaScript: { type: 'move', x: 10, y: 20 } → 객체 전체가 힙에 저장됨
    //   Rust: Move { x: 10, y: 20 } variant만 저장, 다른 variant는 0바이트
    //   이 메모리 효율성은 JavaScript의 객체와는 다른 Rust만의 특징입니다.
    //
    // TypeScript Union Type과의 비교:
    //   TypeScript: type Status = 'active' | { x: number, y: number };
    //     → 런타임에 어떤 타입인지 확인 필요 (type guard)
    //   Rust: enum Status { Active, Move { x: i32, y: i32 } }
    //     → 컴파일타임에 모든 variant를 매칭해야 함 (exhaustive match)
    //   Rust의 match는 "모든 경우를 처리"해야 하므로,
    //   new variant를 추가할 때 컴파일 에러로 미처리 case를 알려줍니다.
//
// 패턴 매칭 (match)은 JavaScript의 switch보다 훨씬 강력합니다:
//   JavaScript: switch(msg.type) { case 'move': ... }
//   Rust: match msg { Message::Move { x, y } => ... }
//   Rust의 match는 exhaustive (모든 경우를 처리해야 함)

fn section_8_enums_and_match() {
    // JavaScript의 `const Direction = { UP: 'up', DOWN: 'down', LEFT: 'left', RIGHT: 'right' };`
    // 하지만 Rust Enum은 데이터를 담을 수 있습니다!
    //
    // JavaScript:
    //   const msg = { type: 'move', x: 10, y: 20 };
    //   switch (msg.type) {
    //     case 'move': console.log(msg.x, msg.y); break;
    //   }
    // Rust:
    enum Message {
        Quit,                          // 데이터 없는 variant (JavaScript: 'quit' 문자열)
        Move { x: i32, y: i32 },      // 객체 데이터 (JavaScript: { x: 1, y: 2 })
        Write(String),                 // 문자열 데이터 (JavaScript: 'hello')
        ChangeColor(i32, i32, i32),   // 튜플 데이터 (JavaScript: [255, 128, 0])
        // 각 variant에 서로 다른 타입과 크기의 데이터를 담을 수 있습니다.
        // JavaScript에서는 Object로 모든 것을 처리하지만,
        // Rust에서는 Enum으로 타입을 명시적으로 구분합니다.
    }

    // JavaScript는 처리에 `switch (msg)`를 사용
    // Rust는 패턴 매칭에 `match`를 사용 - 모든 경우를 처리해야 함 (exhaustive)
    //
    // JavaScript:
    //   switch (msg.type) {
    //     case 'quit': ...; break;
    //     case 'move': ...; break;
    //     // break를 잊으면 fall-through 발생!
    //   }
    // Rust:
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("   Quit message"),
        // Message::Quit는 데이터가 없는 variant이므로 패턴이 단순합니다.
        Message::Move { x, y } => println!("   Move: ({}, {})", x, y),
        // { x, y }는 struct variant의 필드를 해체합니다.
        // JavaScript: const { x, y } = msg; console.log(x, y);
        Message::Write(text) => println!("   Write: {}", text),
        // (text)는 tuple variant의 첫 번째 요소를 해체합니다.
        Message::ChangeColor(r, g, b) => println!("   ChangeColor: RGB({}, {}, {})", r, g, b),
        // (r, g, b)는 tuple variant의 세 요소를 해체합니다.
    }
    // JavaScript의 switch와 유사하지만, Rust는 모든 경우를 처리해야 함 (exhaustive match)
    // JavaScript: switch에서 break를 잊으면 fall-through 발생 가능
    // Rust: match는 자동으로 fall-through가 없고, 모든 경우를 처리해야 함

    // Option - JavaScript의 `null`/`undefined`를 처리하기 위한 Enum
    // Java의 Optional과 매우 유사
    //
    // JavaScript:
    //   const result = findUser(42); // null 또는 { name: "Alice" }
    //   if (result !== null) { ... }
    // Rust:
    let some_number: Option<i32> = Some(5);  // 값이 있음
    let no_number: Option<i32> = None;       // 값이 없음
    // Some(T) → T값이 있음, None → 값이 없음
    // JavaScript: null/undefined를 혼용하지만,
    // Rust: Option<T>로 "값이 없을 수 있음"을 타입으로 표현
    println!("Section 8 - Option: {:?}, {:?}", some_number, no_number);

    // JavaScript의 null 처리: `x !== null ? x : x.toString()`
    // Rust: match로 안전한 처리
    //
    // JavaScript:
    //   const result = someNumber !== null ? someNumber : 0;
    //   // null 체크를 잊으면 런타임 에러!
    // Rust:
    match some_number {
        Some(n) => println!("   Some: {}", n),
        // Some(n)은 n이 i32 값입니다. 패턴 매칭으로 값을 추출합니다.
        None => println!("   None (null/undefined에 해당)"),
        // None은 JavaScript의 null 또는 undefined에 해당합니다.
    }
    // Rust의 match는 항상 Some 또는 None 둘 중 하나를 처리합니다.
    // JavaScript: if (x === null)는 잊을 수 있지만,
    // Rust: match는 컴파일타임에 모든 경우를 처리했는지 확인합니다.
}

// ============================================================
// Section 9: Pattern Matching (패턴 매칭)
// ============================================================
//
// JavaScript의 `switch`보다 Rust의 `match`가 훨씬 강력합니다.
//
// JavaScript vs Rust 패턴 매칭 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript switch        │ Rust match                             │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ switch(value) {           │ match value {                           │
// │   case 1: ...; break;     │   1 => ...,                             │
// │   case 2: ...; break;     │   2 => ...,                             │
// │   default: ...            │   _ => ...,                              │
// │ }                         │ }                                      │
// │ break 잊으면 fall-through │ fall-through 불가                      │
// │ 모든 case 처리 안해도 OK  │ 모든 경우 처리 필수 (exhaustive)       │
// │ 단순 값만 매칭            │ 값, 범위, 구조체 해체, 조건부 매칭     │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 패턴 매칭의 주요 기능:
//   1. 리터럴 매칭: 0, 1, 2, ... (단순 값 비교)
//   2. 여러 값 매칭: 1 | 2 | 3 (OR 패턴)
//   3. 범위 매칭: 10..=20, 0..100 (JavaScript에서는 불가능)
//   4. 변수 매칭: x (모든 값과 매칭)
//   5. 와일드카드: _ (모든 값과 매칭하지만 값 사용 안 함)
//   6. 구조체 해체: Point { x, y }
//   7. 튜플 해체: (x, y)
//   8. Option/Result 매칭: Some(x), Ok(x), Err(e)
//   9. if let: 단일 패턴만 처리 (JavaScript의 if와 유사)
//
// 범위 매칭 (Range Matching):
//   10..=20 → 10 이상 20 이하 (JavaScript: n >= 10 && n <= 20)
//   10..20  → 10 이상 20 미만 (JavaScript: n >= 10 && n < 20)
//   JavaScript에는 이런 범위 매칭이 없습니다.

fn section_9_pattern_matching() {
    // JavaScript의 switch에 해당
    let number = 42;
    match number {
        0 => println!("   0"),
        // 0과 정확히 매칭
        1 | 2 | 3 => println!("   1, 2, or 3"), // |로 여러 값 매칭
        // 1, 2, 3 중 하나와 매칭
        // JavaScript: case 1: case 2: case 3: (fall-through)
        // Rust: 1 | 2 | 3 (OR 패턴, fall-through 없음)
        10..=20 => println!("   10~20 range"),  // 범위 매칭 (JavaScript에서는 불가능)
        // 10 <= number <= 20 (포함 범위, ..=)
        // JavaScript: if (n >= 10 && n <= 20)
        21..=100 => println!("   21~100 range"),
        // 21 <= number <= 100 (포함 범위)
        // JavaScript의 switch에는 범위 매칭이 없습니다.
        // Rust의 범위 매칭은 컴파일타임에 중복을 검증합니다.
        _ => println!("   Other number"), // _ = 기본 케이스 (JavaScript의 switch default)
        // _는 "모든 나머지 값"과 매칭합니다.
        // JavaScript: default
        // _: 매칭은 하지만 값을 사용하지 않음
        // n: 매칭하고 n에 값을 바인딩
    }

    // 해체 (Destructuring) - JavaScript의 구조 분해 할당과 유사
    //
    // JavaScript:
    //   const point = [3, 5];
    //   const [x, y] = point;
    //   console.log(x, y); // 3, 5
    // Rust:
    let point = (3, 5); // JavaScript: const point = [3, 5];
    // Rust의 튜플 (3, 5)은 JavaScript의 배열 [3, 5]에 해당합니다.
    // 하지만 튜플은 고정 크기와 고정 타입을 가집니다.
    let (x, y) = point; // JavaScript: const [x, y] = point;
    // 튜플 해체는 JavaScript의 배열 구조 분해 할당과 정확히 동일합니다.
    // JavaScript: const [a, b, c] = [1, 2, 3];
    // Rust: let (a, b, c) = (1, 2, 3);
    println!("   Destructuring: ({}, {})", x, y);

    // 객체 해체 (Object Destructuring) - JavaScript의 `const { name, age } = user;`와 유사
    //
    // JavaScript:
    //   const user = { name: "Alice", age: 30 };
    //   const { name, age } = user;
    // Rust:
    struct Point2D {
        x: i32,
        y: i32,
    }
    let p = Point2D { x: 42, y: 99 };
    let Point2D { x: px, y: py } = p; // JavaScript: const { x: px, y: py } = p;
    // { x: px, y: py }는 "p.x를 px에, p.y를 py에 바인딩"한다는 의미입니다.
    // JavaScript: const { x: px, y: py } = p; // 동일 개념!
    // x: px → p의 x 필드를 px라는 변수에 바인딩
    // x만 쓴다면 { x, y }로 간단히 할 수 있습니다.
    println!("   Object destructuring: ({}, {})", px, py);

    // if let - JavaScript의 `if (condition)`에 해당
    //
    // JavaScript:
    //   const favoriteColor = null;
    //   if (favoriteColor) {
    //     console.log(favoriteColor);
    //   }
    // Rust:
    let favorite_color: Option<&str> = Some("blue");
    // Option<&str>는 "문자열이 있을 수도 있고 없을 수도 있는" 타입입니다.
    if let Some(color) = favorite_color {
        // Some(color)는 favorite_color가 Some(값)일 때만 실행됩니다.
        // color에 Some 안의 값이 바인딩됩니다.
        println!("   Favorite color: {}", color);
    } else {
        // None일 때 실행됩니다.
        println!("   No favorite color");
    }
    // JavaScript: `if (favoriteColor) { console.log(favoriteColor); }`와 유사
    // 하지만 Rust의 if let은 None일 때의 처리도 명시적으로 작성합니다.
    // JavaScript의 truthy/falsy에 의존하지 않으므로 더 안전합니다.
}

// ============================================================
// Section 10: Collections (컬렉션)
// ============================================================
//
// Rust 컬렉션은 JavaScript의 Array, Object(=Map)에 해당합니다.
//
// JavaScript vs Rust 컬렉션 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ let arr = [1, 2, 3];     │ let arr = vec![1, 2, 3];               │
// │ 배열의 타입이 다 섞일 수 │ Vec<T>는 모든 요소가 같은 타입          │
// │ arr = [1, "a", true];    │ arr = vec![1, 2, 3]; // OK             │
// │                          │ arr = vec![1, "a"]; // ERROR!           │
// │ const map = new Map();    │ use std::collections::HashMap;         │
// │ map.set("key", 1);        │ map.insert("key", 1);                  │
// │ map.get("key")             │ map.get("key")                          │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 주요 컬렉션 타입:
//   Vec<T>      → JavaScript Array (가변 길이 배열)
//   HashMap<K,V> → JavaScript Map (키-값 저장소)
//   String      → JavaScript String (가변 문자열)
//   &str        → JavaScript String (불변 문자열 참조)
//
// Vec 메모리 레이아웃:
//   스택: [포인터 → 힙][길이][용량] (24바이트)
//   힙:   [elem1][elem2][elem3][...][unused capacity...]
//   Vec은 힙에 연속적으로 데이터를 저장하므로, 인덱스 접근이 O(1)입니다.
//
// HashMap 메모리 레이아웃:
//   스택: [포인터 → 힙][길이] (16바이트)
//   힙:   [버킷 배열] → [키, 값] 쌍들이 해시값으로 배치
//   HashMap은 해시 테이블을 사용하므로, 평균 O(1) 탐색 시간입니다.

fn section_10_collections() {
    // Vec<Vector>는 JavaScript의 Array에 해당
    //
    // JavaScript: let numbers = [1, 2, 3, 4, 5];
    // Rust:
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // JavaScript: let numbers = [1, 2, 3, 4, 5];
    // vec![] 매크로로 Vec을 쉽게 생성합니다.
    // JavaScript의 []과 정확히 동일하지만, Rust는 타입을 명시합니다.
    // JavaScript: Array.isArray(numbers) → true
    // Rust: numbers.is_empty() → false
    println!("Section 10 - Vec: {:?}", numbers);
    // {:?}는 디버그 출력입니다. JavaScript: console.log(numbers)
    // JavaScript: [1, 2, 3, 4, 5]
    // Rust: [1, 2, 3, 4, 5]

    // JavaScript의 array.push에 해당
    //
    // JavaScript: numbers.push(6);
    // Rust: numbers.push(6);
    // 둘 다 배열 끝에 요소를 추가합니다.
    // JavaScript: arr.push(6); arr.length → 6
    // Rust: numbers.push(6); numbers.len() → 6
    numbers.push(6);
    println!("   After push: {:?}", numbers);

    // JavaScript의 array.forEach에 해당
    //
    // JavaScript: numbers.forEach(num => console.log(num));
    // Rust:
    for num in &numbers {
        // &numbers는 numbers의 참조입니다 (소유권 이동 없음).
        // JavaScript: for (const num of numbers) { console.log(num); }
        // Rust: for num in &numbers { println!("{}", num); }
        // 둘 다 배열의 각 요소를 순회합니다.
        // JavaScript: for...of는 복사본을 제공
        // Rust: &numbers는 참조를 제공 (복사 없음, 효율적)
        println!("   num: {}", num);
    }

    // JavaScript의 array.map에 해당
    //
    // JavaScript: const doubled = numbers.map(x => x * 2);
    // Rust:
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    // numbers.iter() → Iterator를 생성 (각 요소의 참조 &i32)
    // .map(|x| x * 2) → 각 요소를 2배로 변환
    // .collect() → 결과를 Vec<i32>로 모음
    // JavaScript의 map은 새 배열을 생성하지만,
    // Rust의 map도 Lazy Iterator이므로 collect()에서じめて 실제 생성됨.
    println!("   map(x*2): {:?}", doubled);

    // JavaScript의 array.filter에 해당
    //
    // JavaScript: const evens = numbers.filter(x => x % 2 === 0);
    // Rust:
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    // numbers.iter() → Iterator<&i32> 생성
    // .filter(|x| **x % 2 == 0) → 짝수만 남김
    //   **x → &i32에서 i32로 dereference (두 번 필요한 이유는 iterator가 &i32를 반환)
    // .copied() → &i32를 i32로 복사 (Copy trait 구현이므로 빠름)
    // .collect() → Vec<i32>로 모음
    // JavaScript의 filter는 새 배열을 생성하지만,
    // Rust의 filter도 Lazy Iterator이므로 collect()에서じめて 실제 생성됨.
    println!("   filter(even): {:?}", evens);

    // HashMap은 JavaScript의 Map에 해당
    //
    // JavaScript:
    //   const scores = new Map();
    //   scores.set("Rust", 95);
    //   scores.get("Rust"); // 95
    // Rust:
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    // JavaScript: const scores = new Map();
    // Rust: HashMap::new() → 빈 HashMap 생성
    // HashMap<String, i32> → 키는 String, 값은 i32
    // JavaScript의 Map은 키에 어떤 타입도 사용 가능하지만,
    // Rust의 HashMap은 키 타입이 Hash + Eq trait을 구현해야 함.
    scores.insert("Rust".to_string(), 95); // JavaScript: scores.set("Rust", 95);
    // insert()는 키-값 쌍을 추가합니다.
    // JavaScript: map.set(key, value)
    // Rust: map.insert(key, value)
    scores.insert("JavaScript".to_string(), 85);
    scores.insert("Python".to_string(), 90);

    // JavaScript: scores.get("Rust")
    //
    // JavaScript: const score = scores.get("Rust"); // 95 또는 undefined
    // Rust:
    if let Some(score) = scores.get("Rust") {
        // scores.get()은 Option<&i32>를 반환합니다.
        // JavaScript의 Map.get()은 값 또는 undefined를 반환하지만,
        // Rust의 HashMap.get()은 Option<&T>를 반환합니다.
        // JavaScript: if (score !== undefined) { console.log(score); }
        // Rust: if let Some(score) = scores.get("Rust") { ... }
        // 둘 다 "값이 있는지" 확인하지만, Rust는 타입 안전합니다.
        println!("   Rust score: {}", score);
    }

    // JavaScript의 Object.keys와 유사
    //
    // JavaScript: for (const [key, value] of scores.entries()) { ... }
    // Rust:
    for (language, score) in &scores {
        // &scores는 HashMap의 참조입니다.
        // iter()는 (K, V)의 참조 튜플을 반환합니다.
        // JavaScript: for (const [lang, sc] of scores.entries()) { console.log(lang, sc); }
        // Rust: for (language, score) in &scores { ... }
        // 둘 다 키-값 쌍을 순회합니다.
        println!("   {} : {}", language, score);
   }
}

// ============================================================
// Section 11: Closures (클로저)
// ============================================================
//
// JavaScript의 화살표 함수 (`=>`)와 Rust의 클로저 (`|...|`)
// - JavaScript: `(x) => x * 2`
// - Rust: `|x| x * 2`
// - 둘 다 다른 함수의 인수로 전달할 수 있는 "1급 함수(first-class function)"입니다
//
// JavaScript vs Rust 클로저 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ const add = (a, b) =>    │ let add = |a: i32, b: i32| -> i32 {    │
// │   a + b;                  │   a + b                                 │
// │ };                        │ };                                      │
// │                          │                                         │
// │ arr.map(x => x * 2)      │ arr.iter().map(|x| x * 2).collect()    │
// │ arr.filter(x => x > 0)   │ arr.iter().filter(|x| *x > 0).collect()│
// │ arr.reduce((a, x) =>     │ arr.iter().fold(0, |a, x| a + x)       │
// │   a + x, 0)              │                                         │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 클로저의 변수 캡처 방식:
//   &T (불변 참조로 캡처) - clone()이 필요 없을 때
//   &mut T (변경 가능 참조로 캡처) - 값을 수정할 때
//   T (소유권으로 캡처) - move 클로저, 한 번만 호출할 때
//
// JavaScript의 화살표 함수는 항상 외부 변수를 참조로 캡처하지만,
// Rust의 클로저는 사용 방식에 따라 캡처 방식이 동적으로 결정됩니다.

fn section_11_closures() {
    // JavaScript의 `const double = (x) => x * 2;`에 해당
    //
    // JavaScript:
    //   const double = (x) => x * 2;
    //   double(21); // 42
    // Rust:
    let double_fn = |x| x * 2; // 파라미터 타입과 반환 타입은 추론
    // |x| → 파라미터 x
    // x * 2 → 표현식 (반환값)
    // ;(세미콜론)이 없으므로 반환값이 있습니다.
    // JavaScript의 화살표 함수와 거의 동일하지만,
    // Rust는 타입을 명시할 수도 있습니다: |x: i32| -> i32 { x * 2 }
    println!("Section 11 - Closure: double(21) = {}", double_fn(21));

    // 타입을 명시적으로 지정
    //
    // fn(i32, i32) -> i32는 "i32 두 개를 받아 i32를 반환하는 함수 포인터 타입"입니다.
    // JavaScript: let add: (a: number, b: number) => number = (a, b) => a + b; (TypeScript)
    // Rust: let add: fn(i32, i32) -> i32 = |a, b| a + b;
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    // fn(...) -> ...은 함수 포인터 타입입니다.
    // JavaScript의 (a, b) => a + b에 대응하지만,
    // Rust의 fn은 클로저가 아닌 일반 함수 포인터입니다.
    // 클로저 캡처가 없으면 fn 포인터로 변환됩니다.
    println!("   add(3, 4) = {}", add(3, 4));

    // JavaScript의 `arr.map(x => x * 2)`에 해당
    //
    // JavaScript:
    //   const numbers = [1, 2, 3, 4, 5];
    //   const squared = numbers.map(x => x * 2); // [1, 4, 9, 16, 25]
    // Rust:
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    // numbers.iter() → Iterator<&i32> 생성
    // .map(|x| x * x) → 각 요소를 제곱
    // .collect() → Vec<i32>로 결과 모음
    // JavaScript의 map은 새 배열을 즉시 생성하지만,
    // Rust의 map은 Lazy Iterator이므로 collect()에서じめて 실제 생성됨.
    println!("   map(x*x): {:?}", squared);

    // JavaScript의 `arr.filter(x => x > 2)`에 해당
    //
    // JavaScript:
    //   const numbers = [1, 2, 3, 4, 5];
    //   const filtered = numbers.filter(x => x > 2); // [3, 4, 5]
    // Rust:
    let greater_than_two: Vec<i32> = numbers.iter().filter(|x| **x > 2).copied().collect();
    // .filter(|x| **x > 2) → x가 &i32이므로 **x로 dereference
    //   JavaScript: filter(x => x > 2) → x는 number
    //   Rust: filter(|x| **x > 2) → x는 &i32, **x는 i32
    // .copied() → &i32를 i32로 복사 (Copy trait이므로 빠름)
    println!("   filter(> 2): {:?}", greater_than_two);

    // JavaScript의 `arr.reduce((acc, x) => acc + x, 0)`에 해당
    //
    // JavaScript:
    //   const numbers = [1, 2, 3, 4, 5];
    //   const sum = numbers.reduce((acc, x) => acc + x, 0); // 15
    // Rust:
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    // fold(init, closure) → reduce에 해당
    //   init: 초기값 (0)
    //   closure: |acc, x| → acc는 누적값, x는 현재 요소
    // JavaScript의 reduce는 배열 요소를 직접 사용하지만,
    // Rust의 fold는 Iterator의 요소를 사용합니다 (참조일 수 있음).
    println!("   fold(sum): {}", sum);

    // 클로저는 주변 스코프의 변수를 캡처할 수 있습니다
    // JavaScript의 중첩 함수가 외부 변수에 접근하는 것과 유사
    //
    // JavaScript:
    //   const multiplier = 10;
    //   const multiplyByTen = (x) => x * multiplier;
    //   multiplyByTen(5); // 50
    // Rust:
    let multiplier = 10;
    let multiply_by_ten = |x| x * multiplier; // multiplier를 캡처
    // multiplier를 캡처할 때 Rust는 자동으로 캡처 방식을 결정합니다:
    //   - multiplier가 Copy 타입이면: 값으로 복사 (&Copy)
    //   - multiplier가 Copy가 아니면: 참조로 캡처 (&mut 또는 &)
    // JavaScript의 중첩 함수가 항상 참조로 캡처하는 것과 달리,
    // Rust는 타입에 따라 최적의 방식을 선택합니다.
    //
    // 클로저 캡처 모드 3가지 (JavaScript에는 없는 개념):
    //   1. &T (불변 참조로 캡처) - 값을 읽기만 할 때
    //      let s = String::from("hello");
    //      let f = || println!("{}", s); // s를 복사하지 않고 참조만 사용
    //   2. &mut T (변경 가능 참조로 캡처) - 값을 수정할 때
    //      let mut count = 0;
    //      let mut f = || { count += 1; }; // count를 변경 가능
    //   3. T (소유권으로 캡처) - move 키워드 또는 Copy olmayan 타입
    //      let s = String::from("hello");
    //      let f = move || println!("{}", s); // s의 소유권을 클로저가 가져감
    //      // move 후 s를 사용할 수 없음!
    //   JavaScript의 화살표 함수는 항상 참조로 캡처하지만,
    //   Rust는 move로 소유권 이동을 명시적으로 제어할 수 있습니다.
    println!("   captured: multiply_by_ten(5) = {}", multiply_by_ten(5));
}

// ============================================================
// Section 12: Traits (트레이트)
// ============================================================
//
// JavaScript의 덕 타이핑("오리가처럼 울리면 그것은 오리다")과 유사하지만 타입 안전합니다.
// Java의 Interface와 매우 유사합니다.
// - Java: `interface Printable { void print(); }`
// - Rust: `trait Printable { fn print(&self); }`
// JavaScript에는 Interface가 없지만, TypeScript에는 있습니다.
// Rust의 Trait은 TypeScript Interface와 가장 유사합니다.
//
// JavaScript vs Rust Trait 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (덕 타이핑)   │ Rust (Trait)                           │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ 객체가 메서드가 있으면    │ trait를 implement해야 함               │
// │ 작동하는 것처럼 보인다   │ 컴파일타임에 검증                      │
// │ 런타임에 에러 가능       │ 컴파일타임에 에러 발견                  │
// │ const duck = { quack(){} }│ struct Duck;                            │
// │                          │ impl Quackable for Duck { fn quack(){} }│
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Trait의 주요 용도:
//   1. 공통 행동 정의: 여러 타입이 같은 메서드를 가지도록 강제
//   2. 제네릭 제한: 특정 trait을 구현한 타입만 허용
//   3. polymorphism: 서로 다른 타입을 통일된 인터페이스로 처리
//
// 메모리 레이아웃:
//   trait Drawable { fn draw(&self) -> String; }
//   struct Circle { radius: f64 }
//   struct Rectangle { width: f64, height: f64 }
//
//   Circle 인스턴스: [f64 radius] (8바이트)
//   Rectangle 인스턴스: [f64 width, f64 height] (16바이트)
//   Box<dyn Drawable>: [포인터 → vtable] (16바이트)
//     vtable: [draw 메서드 포인터]

fn section_12_traits() {
    // Java의 `interface Drawable { void draw(); }`에 해당
    //
    // Java:
    //   interface Drawable {
    //     String draw();
    //   }
    // Rust:
    trait Drawable {
        // trait은 메서드 시그니처만 정의합니다 (구현은 각 struct가 함).
        // &self는 "이 객체의 참조"를 의미합니다.
        // JavaScript: function draw() { return '...'; }
        // Rust: fn draw(&self) -> String
        fn draw(&self) -> String; // JavaScript: `draw() { return '...'; }`
        // &self는 JavaScript의 this에 해당합니다.
        // JavaScript: this.radius / Rust: self.radius
    }

    // struct에 Trait 구현 (JavaScript의 class에 해당)
    //
    // JavaScript:
    //   class Circle {
    //     constructor(radius) { this.radius = radius; }
    //     draw() { return `Circle: radius = ${this.radius}`; }
    //   }
    // Rust:
    #[derive(Debug)]
    // #[derive(Debug)]는 Debug trait을 자동으로 구현해줍니다.
    // JavaScript: console.log(circle) → Object 출력
    // Rust: println!("{:?}", circle) → Debug 형식으로 출력
    // Debug trait은 {:?} 포맷터로 사용할 수 있는 trait입니다.
    struct Circle {
        radius: f64,   // JavaScript: this.radius
    }
    struct Rectangle {
        width: f64,    // JavaScript: this.width
        height: f64,   // JavaScript: this.height
    }

    impl Drawable for Circle {
        // Circle이 Drawable trait을 구현합니다.
        // JavaScript: Circle.prototype.draw = function() { ... }
        // Rust: impl Drawable for Circle { ... }
        fn draw(&self) -> String {
            format!("Circle: radius = {}", self.radius)
            // JavaScript: return `Circle: radius = ${this.radius}`;
            // format! 매크로는 JavaScript의 템플릿 리터럴과 유사합니다.
        }
    }

    impl Drawable for Rectangle {
        // Rectangle도 Drawable trait을 구현합니다.
        // JavaScript:
        //   class Rectangle {
        //     constructor(w, h) { this.width = w; this.height = h; }
        //     draw() { return `Rectangle: ${this.width} x ${this.height}`; }
        //   }
        fn draw(&self) -> String {
            format!("Rectangle: {} x {}", self.width, self.height)
            // JavaScript: return `Rectangle: ${this.width} x ${this.height}`;
        }
    }

    // JavaScript의 `function drawAll(shape)`에 해당
    // 하지만 Rust는 타입 안전합니다 (구현체는 모두 Drawable이 될 수 있음)
    //
    // JavaScript:
    //   function drawAll(shapes) {
    //     shapes.forEach(s => console.log(s.draw()));
    //   }
    //   drawAll([new Circle(5), new Rectangle(10, 20)]);
    // Rust:
    let shapes: Vec<Box<dyn Drawable>> = vec![
        // Box<dyn Drawable>는 "trait object"입니다.
        // 서로 다른 타입(Circle, Rectangle)을 통일된 타입으로 저장합니다.
        // JavaScript: [new Circle(5), new Rectangle(10, 20)]
        // Rust: Vec<Box<dyn Drawable>>
        Box::new(Circle { radius: 5.0 }),   // JavaScript: new Circle(5)
        Box::new(Rectangle {
            width: 10.0,                    // JavaScript: new Rectangle(10, 20)
            height: 20.0,
        }),
    ];

    for shape in &shapes {
        // &shapes는 shapes의 참조입니다 (소유권 이동 없음).
        // shape는 &Box<dyn Drawable>입니다.
        println!("   {}", shape.draw());
        // JavaScript: shapes.forEach(s => console.log(s.draw()));
    }

    // JavaScript에서는 `obj.toString()`을 호출하려면 `obj`가 toString 메서드를 가져야 합니다
    // Rust에서는 Trait 구현이 자동으로 `to_string()`을 추가합니다!
    //
    // JavaScript:
    //   const circle = { radius: 5, toString() { return `Circle: ${this.radius}`; } };
    //   console.log(circle.toString());
    // Rust:
    let circle = Circle { radius: 5.0 };
    println!("   Circle: {:?}", circle); // Debug trait로 출력
    // {:?}는 Debug trait을 사용하여 출력합니다.
    // JavaScript: console.log(circle) → Object의 모든 필드 출력
    // Rust: println!("{:?}", circle) → Debug trait 구현 시 필드 출력
}

// ============================================================
// Section 13: Error Handling (에러 처리)
// ============================================================
//
// Rust의 Result<T, E>는 JavaScript의 try/catch에 해당합니다.
//
// JavaScript vs Rust 에러 처리 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ try {                     │ let result: Result<T, E> = some_fn();   │
// │   const x = parseInt(s); │ match result {                         │
// │ } catch(e) {              │   Ok(x) => ...,                         │
// │   console.log(e);         │   Err(e) => ...,                        │
// │ }                         │ }                                      │
// │ 에러를 잊으면 런타임 에러 │ Result를 처리하지 않으면 컴파일 경고     │
// │ null/undefined 가능       │ Option<T>로 명확히 표현                │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Result<T, E> 타입:
//   Ok(T) → 성공, T값 포함
//   Err(E) → 실패, E에러 정보 포함
//   JavaScript: try/catch는 런타임에 동작하지만,
//   Rust: Result는 컴파일타임에 타입이 결정됩니다.
//
// ? 연산자 (Try Operator):
//   JavaScript: try { return fn(); } catch(e) { return; }
//   Rust: let x = fn()?; // Err이면 즉시 반환
//   ?는 에러 전파(error propagation)를 간결하게 처리합니다.
//
// Option<T> 타입:
//   Some(T) → 값이 있음
//   None → 값이 없음
//   JavaScript: null/undefined 대신 타입 안전

fn section_13_error_handling() {
    // JavaScript: `try { parseInt("42"); } catch(e) { console.log(e); }`
    //
    // JavaScript:
    //   try {
    //     const number = parseInt("42");
    //     console.log(number);
    //   } catch(e) {
    //     console.log(e);
    //   }
    // Rust:
    let parsed: Result<i32, std::num::ParseIntError> = "42".parse();
    // "42".parse() → Result<i32, ParseIntError>
    //   Ok(42) → 파싱 성공
    //   Err(e) → 파싱 실패 (예: "abc".parse())
    // JavaScript: parseInt("42") → 42, parseInt("abc") → NaN
    // Rust: "42".parse::<i32>() → Ok(42), "abc".parse::<i32>() → Err(e)
    // JavaScript의 NaN은 "잘못된 숫자"를 의미하지만,
    // Rust의 Err(e)는 명확히 에러 타입을 포함합니다.
    //
    // Result<T, E> vs JavaScript try/catch의 근본적 차이:
    //   JavaScript try/catch: 런타임에 에러를 잡음 (에러를 놓칠 수 있음)
    //     try { const x = JSON.parse("invalid"); } catch(e) { ... }
    //     JSON.parse("invalid")를 try를 잊으면 런타임 에러 발생
    //   Rust Result: 컴파일타임에 에러 처리를 강제
    //     let x: Result<i32, _> = "invalid".parse();
    //     match x { Ok(v) => ..., Err(e) => ... }
    //     Result를 match하지 않으면 컴파일 경고 발생!
    //     ? 연산자로 에러 전파: "abc".parse::<i32>()? → Err이면 즉시 반환
    //   이 차이는 Rust가 "에러를 항상 처리하도록 강제"하는 설계 철학입니다.
    match parsed {
        Ok(number) => println!("OK - Result: {}", number),
        // Ok(number) → number는 i32 값입니다.
        Err(e) => println!("Error: {:?}", e),
        // Err(e) → e는 ParseIntError입니다.
        // JavaScript: catch(e) { console.log(e.message); }
    }

    // Rust의 ? 연산자는 JavaScript의 `try { ... } catch(e) { return; }`에 해당
    //
    // JavaScript:
    //   function safeParse(s) {
    //     try {
    //       const num = parseInt(s);
    //       if (isNaN(num)) throw new Error("Invalid number");
    //       return num;
    //     } catch(e) {
    //       throw e;
    //     }
    //   }
    // Rust:
    fn safe_parse(s: &str) -> Result<i32, std::num::ParseIntError> {
        // JavaScript: try { return parseInt(s); } catch(e) { throw e; }
        // Rust: `?` 연산자는 오류가 발생하면 함수를 즉시 종료
        let num = s.parse::<i32>()?; // 이 값이 Err이면 함수가 즉시 오류 반환
        // ? 연산자:
        //   - Ok(num)이면 num 값을 추출하여 num 변수에 바인딩
        //   - Err(e)이면 함수를 즉시 Err(e)로 반환
        // JavaScript: try { return parseInt(s); } catch(e) { throw e; }
        // Rust: s.parse::<i32>()? → 동일한 개념 but 타입 안전
        Ok(num) // OK면 Ok로 감싸서 반환
        // JavaScript: return num;
        // Rust: Ok(num) → Result<i32, E>를 반환
    }

    // println!("safe_parse('456'): {:?}", safe_parse("456"));

    // Option<T> - JavaScript의 null/undefined를 처리하는 타입
    // Java의 Optional과 매우 유사
    //
    // JavaScript:
    //   const maybeName = findUser(42); // null 또는 "Alice"
    //   const name = maybeName || 'Guest'; // null이면 'Guest'
    // Rust:
    let maybe_name: Option<String> = Some("Alice".to_string());
    // Some("Alice") → 값이 있음
    // None → 값이 없음
    // JavaScript의 null/undefined 대신 Option<T>를 사용합니다.
    // JavaScript: const name = maybeName || 'Guest';
    // Rust: maybe_name.unwrap_or("Guest".to_string());
    //   unwrap_or() → Some이면 값을 추출, None이면 기본값 반환
    let name = maybe_name.unwrap_or("Guest".to_string());
    println!("   name: {}", name);

    // ? 연산자 - None이면 즉시 None 반환
    //
    // JavaScript:
    //   function getName() {
    //     const name = maybeName;
    //     if (name === null) return 'Guest';
    //     return name;
    //   }
    // Rust:
    // fn demo() -> Option<String> {
    //   let n = maybe_name?; // None이면 즉시 None 반환
    //   Some(n)
    // }
    // ? 연산자는 Option과 Result 모두에서 작동합니다.
    // Option: None이면 즉시 None 반환
    // Result: Err이면 즉시 Err 반환
}

// ============================================================
// Section 14: Generics (제네릭스)
// ============================================================
//
// JavaScript에는 제네릭이 없습니다 (TypeScript는 있음).
// Rust의 제네릭은 TypeScript의 제네릭과 매우 유사합니다.
//
// JavaScript vs Rust 제네릭 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (타입 없음)   │ Rust (제네릭)                           │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ function identity(x) {   │ fn identity<T>(value: T) -> T {        │
// │   return x;               │   value                                 │
// │ }                         │ }                                      │
// │ 타입 안전성 없음            │ 컴파일타임에 타입 검증                  │
// │ identity(42) → 42        │ identity(42) → 42                      │
// │ identity("hello") → "h"  │ identity("hello") → "hello"            │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 제네릭의 주요 용도:
//   1. 타입 추상화: 특정 타입에 의존하지 않는 코드 작성
//   2. 재사용성: 같은 로직을 다양한 타입에 적용
//   3. 타입 안전성: 컴파일타임에 타입 검증
//
// TypeScript와 Rust 제네릭 비교:
//   TypeScript: function identity<T>(value: T): T { return value; }
//   Rust:       fn identity<T>(value: T) -> T { value }
//   둘 다 거의 동일한 문법과_semantics_를 가집니다.

fn section_14_generics() {
    // JavaScript: function identity(x) { return x; }  // 타입 없음
    //
    // JavaScript:
    //   function identity(x) {
    //     return x;  // 어떤 타입든 작동하지만 타입 안전성 없음
    //   }
    //   identity(42); // 42
    //   identity("hello"); // "hello"
    //   const result: string = identity(42); // TypeScript: 컴파일 에러!
    // Rust:
    fn identity<T>(value: T) -> T {
        // <T>는 "T라는 타입 매개변수"를 의미합니다.
        // T는 호출할 때 구체적인 타입으로 결정됩니다.
        // JavaScript: function identity(x) { return x; } (타입 없음)
        // Rust: fn identity<T>(value: T) -> T (타입 안전)
        value // 모든 타입으로 작동
        // TypeScript: return value;
        // Rust: value (표현식이 암시적 반환)
    }
    println!("Section 14 - Generics: identity(42) = {}", identity(42));
    // identity(42) → T = i32
    // identity("hello") → T = &str
    println!("   identity(\"hello\") = {}", identity("hello"));
    // TypeScript:
    //   identity(42); // number
    //   identity("hello"); // string
    // Rust와 TypeScript 모두 컴파일타임에 타입을 검증합니다.

    // JavaScript: [1, "hello", true]  // 배열 요소의 타입이 다를 수 있음
    //
    // JavaScript:
    //   const arr = [1, "hello", true]; // 타입이 다 섞일 수 있음
    //   arr[0] + arr[1]; // "1hello" (예기치 않은 동작)
    // Rust:
    let numbers: Vec<i32> = vec![1, 2, 3];
    // Vec<i32> → 모든 요소가 i32 타입
    // JavaScript의 [1, 2, 3]과 유사하지만, 타입이 고정됨
    // Rust의 제네릭 배열(Vec)은 모든 요소가 같은 타입이어야 함
    println!("   Generic array: {:?}", numbers);

    // JavaScript의 Array.map에 해당하는 Rust의 제네릭 함수
    //
    // JavaScript:
    //   const arr = [10, 20, 30];
    //   function first(arr) { return arr[0]; }
    //   first(arr); // 10
    //   // 타입 안전성 없음: first("string") → undefined
    // Rust:
    fn first<T>(slice: &[T]) -> &T {
        // &[T] → T 타입의 슬라이스 참조 (JavaScript의 배열 참조)
        // &T → T 타입의 참조 반환
        // JavaScript: function first(arr) { return arr[0]; }
        // Rust: fn first<T>(slice: &[T]) -> &T { &slice[0] }
        &slice[0] // 첫 번째 요소 반환
        // JavaScript: return arr[0];
        // Rust: &slice[0] → slice의 첫 번째 요소 참조 반환
    }
    let numbers2: &[i32] = &[10, 20, 30];
    // &[i32] → i32 배열의 불변 참조
    // JavaScript: const arr = [10, 20, 30];
    let first_num = first(numbers2);
    // first([10, 20, 30]) → T = i32
    println!("   first([10, 20, 30]) = {}", first_num);
    // TypeScript:
    //   const arr = [10, 20, 30] as number[];
    //   function first<T>(arr: T[]): T { return arr[0]; }
    //   first(arr); // number
}

// ============================================================
// Section 15: Lifetimes (라이프타임)
// ============================================================
//
// Rust의 라이프타임은 JavaScript에 존재하지 않는 개념입니다.
// JavaScript는 GC(가비지 컬렉터)로 자동으로 메모리를 정리하지만, Rust에는 GC가 없습니다.
// 라이프타임은 컴파일타임에 "이 참조가 얼마나 유효한지"를 보장합니다.
//
// JavaScript vs Rust 라이프타임 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (GC)          │ Rust (Lifetime)                        │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ GC가 자동으로 정리       │ 컴파일러가 라이프타임을 검증           │
// │ 런타임에 동작            │ 컴파일타임에 검증                      │
// │ dangling reference 가능  │ 컴파일러가 dangling reference 방지     │
// │ const f = () => obj.prop │ fn f<'a>(obj: &'a Obj) → &'a T        │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 라이프타임 설명자 ('a):
//   'a는 "이 참조는 최소한 'a만큼의 수명 동안 유효합니다"라는 의미입니다.
//   JavaScript: GC가 알아서 정리
//   Rust: 개발자가 라이프타임을 명시하여 컴파일러가 검증
//
// 라이프타임의 핵심 규칙:
//   1. 각 참조에는 라이프타임이 있습니다
//   2. 컴파일러가 라이프타임을 추론할 수 있으면 명시하지 않아도 됨
//   3. 여러 참조가 있을 때 어느 것이 더 오래 지속되는지 명시 필요

fn section_15_lifetimes() {
    // 대부분의 경우, 라이프타임 없이도 컴파일러가 추론
    //
    // JavaScript:
    //   const obj = { prop: "hello" };
    //   const f = () => obj.prop; // GC가 obj를 정리할 때까지 유효
    // Rust:
    //   대부분의 함수에서 라이프타임을 명시하지 않아도 됩니다.
    //   컴파일러가 자동으로 추론하기 때문입니다.
    //   하지만 여러 참조가 있을 때는 명시적으로 라이프타임을 지정해야 할 수 있습니다.

    // 간단한 예시: 두 문자열 중 더 긴 것 반환
    //
    // JavaScript:
    //   const longest = (a, b) => a.length > b.length ? a : b;
    //   longest("hello", "world!"); // "world!" 반환
    //   // a와 b 중 더 긴 것 반환, 반환된 값은 원본과 독립적
    // Rust:
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        // 'a는 "s1과 s2 중 더 오래 지속되는 수명"을 의미합니다.
        // 즉, 반환된 &str은 s1과 s2 중 더 오래 살아있는 것보다
        // 짧거나 같은 수명을 가집니다.
        //
        // JavaScript:
        //   function longest(a, b) { return a.length > b.length ? a : b; }
        // Rust:
        //   fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
        //
        // 'a의 의미: 반환값이 s1과 s2 모두를 참조할 수 있는 수명
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
        // JavaScript: return a.length > b.length ? a : b;
    }

    let result = longest("hello", "world!");
    // longest("hello", "world!") → "world!" (5 > 5 → false)
    // result는 "world!"를 참조하는 &str입니다.
    // "hello"와 "world!"는 소스 코드에 직접 작성된 리터럴이므로
    // 프로그램 전체 수명을 가집니다 (static lifetime).
    println!(
        "Section 15 - Lifetime: longest('hello', 'world!') = {}",
        result
    );
    // JavaScript: `const longest = (a, b) => a.length > b.length ? a : b;` 동일한 로직
    // 하지만 Rust는 컴파일타임에 "result가 s1/s2보다 오래 살아남지 않을 것"을 보장
    // JavaScript: dangling reference가 발생할 수 있지만 (예: 객체가 GC되면),
    // Rust: 컴파일러가 dangling reference를 방지합니다.

    // 라이프타임 추론 - 대부분의 경우 명시할 필요가 없습니다
    //
    // JavaScript:
    //   function getFirst(str) { return str[0]; }
    //   getFirst("hello"); // "h"
    // Rust:
    fn get_first(s: &str) -> &str {
        // 컴파일러가 라이프타임을 자동으로 추론합니다.
        // 입력 참조(&str)와 출력 참조(&str)의 라이프타임이 동일하므로,
        // 컴파일러가 자동으로 'a를 추론할 수 있습니다.
        // 이는 "lifetime elision"이라고 합니다.
        s
        // JavaScript: return str[0]; (하지만 JavaScript의 문자는 String)
    }
    let word = get_first("hello lifetime");
    // get_first("hello lifetime") → "hello lifetime"의 첫 번째 문자 슬라이스
    println!("   Auto inference: {}", word);
    // JavaScript: console.log(getFirst("hello lifetime")[0]); → "h"
    // Rust: println!("{}", word); → "hello lifetime" 전체 (첫 번째 문자 아님)
    // get_first는 첫 번째 문자가 아니라 전체 문자열을 반환합니다.
    // JavaScript의 str[0]에 해당하는 것은 get_first(&s[..1])입니다.
}

// ============================================================
// Section 16: Async/Await (비동기 처리)
// ============================================================
//
// Rust의 async 처리는 JavaScript의 `async/await`과 매우 유사합니다.
//
// JavaScript vs Rust 비동기 처리 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ async function fetch() { │ async fn fetch() -> Result<..., ...> {  │
// │   const res = await...;  │   let res = ...await;                  │
// │ }                        │ }                                      │
// │ 이벤트 루프              │ tokio 런타임                           │
// │ Promise.all([...])       │ tokio::join!(...)                      │
// │ setTimeout(fn, ms)       │ tokio::time::sleep(Duration::from_...) │
// │ .then() / .catch()       | ? 연산자 / Result                      │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// async/await의 핵심 개념:
//   async fn → 비동기 함수 (Future를 반환)
//   .await → Future가 완료될 때까지 대기
//   Future → "미래에 완료될 작업"을 나타내는 트레이트
//
// JavaScript의 Promise vs Rust의 Future:
//   JavaScript Promise: 실행되면 즉시 시작 (eager)
//   Rust Future: .await할 때까지 시작하지 않음 (lazy)

fn section_16_async_example() {
    println!("Section 16 - Async: Async 처리 예제");
    // JavaScript:
    //   async function example() {
    //     console.log("Before sleep");
    //     await new Promise(resolve => setTimeout(resolve, 100));
    //     console.log("After sleep");
    //   }
    // Rust:
    //   async fn example() {
    //     println!("Before sleep");
    //     tokio::time::sleep(Duration::from_millis(100)).await;
    //     println!("After sleep");
    //   }
    println!("   Rust는 JavaScript와 동일한 async/await 패턴을 사용합니다!");
    // JavaScript:
    //   async function fetch(url) {
    //     const res = await fetch(url);
    //     const data = await res.json();
    //     return data;
    //   }
    // Rust:
    //   async fn fetch(url: &str) -> Result<Data, Error> {
    //     let res = reqwest::get(url).await?;
    //     let data = res.json().await?;
    //     Ok(data)
    //   }
    // 둘 다 동일하게 읽을 수 있는 코드를 작성할 수 있습니다.
}

// ============================================================
// Section 17: Modules & Crates (모듈 & 크레이트)
// ============================================================
//
// Rust의 모듈 시스템은 JavaScript의 ES6 Module(import/export)에 해당합니다.
//
// JavaScript vs Rust 모듈 시스템 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript ES6 Module    │ Rust Module                            │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ import { foo } from...   │ use crate::foo;                        │
// │ export function bar(){}  │ pub fn bar() {}                        │
// │ export default class...  │ pub struct... / impl...                │
// │ ./relative/path.js       │ crate::module::path                    │
// │ npm Package              │ Cargo Crate                            │
// │ package.json             │ Cargo.toml                             │
// │ node_modules/            │ ~/.cargo/registry/                     │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 가시성 (Visibility):
//   JavaScript: 모든 것이 기본적으로 공개 (export 하면 외부 사용)
//   Rust: 모든 것이 기본적으로 비공개 (pub 없으면 외부 사용 불가)
//   pub → 외부에서 접근 가능 (JavaScript의 export와 유사)
//   pub(crate) → 같은 크레이트 내부에서만 접근 가능
//
// Crate vs Package:
//   Crate → 컴파일 단위 (JavaScript의 파일/module에 해당)
//   Package → Cargo 프로젝트 (JavaScript의 npm package에 해당)
//   한 Package에 여러 Crate 포함 가능 (예: main crate + test crate)

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
    //
    // Rust의 동일한 구조:
    //   // lib.rs 또는 math.rs
    //   pub fn add(a: i32, b: i32) -> i32 { a + b }
    //
    //   // main.rs
    //   use crate::add;

    // Rust에서 `mod` 키워드로 모듈(네임스페이스)을 생성합니다
    // JavaScript의 `import`에 해당
    //
    // JavaScript:
    //   // math.js
    //   export function add(a, b) { return a + b; }
    //   export function subtract(a, b) { return a - b; }
    // Rust:
    //   pub fn add(a: i32, b: i32) -> i32 { a + b }
    //   pub fn subtract(a: i32, b: i32) -> i32 { a - b }

    // JavaScript: `export function add(a, b) { return a + b; }`
    //
    // JavaScript:
    //   export function add(a, b) {
    //     return a + b;
    //   }
    // Rust:
    fn add(a: i32, b: i32) -> i32 {
        a + b
        // JavaScript: return a + b;
        // Rust: 마지막 표현식이 암시적 반환
    }

    // JavaScript: `export function subtract(a, b) { return a - b; }`
    //
    // JavaScript:
    //   export function subtract(a, b) {
    //     return a - b;
    //   }
    // Rust:
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // `pub` 키워드로 외부에서 접근 가능하게 합니다 (JavaScript의 `export`)
    //
    // JavaScript:
    //   export function multiply(a, b) { return a * b; }
    // Rust:
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    // pub 없이 정의된 add/subtract는 이 모듈 내부에서만 사용 가능합니다.
    // JavaScript의 export 없이 정의된 함수와 유사합니다 (외부에서 사용 불가).

    pub fn divide(a: i32, b: i32) -> Result<f64, String> {
        // JavaScript:
        //   export function divide(a, b) {
        //     if (b === 0) throw new Error("Division by zero!");
        //     return a / b;
        //   }
        if b == 0 {
            Err("Division by zero!".to_string()) // JavaScript: throw new Error("Division by zero!")
            // JavaScript의 throw는 Rust의 Err에 해당합니다.
            // JavaScript: throw new Error("message")
            // Rust: Err("message".to_string())
        } else {
            Ok(a as f64 / b as f64) // JavaScript: return a / b
            // JavaScript의 return은 Rust의 Ok()에 해당합니다.
            // JavaScript: return a / b
            // Rust: Ok(a / b) → Result<f64, String>
            // as f64는 정수를 부동 소수점으로 변환합니다 (cast).
            // JavaScript: const result = a / b; (자동 타입 변환)
            // Rust: const result = a as f64 / b as f64; (명시적 변환)
        }
    }

    // JavaScript의 `import`에 해당하는 것이 Rust의 `use` 키워드
    //
    // JavaScript:
    //   import { add, multiply } from './math.js';
    //   console.log(add(5, 3)); // 8
    //   console.log(multiply(4, 7)); // 28
    // Rust:
    //   use crate::module::{add, multiply};
    //   println!("{}", add(5, 3)); // 8
    //   println!("{}", multiply(4, 7)); // 28

    // 모듈 내의 함수 호출
    //
    // JavaScript:
    //   import { add, multiply } from './math';
    //   const result_add = add(5, 3);
    //   const result_multiply = multiply(4, 7);
    // Rust:
    let result_add = add(5, 3);
    let result_multiply = multiply(4, 7);
    let result_divide = divide(10, 2).unwrap_or(0.0);
    // divide(10, 2) → Ok(5.0)
    // unwrap_or(0.0) → Ok이면 값을 추출, Err이면 0.0 반환
    // JavaScript: const result = divide(10, 2) || 0;
    println!(
        "   add(5, 3) = {}, multiply(4, 7) = {}, divide(10, 2) = {:.1}",
        result_add, result_multiply, result_divide
    );

    // JavaScript의 `export default`에 해당하는 패턴
    //
    // JavaScript:
    //   export default class Calculator {
    //     constructor() { this.history = []; }
    //     add(a, b) { this.history.push(`${a} + ${b}`); return a + b; }
    //     getHistory() { return this.history; }
    //   }
    //   import Calculator from './math';
    // Rust:
    struct Calculator {
        history: Vec<String>, // JavaScript: private field처럼 사용 (pub 없으면 외부 접근 불가)
        // JavaScript의 class에서는 this.history = [];로 private 필드를 선언하고,
        // Rust에서는 pub 없이 필드를 정의하면 외부에서 접근할 수 없습니다.
        // JavaScript: Calculator의 private 필드
        // Rust: Calculator의 private 필드 (pub 없음)
    }

    impl Calculator {
        // impl은 JavaScript의 class 메서드를 정의하는 것에 해당합니다.
        // JavaScript: Calculator.prototype.add = function(a, b) { ... }
        // Rust: impl Calculator { fn add(&mut self, a: i32, b: i32) -> i32 { ... } }

        fn new() -> Self {
            // JavaScript: `constructor() { this.history = []; }`
            // JavaScript:
            //   constructor() {
            //     this.history = [];
            //   }
            // Rust:
            Calculator {
                history: Vec::new(),
                // JavaScript: this.history = []
                // Rust: Vec::new() → 빈 Vec 생성
            }
        }

        // JavaScript: `add(a, b) { this.history.push(\`${a} + ${b}\`); return a + b; }`
        //
        // JavaScript:
        //   add(a, b) {
        //     this.history.push(`${a} + ${b}`);
        //     return a + b;
        //   }
        // Rust:
        fn add(&mut self, a: i32, b: i32) -> i32 {
            // &mut self → this에 해당 (변경 가능 참조)
            let result = a + b;
            self.history.push(format!("{} + {} = {}", a, b, result));
            // JavaScript: this.history.push(`${a} + ${b} = ${result}`);
            // format!은 JavaScript의 템플릿 리터럴과 동일합니다.
            result
        }

        fn get_history(&self) -> &Vec<String> {
            // JavaScript: getHistory() { return this.history; }
            // Rust: fn get_history(&self) -> &Vec<String>
            &self.history
            // JavaScript: return this.history;
            // Rust: &self.history → this.history의 참조 반환
        }
    }

    let mut calc = Calculator::new();
    // JavaScript: const calc = new Calculator();
    calc.add(10, 20);
    calc.add(30, 40);
    // JavaScript: calc.add(10, 20); calc.add(30, 40);
    println!("   Calculator history: {:?}", calc.get_history());
    // JavaScript: console.log(calc.getHistory());
    // Rust: {:?}는 Debug trait을 사용하여 출력합니다.

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
    // JavaScript의 import/export와 Rust의 use/pub 비교:
    //   JavaScript: export function add() { ... }
    //   Rust: pub fn add() { ... }
    //   JavaScript: import { add } from './math';
    //   Rust: use crate::math::add;
    //
    // `use`로 모듈의 함수를 가져오기:
    //   `use crate::module_name::function_name;` (crate = 현재 프로젝트)
    //   `use std::collections::HashMap;` (std 라이브러리)
    //   JavaScript: import { HashMap } from 'std/collections';

    // JavaScript의 npm Package = Rust의 Crate
    //
    // JavaScript                  Rust
    // ──────────────────────────  ──────────────────────────────────
    // npm Package                 Cargo Crate
    // package.json                Cargo.toml
    // node_modules/               ~/.cargo/registry/
    // npm install                 cargo build
    // npm publish                 cargo publish
    // https://npmjs.com           https://crates.io
    //
    // JavaScript: `"dependencies": { "express": "^4.18.0" }`
    // Rust: `express = "4.18"` (Cargo.toml)
    //   ^4.18.0 → 4.18.x 버전 (메이너 버전 고정)
    //   4.18 → 4.18.* 버전 (마이너 버전 고정)
}

// ============================================================
// Section 18: File I/O (파일 입출력)
// ============================================================
//
// Rust의 File I/O는 JavaScript의 `fs` 모듈 (Node.js)에 해당합니다.
//
// JavaScript (Node.js) vs Rust File I/O 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (Node.js)     │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ const fs = require('fs') │ use std::fs;                            │
// │ fs.readFileSync(...)     │ fs::read_to_string(...)                 │
// │ fs.writeFileSync(...)    │ fs::write(...)                          │
// │ fs.existsSync(...)       │ Path::exists(...)                       │
// │ fs.mkdirSync(...)        │ fs::create_dir_all(...)                │
// │ fs.readdirSync(...)      │ fs::read_dir(...)                       │
// │ fs.rmSync(...)           │ fs::remove_file(...)                   │
// │ fs.statSync(...)         │ fs::metadata(...)                       │
// │ try/catch                │ Result<T, E> + match                   │
// │ async/await              | ? 연산자 + async/await                 │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 에러 처리 비교:
//   JavaScript: try { const data = fs.readFileSync('file.txt'); } catch(e) { ... }
//   Rust: match fs::read_to_string("file.txt") { Ok(data) => ..., Err(e) => ... }
//   JavaScript의 try/catch는 런타임에 동작하지만,
//   Rust의 Result는 컴파일타임에 에러 처리를 강제합니다.

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
    //
    // JavaScript:
    //   const fs = require('fs');
    //   const content = fs.readFileSync('README.md', 'utf8');
    //   console.log(content);
    // Rust:
    //   let content = std::fs::read_to_string("README.md");
    //   println!("{}", content);
    //
    // 읽기 실패 시 Result::Err를 반환 (try/catch 대신)
    // JavaScript: try { content = fs.readFileSync('nonexistent.txt'); } catch(e) { ... }
    // Rust: Result를 match로 처리
    //
    // JavaScript:
    //   try {
    //     const content = fs.readFileSync('Cargo.toml', 'utf8');
    //     console.log(content);
    //   } catch(e) {
    //     console.error('File read error:', e.message);
    //   }
    // Rust:
    let read_result = std::fs::read_to_string("Cargo.toml");
    match read_result {
        Ok(content) => {
            // 파일이 성공적으로 읽혔을 때
            // JavaScript: console.log(content);
            let line_count = content.lines().count();
            // JavaScript: console.log(content.split('\n').length);
            println!("   Cargo.toml 읽기 성공! ({}줄)", line_count);
            // 첫 줄만 출력
            if let Some(first_line) = content.lines().next() {
                // JavaScript: console.log(content.split('\n')[0]);
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
    //
    // JavaScript:
    //   const fs = require('fs');
    //   fs.writeFileSync('hello.txt', 'Hello from Rust!');
    // Rust:
    //   std::fs::write("hello.txt", "Hello from Rust!");
    //
    // 쓰기 성공 시 Result::Ok(()) 반환
    // JavaScript:
    //   try {
    //     fs.writeFileSync('hello.txt', 'Hello from Rust!');
    //     console.log('File written successfully');
    //   } catch(e) {
    //     console.error('Write error:', e.message);
    //   }
    let write_result = std::fs::write("hello_rust.txt", "Hello from Rust!\nThis is a test file.\n");
    match write_result {
        Ok(()) => println!("   파일 쓰기 성공! (hello_rust.txt 생성)"),
        // JavaScript: console.log('File written successfully');
        Err(e) => println!("   파일 쓰기 실패: {}", e),
        // JavaScript: console.error('Write error:', e.message);
    }

    // JavaScript의 `fs.existsSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   if (fs.existsSync('file.txt')) {
    //     console.log('File exists');
    //   }
    // Rust:
    //   use std::path::Path;
    //   if Path::new("file.txt").exists() { ... }
    use std::path::Path;
    let cargo_toml_exists = Path::new("Cargo.toml").exists();
    // JavaScript: fs.existsSync('Cargo.toml')
    let nonexistent_exists = Path::new("nonexistent_file.txt").exists();
    // JavaScript: fs.existsSync('nonexistent_file.txt')
    println!("   Cargo.toml 존재: {}", cargo_toml_exists);
    println!("   nonexistent_file.txt 존재: {}", nonexistent_exists);

    // JavaScript의 `fs.mkdirSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   fs.mkdirSync('temp', { recursive: true });
    // Rust:
    //   std::fs::create_dir_all("temp");
    // create_dir_all은 중첩 디렉토리도 함께 생성합니다.
    // JavaScript의 { recursive: true }와 동일합니다.
    let mkdir_result = std::fs::create_dir_all("tutorial_temp_dir");
    match mkdir_result {
        Ok(()) => println!("   디렉토리 생성 성공! (tutorial_temp_dir)"),
        Err(e) => println!("   디렉토리 생성 실패: {}", e),
    }

    // JavaScript의 `fs.readdirSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   const files = fs.readdirSync('.');
    //   console.log(files.slice(0, 5));
    // Rust:
    //   for entry in std::fs::read_dir(".")? { ... }
    // read_dir는 Iterator를 반환합니다.
    let dir_result = std::fs::read_dir(".");
    if let Ok(entries) = dir_result {
        // JavaScript: const files = fs.readdirSync('.');
        let mut count = 0;
        for entry in entries.take(5) {
            // 최대 5개만 표시 (JavaScript: files.slice(0, 5))
            // JavaScript: files.slice(0, 5) → 배열의 처음 5개
            // Rust: entries.take(5) → Iterator의 처음 5개
            if let Ok(e) = entry {
                // JavaScript: for (const file of files.slice(0, 5)) { ... }
                if let Some(name) = e.path().file_name() {
                    // JavaScript: console.log(file);
                    println!("   디렉토리 항목: {:?}", name);
                    count += 1;
                }
            }
        }
        println!("   (총 {}개 중 5개 표시)", count);
    }

    // JavaScript의 `fs.rmSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   fs.rmSync('hello.txt');
    //   fs.rmSync('temp_dir', { recursive: true });
    // Rust:
    //   std::fs::remove_file("hello.txt");
    //   std::fs::remove_dir_all("temp_dir");
    // cleanup:
    let _ = std::fs::remove_file("hello_rust.txt");
    let _ = std::fs::remove_dir_all("tutorial_temp_dir");
    // _는 반환값을 무시한다는 의미입니다.
    // JavaScript: try { fs.rmSync('hello.txt'); } catch {} (에러 무시)

    // JavaScript의 `fs.statSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   const stats = fs.statSync('Cargo.toml');
    //   console.log(stats.size); // 바이트 수
    //   console.log(stats.mtime); // 수정 시간
    // Rust:
    //   let metadata = std::fs::metadata("Cargo.toml");
    //   metadata.len() → 바이트 수
    //   metadata.modified() → 수정 시간
    if let Ok(metadata) = std::fs::metadata("Cargo.toml") {
        println!("   Cargo.toml 크기: {} bytes", metadata.len());
        // JavaScript: console.log(stats.size);
        println!(
            "   수정 시간: {:?}",
            metadata
                .modified()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH)
        );
        // JavaScript: console.log(stats.mtime);
        // unwrap_or_else → Err이면 UNIX_EPOCH (1970-01-01) 반환
    }

    // JavaScript의 `fs.appendFileSync()`에 해당하는 Rust 함수
    //
    // JavaScript:
    //   const fs = require('fs');
    //   fs.appendFileSync('log.txt', 'new line\n');
    //   // 또는:
    //   const stream = fs.createWriteStream('log.txt', { flags: 'a' });
    //   stream.write('new line\n');
    // Rust:
    //   std::fs::OpenOptions::new()
    //     .append(true)
    //     .open("file.txt")
    //   → 파일에 내용을 추가합니다.
    //   JavaScript: fs.appendFileSync('log.txt', 'new line\n');
    //   Rust: std::fs::OpenOptions::new().append(true).open("file.txt")
}

// ============================================================
// Section 19: Testing (테스트)
// ============================================================
//
// Rust의 테스트 시스템은 JavaScript의 Jest/Mocha/Vitest에 해당합니다.
//
// JavaScript (Jest) vs Rust 테스트 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (Jest)        │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ test('adds 1+2', () => { │ #[test]                                  │
// │   expect(add(1,2)).toBe( │ fn test_add() {                        │
// │     3);                   │   assert_eq!(add(1,2), 3);             │
// │ })                       │ }                                      │
// │ describe('math', () => { │ #[cfg(test)]                            │
// │   test('add', ...)       │ mod tests {                            │
// │ })                       │   #[test] fn test_add() { ... }        │
// │ jest --coverage          │ cargo test -- --nocapture              │
// │ test.skip(...)           │ #[ignore]                               │
// │ expect(fn).toThrow()     │ #[should_panic]                        │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 테스트 실행:
//   JavaScript: npm test / jest
//   Rust: cargo test
//   JavaScript: jest --testNamePattern='test_add'
//   Rust: cargo test test_add
//
// assertion 비교:
//   JavaScript              Rust                    의미
//   expect(x).toBe(3)     assert_eq!(x, 3)    값이 동일
//   expect(x).toBeTruthy()  assert!(x)              참
//   expect(x).toBeFalsy()   assert!(!x)             거짓
//   expect(x).not.toBe(3)   assert_ne!(x, 3)        값이 다름
//   expect(fn).toThrow()    #[should_panic]         패닉 발생

fn section_19_testing() {
    println!("Section 19 - Testing");

    // JavaScript (Jest):
    //   test('adds 1 + 2', () => {
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
    //
    // assertion 비교표:
    //   JavaScript (Jest)          Rust                 설명
    //   ──────────────────────     ──────────────────   ────────────────────────
    //   expect(x).toBe(3)          assert_eq!(x, 3)     값이 동일
    //   expect(x).toEqual([1,2])   assert_eq!(x, vec![1,2])  배열이 동일
    //   expect(x).toBeTruthy()     assert!(x)           참
    //   expect(x).toBeFalsy()      assert!(!x)          거짓
    //   expect(x).not.toBe(3)      assert_ne!(x, 3)     값이 다름
    //   expect(x).toBeGreaterThan(5) assert!(x > 5)    5보다 큼
    //   expect(fn).toThrow()       #[should_panic]      패닉 발생

    // 테스트할 함수 정의
    //
    // JavaScript:
    //   function addPositive(a, b) { return a + b; }
    //   function isEven(n) { return n % 2 === 0; }
    // Rust:
    fn add_positive(a: i32, b: i32) -> i32 {
        a + b
    }

    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // JavaScript의 `describe()` 블록에 해당하는 것이 Rust의 `mod tests`
    //
    // JavaScript:
    //   describe('math', () => {
    //     test('add', () => { ... });
    //     test('isEven', () => { ... });
    //   });
    //
    // Rust:
    //   #[cfg(test)]
    //   mod tests {
    //     use super::*;
    //     #[test]
    //     fn test_add() { ... }
    //     #[test]
    //     fn test_is_even() { ... }
    //   }
    //
    // #[cfg(test)]은 "테스트 빌드일 때만 이 모듈을 포함하세요"라는 의미입니다.
    // JavaScript: 테스트 파일은 별도로 관리하지만,
    // Rust: 테스트 코드를 소스 파일 내에 직접 작성할 수 있습니다.

    // JavaScript의 `beforeEach()`에 해당하는 것이 Rust의 `#[before]` (없음)
    // 대신 함수 내에서 직접 초기화
    //
    // JavaScript:
    //   let sharedData;
    //   beforeEach(() => { sharedData = createData(); });
    // Rust:
    //   fn test_something() {
    //     let shared_data = create_data(); // 각 테스트에서 직접 초기화
    //   }

    // JavaScript의 `afterEach()`에 해당하는 것이 Rust의 `#[after]` (없음)
    // 대신 함수 내에서 직접 정리
    //
    // JavaScript:
    //   afterEach(() => { cleanup(); });
    // Rust:
    //   fn test_something() {
    //     let resource = acquire_resource();
    //     // ... 테스트 ...
    //     drop(resource); // 함수 종료 시 자동으로 정리
    //   }

    // JavaScript의 `it('should throw', () => { expect(fn).toThrow(); })`
    // Rust: `#[should_panic]`
    //
    // JavaScript:
    //   test('divides by zero', () => {
    //     expect(() => divide(1, 0)).toThrow();
    //   });
    // Rust:
    //   #[test]
    //   #[should_panic]
    //   fn test_divide_by_zero() {
    //     divide(1, 0); // 패닉이어야 함
    //   }

    // JavaScript의 `test.skip('...', ...)`에 해당하는 것이 Rust의 `#[ignore]`
    // JavaScript의 `test.only('...', ...)`에 해당하는 것이 Rust의 `--exact` 플래그
    //
    // JavaScript: test.skip('slow test', ...);
    // Rust: #[test] #[ignore] fn test_slow() { ... }
    //   cargo test ignored → 무시된 테스트만 실행

    // 실제 테스트 실행
    //
    // JavaScript:
    //   test('addPositive(1, 2) equals 3', () => {
    //     expect(addPositive(1, 2)).toBe(3);
    //   });
    // Rust:
    //   #[test]
    //   fn test_add_positive() {
    //     assert_eq!(add_positive(1, 2), 3);
    //   }
    let test_value = add_for_test(3, 4);
    println!("   add_for_test(3, 4) = {} (expect: 7)", test_value);
    // JavaScript: expect(addForTest(3, 4)).toBe(7);

    // JavaScript: expect(isEven(4)).toBeTruthy();
    // Rust: assert!(is_even_for_test(4));
    println!(
        "   is_even_for_test(4) = {} (expect: true)",
        is_even_for_test(4)
    );
    // JavaScript: expect(isEven(4)).toBeTruthy();

    // JavaScript: expect(isEven(3)).toBeFalsy();
    // Rust: assert!(!is_even_for_test(3));
    println!(
        "   is_even_for_test(3) = {} (expect: false)",
        is_even_for_test(3)
    );
    // JavaScript: expect(isEven(3)).toBeFalsy();

    // JavaScript의 `expect(value).toBeGreaterThan(5)`에 해당하는 것이 Rust의 `assert!(value > 5)`
    let result = add_for_test(10, 5);
    println!("   add(10, 5) = {} (> 5: {})", result, result > 5);
    // JavaScript: expect(add(10, 5)).toBeGreaterThan(5);

    // JavaScript의 `expect(value).toBeInstanceOf(Array)`에 해당하는 것이 Rust의 `assert!(value.is::<Vec<_>>())`
    let numbers = vec![1, 2, 3, 4, 5];
    println!("   Vec is array-like: {}", !numbers.is_empty());
    // JavaScript: expect(numbers).toBeInstanceOf(Array);

    // 테스트 실행 방법:
    //   JavaScript: npm test 또는 jest
    //   Rust: cargo test
    //
    //   JavaScript: jest --testNamePattern='test_add'
    //   Rust: cargo test test_add
    //
    //   JavaScript: jest --coverage
    //   Rust: cargo test -- --show-output (또는 cargo llvm-cov for coverage)
    //
    //   JavaScript: npm test -- --watch
    //   Rust: cargo test -- --watch (Rust 1.70+)

    // Rust의 테스트는 컴파일타임에 타입 체크가 되기 때문에
    // JavaScript의 테스트보다 훨씬 안전한 편입니다
    //
    // JavaScript:
    //   test('add', () => {
    //     expect(add("1", 2)).toBe(3); // 런타임 에러! (12 !== 3)
    //   });
    // Rust:
    //   #[test]
    //   fn test_add() {
    //     assert_eq!(add(1, 2), 3); // 컴파일타임 에러 방지!
    //   }
    // Rust의 타입 시스템은 테스트 전에 타입 오류를 발견합니다.
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
    //
    // JavaScript:
    //   test('adds 1 + 2 to equal 3', () => {
    //     expect(add(1, 2)).toBe(3);
    //   });
    // Rust:
    #[test]
    fn test_add() {
        assert_eq!(add_for_test(1, 2), 3);
        // JavaScript: expect(add(1, 2)).toBe(3);
    }

    // JavaScript: `test('returns negative result', () => { expect(add(-1, -2)).toBe(-3); });`
    //
    // JavaScript:
    //   test('returns negative result', () => {
    //     expect(add(-1, -2)).toBe(-3);
    //   });
    // Rust:
    #[test]
    fn test_negative_add() {
        assert_eq!(add_for_test(-1, -2), -3);
    }

    // JavaScript: `test('isEven', () => { expect(isEven(4)).toBeTruthy(); expect(isEven(3)).toBeFalsy(); });`
    //
    // JavaScript:
    //   test('isEven', () => {
    //     expect(isEven(4)).toBeTruthy();
    //     expect(isEven(3)).toBeFalsy();
    //   });
    // Rust:
    #[test]
    fn test_is_even() {
        assert!(is_even_for_test(4));
        // JavaScript: expect(isEven(4)).toBeTruthy();
        assert!(!is_even_for_test(3));
        // JavaScript: expect(isEven(3)).toBeFalsy();
    }

    // JavaScript: `test('should throw', () => { expect(fn).toThrow(); });`
    //
    // JavaScript:
    //   test('should throw', () => {
    //     expect(fn).toThrow('error message');
    //   });
    // Rust:
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic"); // JavaScript: throw new Error("test");
    }

    // JavaScript의 `describe('math', () => { ... });`에 해당하는 것이 Rust의 `mod`
    //
    // JavaScript:
    //   describe('math', () => {
    //     test('filter evens', () => { ... });
    //     test('map doubles', () => { ... });
    //   });
    // Rust:
    mod advanced_tests {
        use super::*;

        // JavaScript: `test('filter evens', () => { ... });`
        //
        // JavaScript:
        //   test('filter evens', () => {
        //     expect(filter([1,2,3,4,5]).filter(x => x%2==0)).toEqual([2,4]);
        //   });
        // Rust:
        #[test]
        fn test_filter_evens() {
            let numbers = vec![1, 2, 3, 4, 5, 6];
            let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
            assert_eq!(evens, vec![2, 4, 6]);
        }

        // JavaScript: `test('map doubles', () => { ... });`
        //
        // JavaScript:
        //   test('map doubles', () => {
        //     expect(map([1,2,3], x => x*2)).toEqual([2,4,6]);
        //   });
        // Rust:
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
//
// Rust의 Iterator 트레이트은 JavaScript의 배열 메서드와 제네레이터에 해당합니다.
//
// JavaScript vs Rust Iterator 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ arr.map(x => x * 2)      │ arr.iter().map(|x| x * 2).collect()    │
// │ arr.filter(x => x > 0)   │ arr.iter().filter(|x| *x > 0).collect()│
// │ arr.reduce((a,x) => a+x, │ arr.iter().fold(0, |a,x| a+x)         │
// │   0)                     │                                         │
// │ arr.forEach(x => fn(x))  │ arr.iter().for_each(|x| fn(x))    │
// │ arr.find(x => x > 3)     │ arr.iter().find(|x| *x > 3)            │
// │ arr.some(x => x > 5)     │ arr.iter().any(|x| *x > 5)             │
// │ arr.every(x => x > 0)    │ arr.iter().all(|x| *x > 0)             │
// │ arr.slice(0, 3)          │ arr.iter().take(3).copied().collect()  │
// │ arr.slice(3)             │ arr.iter().skip(3).copied().collect()  │
// │ arr.entries()            │ arr.iter().enumerate()                 │
// │ [...arr1, ...arr2]       │ arr1.iter().chain(arr2.iter())         │
// │ function* range() { yield │ Iterator는 Lazy (collect까지 실행 안 함) │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Lazy Evaluation (지연 평가):
//   JavaScript의 배열 메서드는 즉시 실행됩니다 (eager).
//   JavaScript: arr.map(x => x * 2).filter(x => x > 5);
//     → map이 먼저 실행되어 새 배열 생성 → filter가 실행
//   Rust의 Iterator는 Lazy입니다 (lazy).
//   Rust: arr.iter().map(|x| x * 2).filter(|x| *x > 5).collect();
//     → collect()에서じめて 실제로 실행됨
//     → 한 번의 순회로 모든 처리를 완료 (최적화!)
//
// 성능 비교:
//   JavaScript: map + filter + slice → 3번의 배열 생성 + 3번의 순회
//   Rust: map + filter + take → 1번의 순회 (컴파일타임 최적화)
//
// Iterator의 2가지 카테고리 (핵심 개념!):
//   1. Adapter (변환자) - Lazy, 즉시 실행 안 함
//      .map(), .filter(), .take(), .skip(), .chain() 등
//      이 메서드들은 새로운 Iterator를 반환할 뿐, 실제 처리는 안 함
//   2. Consumer (소비자) - Eager, 즉시 실행 함
//      .collect(), .for_each(), .sum(), .count() 등
//      이 메서드들이 Iterator의 "트리거"로, 이때じめて 실제 처리가 실행됨
//   JavaScript의 map/filter는 모두 즉시 실행되지만 (adapter + consumer가 통합되어),
//   Rust는 adapter와 consumer가 분리되어 있어 성능 최적화가 가능합니다.
//   예를 들어: arr.iter().map(f1).map(f2).map(f3).collect()
//     → JavaScript: 3개의 intermediate array 생성
//     → Rust: 한 번의 순회로 f1 → f2 → f3 체인 처리 (중간 배열 없음!)

fn section_20_iterators() {
    println!("Section 20 - Iterators");

    // JavaScript의 for...of와 Rust의 for...in 비교
    //
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
    // JavaScript의 for...of는 배열의 각 값을 복사해서 제공합니다.
    // Rust의 for num in &arr는 배열의 각 값을 참조해서 제공합니다 (복사 없음).
    // JavaScript: for (const num of arr) → num은 number 복사
    // Rust: for num in &arr → num은 &i32 참조

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // JavaScript의 Array.map() = Rust의 Iterator::map()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const doubled = arr.map(x => x * 2); // [2, 4, 6, 8, 10]
    // Rust:
    //   let arr = vec![1, 2, 3, 4, 5];
    //   let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    // numbers.iter() → &i32의 Iterator 생성
    // .map(|x| x * 2) → 각 요소를 2배로 변환
    // .collect() → Vec<i32>로 결과 모음
    // JavaScript: arr.map()은 즉시 새 배열을 생성하지만,
    // Rust: map()은 Lazy Iterator이므로 collect()에서じめて 생성됨.
    println!("   map(x*2): {:?}", doubled);

    // JavaScript의 Array.filter() = Rust의 Iterator::filter()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5, 6];
    //   const evens = arr.filter(x => x % 2 === 0); // [2, 4, 6]
    // Rust:
    //   let evens: Vec<i32> = arr.iter().filter(|x| *x % 2 == 0).copied().collect();
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    // .filter(|x| **x % 2 == 0) → x는 &i32, **x는 i32
    // JavaScript: filter(x => x % 2 === 0) → x는 number
    // Rust: filter(|x| **x % 2 == 0) → x는 &i32
    // .copied() → &i32를 i32로 복사 (Copy trait이므로 빠름)
    println!("   filter(evens): {:?}", evens);

    // JavaScript의 Array.reduce() = Rust의 Iterator::fold()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const sum = arr.reduce((acc, x) => acc + x, 0); // 15
    // Rust:
    //   let sum: i32 = arr.iter().fold(0, |acc, x| acc + x);
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    // fold(init, closure) → reduce에 해당
    //   init: 초기값 (0)
    //   closure: |acc, x| → acc는 누적값, x는 현재 요소
    // JavaScript의 reduce는 배열 요소를 직접 사용하지만,
    // Rust의 fold는 Iterator의 요소 (참조)를 사용합니다.
    println!("   fold(sum): {}", sum);

    // JavaScript의 Array.forEach() = Rust의 for loop (or Iterator::for_each())
    //
    // JavaScript:
    //   arr.forEach(x => console.log(x));
    // Rust:
    //   for x in &arr { println!("{}", x); }
    //   arr.iter().for_each(|x| println!("{}", x));
    // JavaScript의 forEach는 콜백 함수를 호출하지만,
    // Rust의 for_each도 콜백 함수를 호출합니다.
    // JavaScript: arr.forEach(x => console.log(x));
    // Rust: arr.iter().for_each(|x| println!("{}", x));
    println!("   for_each:");
    numbers.iter().take(3).for_each(|x| print!("   {} ", x));
    // JavaScript: arr.slice(0, 3).forEach(x => console.log(x));
    println!(); // 줄바꿈 (JavaScript: console.log()와 동일)

    // JavaScript의 Array.find() = Rust의 Iterator::find()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const first = arr.find(x => x > 3); // 4
    //   console.log(first); // 4
    // Rust:
    //   let first = arr.iter().find(|x| *x > 3); // Some(&4)
    //   println!("{}", first.unwrap()); // 4
    // JavaScript의 find는 값을 직접 반환하지만,
    // Rust의 find는 참조를 반환합니다 (Option<&T>).
    if let Some(first) = numbers.iter().find(|x| **x > 3) {
        println!("   find(> 3): {}", first);
    }

    // JavaScript의 Array.some() = Rust의 Iterator::any()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const hasLarge = arr.some(x => x > 5); // true
    // Rust:
    //   let has_large = arr.iter().any(|x| *x > 5); // true
    // JavaScript의 some은 "하나라도 조건을 만족하면 true"
    // Rust의 any도 동일한 개념입니다.
    let has_large = numbers.iter().any(|x| *x > 5);
    println!("   any(> 5): {}", has_large);

    // JavaScript의 Array.every() = Rust의 Iterator::all()
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const allPositive = arr.every(x => x > 0); // true
    // Rust:
    //   let all_positive = arr.iter().all(|x| *x > 0); // true
    // JavaScript의 every은 "모든 요소가 조건을 만족하면 true"
    // Rust의 all도 동일한 개념입니다.
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("   all(> 0): {}", all_positive);

    // JavaScript의 Array.slice(0, 3) = Rust의 Iterator::take(3)
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const first3 = arr.slice(0, 3); // [1, 2, 3]
    // Rust:
    //   let first3: Vec<i32> = arr.iter().take(3).copied().collect();
    // JavaScript의 slice는 배열의 일부를 잘라내지만,
    // Rust의 take은 Iterator의 처음 N개만 허용합니다.
    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    println!("   take(3): {:?}", first_three);

    // JavaScript의 arr.slice(3) = Rust의 Iterator::skip(3)
    //
    // JavaScript:
    //   const arr = [1, 2, 3, 4, 5];
    //   const rest = arr.slice(3); // [4, 5]
    // Rust:
    //   let rest: Vec<i32> = arr.iter().skip(3).copied().collect();
    // JavaScript의 slice(3)은 인덱스 3부터 끝까지
    // Rust의 skip(3)은 처음 3개를 건너뛰고 나머지를 반환
    let rest: Vec<i32> = numbers.iter().skip(3).copied().collect();
    println!("   skip(3): {:?}", rest);

    // JavaScript의 arr.entries() = Rust의 Iterator::enumerate()
    //
    // JavaScript:
    //   const arr = ['a', 'b', 'c'];
    //   for (const [i, val] of arr.entries()) {
    //     console.log(i, val); // [0, 'a'], [1, 'b'], [2, 'c']
    //   }
    // Rust:
    //   for (i, val) in arr.iter().enumerate() {
    //     println!("{}: {}", i, val);
    //   }
    // JavaScript의 entries()는 [인덱스, 값] 튜플을 반환
    // Rust의 enumerate()도 (인덱스, 값) 튜플을 반환
    println!("   enumerate:");
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("     [{}]: {}", i, val);
    }

    // JavaScript의 arr.concat(arr2) = Rust의 Iterator::chain()
    //
    // JavaScript:
    //   const arr1 = [1, 2, 3];
    //   const arr2 = [4, 5, 6];
    //   const combined = [...arr1, ...arr2]; // [1, 2, 3, 4, 5, 6]
    // Rust:
    //   let combined: Vec<i32> = arr1.iter().chain(arr2.iter()).copied().collect();
    // JavaScript의 spread operator([...])와 Rust의 chain()은 동일한 개념입니다.
    let more_numbers = vec![11, 12, 13];
    let combined: Vec<i32> = numbers.iter().chain(more_numbers.iter()).copied().collect();
    println!("   chain: {:?}", combined);

    // JavaScript의 Generator (function*):
    //
    // JavaScript:
    //   function* range(start, end) {
    //     for (let i = start; i < end; i++) yield i;
    //   }
    //   for (const n of range(1, 5)) { console.log(n); } // 1, 2, 3, 4
    // Rust의 Iterator는 이와 유사하게 Lazy하게 동작합니다.
    // JavaScript:
    //   const lazy = arr.map(x => { console.log('processing x'); return x * 2; });
    //   lazy.forEach(x => console.log(x)); // map의 console.log가 즉시 실행됨
    // Rust:
    //   let lazy = arr.iter().map(|x| { println!("processing {}", x); x * 2 });
    //   // println이 즉시 실행되지 않음! Lazy evaluation!
    //   let result: Vec<i32> = lazy.collect();
    //   println!("{:?}", result);
    // JavaScript의 Generator와 달리 Rust는 컴파일타임에 최적화됩니다.
    // JavaScript의 map은 즉시 실행 (eager)이지만,
//   Rust의 map은 collect()까지 실행되지 않음 (lazy).

    // Iterator의 Chaining (파이프라인 패턴)
    //
    // JavaScript:
    //   const result = arr.filter(x => x > 2)
    //                    .map(x => x * 2)
    //                    .slice(0, 3);
    //   // 3번의 배열 생성 + 3번의 순회
    // Rust:
    //   let result: Vec<i32> = arr.iter()
    //       .filter(|x| *x > 2)
    //       .map(|x| *x * 2)
    //       .take(3)
    //       .collect();
    //   // 1번의 순회 (최적화!)
    let chained: Vec<i32> = numbers
        .iter()
        .filter(|x| **x > 2) // JavaScript: filter(x => x > 2)
        .map(|x| *x * 2) // JavaScript: map(x => x * 2)
        .take(3) // JavaScript: slice(0, 3)
        .collect();
   println!("   chain pipeline: {:?}", chained);
    // JavaScript: arr.filter(x => x > 2).map(x => x * 2).slice(0, 3);
    // Rust: arr.iter().filter(|x| *x > 2).map(|x| *x * 2).take(3).collect();
    // JavaScript: 3번의 배열 생성 + 3번의 순회
    // Rust: 1번의 순회 (Lazy evaluation + 컴파일타임 최적화)

    // Iterator는 JavaScript의 Array method보다 더 많은 옵션을 제공합니다:
    // JavaScript: map, filter, reduce, find, some, every, forEach, includes, indexOf, etc.
    // Rust: map, filter, fold, find, any, all, for_each, contains, position, etc. + take, skip, step_by, zip, etc.
}

// ============================================================
// Section 21: Concurrency (동시성)
// ============================================================
//
// Rust의 동시성은 JavaScript의 Worker Threads와 Web Workers에 해당합니다.
//
// JavaScript vs Rust 동시성 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript               │ Rust                                    │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ new Worker('worker.js')  │ std::thread::spawn(|| { ... })          │
// │ worker.postMessage(data) │ tx.send(data).unwrap()                 │
// │ worker.onmessage = ...   │ rx.recv().unwrap()                     │
// │ Promise.all([p1, p2])    │ h1.join() + h2.join()                 │
// │ SharedArrayBuffer        │ Arc<Mutex<T>>                          │
// │ Atomics.add(view, 0, 1)  │ *mutex.lock().unwrap() += 1           │
// │ 데이터 경합 가능성 있음   │ 컴파일타임에 데이터 경합 방지          │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// 데이터 경합 (Data Race):
//   JavaScript: 여러 Worker가 같은 SharedArrayBuffer를 수정하면 경합 발생 가능
//   Rust: 컴파일러가 데이터 경합을 컴파일타임에 방지합니다.
//   이것은 Rust의 가장 강력한 동시성 장점입니다.

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
    //
    // JavaScript의 Worker는 별도의 파일에서 실행되지만,
    // Rust의 thread는 같은 코드에서 spawn됩니다.

    // JavaScript의 Promise.all() = Rust의 thread::spawn + join()
    //
    // JavaScript:
    //   const p1 = fetch('/api/users');
    //   const p2 = fetch('/api/posts');
    //   const results = await Promise.all([p1, p2]);
    // Rust:
    //   let handle1 = std::thread::spawn(|| { work1(); });
    //   let handle2 = std::thread::spawn(|| { work2(); });
    //   let r1 = handle1.join().unwrap();
    //   let r2 = handle2.join().unwrap();
    //
    // JavaScript의 Promise.all()은 모든 Promise가 완료될 때까지 대기합니다.
    // Rust의 join()도 모든 thread가 완료될 때까지 대기합니다.

    // JavaScript: `const worker = new Worker(() => { ... });`
    // Rust: `let handle = std::thread::spawn(|| { ... });`
    // JavaScript의 Worker 스코프 = Rust의 `move` closure (변수를 소유권으로 가져옴)
    //
    // JavaScript:
    //   const data = { value: 42 };
    //   const worker = new Worker(() => {
    //     self.postMessage(data); // Worker는 데이터의 복사본을 받음
    //   });
    // Rust:
    //   let data = 42;
    //   let handle = std::thread::spawn(move || {
    //     println!("{}", data); // data를 thread로 이동 (소유권 이전)
    //   });
    // move는 변수의 소유권을 thread로 이동합니다.
    // JavaScript의 Worker는 자동으로 데이터를 직렬화하여 전달하지만,
    // Rust의 thread는 move로 명시적으로 소유권을 이동합니다.

    // 간단한 스레드 예시
    // JavaScript의 setTimeout과 유사한 개념 (하지만 Rust의 thread는 병렬 실행)
    //
    // JavaScript:
    //   setTimeout(() => { console.log('from worker'); }, 100);
    // Rust:
    let handle = std::thread::spawn(|| {
        // JavaScript: `setTimeout(() => { console.log('from worker'); }, 100);`
        println!("   Hello from spawned thread!");
        42 // 반환값 (JavaScript: `self.postMessage(42)`)
        // JavaScript: self.postMessage(42);
        // Rust: 42 (마지막 표현식이 반환값)
    });

    // JavaScript의 `worker.onmessage = (e) => console.log(e.data);`
    // Rust의 `handle.join().unwrap();`로 스레드의 결과를 받습니다
    //
    // JavaScript:
    //   worker.onmessage = (e) => console.log(e.data); // 42
    // Rust:
    let result = handle.join().unwrap();
    // join() → thread가 완료될 때까지 대기
    // unwrap() → Result에서 값 추출 (Err이면 패닉)
    println!("   Thread returned: {}", result);
    // JavaScript: console.log(42);

    // 여러 스레드 병렬 실행 (JavaScript의 Promise.all() 유사)
    //
    // JavaScript:
    //   const tasks = [task1, task2, task3];
    //   const results = await Promise.all(tasks.map(t => t()));
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
    // (1..=3) → 1, 2, 3 범위
    // .map(...) → 각 i에 대해 thread를 spawn
    // .collect() → Vec<JoinHandle<i32>>로 모음
    // JavaScript: const handles = [p1, p2, p3];
    //             await Promise.all(handles.map(h => h()));

    // JavaScript: `for (const r of results) { console.log(r); }`
    // Rust: `for handle in handles { result.push(handle.join().unwrap()); }`
    for handle in handles {
        let r = handle.join().unwrap();
        println!("   Task result: {}", r);
        // JavaScript: console.log(r);
    }

    // JavaScript의 MessageChannel과 유사한 것이 Rust의 Channel (mpsc = Multi-Producer, Single-Consumer)
    //
    // JavaScript:
    //   const { port1, port2 } = new MessageChannel();
    //   port1.postMessage('Hello');
    //   port2.onmessage = (e) => console.log(e.data);
    // Rust:
    //   let (tx, rx) = std::sync::mpsc::channel();
    //   tx.send('Hello');
    //   rx.recv().unwrap();
    //
    // mpsc = Multi-Producer, Single-Consumer
    // 여러 송신자(tx)가 하나의 수신자(rx)로 메시지를 보낼 수 있습니다.

    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    // tx: Sender (송신자)
    // rx: Receiver (수신자)
    // JavaScript: const { port1, port2 } = new MessageChannel();

    // JavaScript: `worker.postMessage({ id: 1, data: [1,2,3] });`
    // Rust: `tx.send((1, vec![1, 2, 3])).unwrap();`
    //
    // JavaScript:
    //   worker.postMessage({ id: 1, data: [1, 2, 3] });
    // Rust:
    std::thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap(); // JavaScript: `postMessage(i)`
            // JavaScript: self.postMessage(i);
            // tx.send() → 메시지를 채널로 전송
            // unwrap() → Err이면 패닉 (전송 실패 시)
        }
    });
    // JavaScript: setTimeout(() => worker.postMessage(i), i * 100);
    // Rust: thread에서 tx.send(i) → 채널로 메시지 전송

    // JavaScript: `worker.onmessage = (e) => console.log(e.data);`
    // Rust: `rx.recv()`으로 메시지 받기
    //
    // JavaScript:
    //   worker.onmessage = (e) => console.log(e.data);
    // Rust:
    println!("   Messages from channel:");
    for received in rx.iter() {
        // rx.iter() → Receiver를 Iterator로 사용
        // JavaScript: for (const msg of messages) { console.log(msg); }
        println!("     Received: {}", received);
    }
    // rx.recv() → 단일 메시지 수신
    // rx.iter() → 모든 메시지 수신 (Receiver가 고갈될까지)

    // JavaScript의 `SharedArrayBuffer`와 `Atomics` = Rust의 `Arc<Mutex<T>>`
    //
    // JavaScript:
    //   const buffer = new SharedArrayBuffer(100);
    //   const view = new Int32Array(buffer);
    //   Atomics.add(view, 0, 1);
    // Rust:
    //   let counter = Arc::new(Mutex::new(0));
    //   let counter_clone = Arc::clone(&counter);
    //   std::thread::spawn(move || {
    //       let mut num = counter_clone.lock().unwrap();
    //       *num += 1;
    //   });
    // Arc = Atomic Reference Counted (여러 스레드에서 공유)
    //   JavaScript의 WeakRef와 유사하지만, Arc는 참조 카운트를 "원자적"으로 증가/감소
    //   스레드가 종료될 때 참조 카운트가 0이 되면 자동으로 메모리 해제
    // Mutex = Mutual Exclusion (한 번에 하나의 스레드만 접근)
    //   JavaScript의 lock 메커니즘 (예: SharedArrayBuffer.Atomics)와 유사하지만,
    //   Rust의 Mutex는 "lock을 놓칠 때" 컴파일 에러를 발생시킴
    //
    // Arc<Mutex<T>> vs JavaScript SharedArrayBuffer:
    //   JavaScript: SharedArrayBuffer는 raw memory이고, 개발자가 직접 lock 관리
    //     const buffer = new SharedArrayBuffer(100);
    //     const view = new Int32Array(buffer);
    //     Atomics.add(view, 0, 1); // lock을 직접 관리해야 함 (실수 가능!)
    //   Rust: Arc<Mutex<T>>는 "타입 안전한 공유 mutable 참조"
    //     let counter = Arc::new(Mutex::new(0));
    //     *counter.lock().unwrap() += 1; // lock을 놓치면 컴파일 에러!
    //   이 차이가 Rust가 "컴파일타임에 데이터 경합을 방지"하는 이유입니다.
    println!("   Rust guarantees no data races at compile time!");
    // JavaScript: 런타임에 데이터 경합이 발생할 수 있음
    // Rust: 컴파일타임에 데이터 경합 방지 (이것이 Rust의 가장 강력한 장점)
    //
    // 데이터 경합이란:
    //   1. 여러 스레드가 같은 데이터에 동시 접근
    //   2. 적어도 한 스레드가 쓰기 연산 수행
    //   3. 동기화 메커니즘이 없음
    //   JavaScript: 이런 경합이 "런타임"에 발생하여 예측 불가능한 결과 초래
    //   Rust: 컴파일러가 "move + 소유권" 규칙으로 경합 자체를 방지
    //     - thread::spawn(move || ...)로 소유권 이동 → 원본 스레드에서 접근 불가
    //     - Arc<Mutex<T>>로 공유 → Mutex lock 없이 접근 시도 시 컴파일 에러
    //
    // JavaScript:
    //   let shared = 0;
    //   setTimeout(() => { shared++; }, 100);
    //   setTimeout(() => { console.log(shared); }, 200);
    //   // 경합 가능성 있음
    // Rust:
    //   let shared = Arc::new(Mutex::new(0));
    //   // 컴파일러가 데이터 경합을 방지합니다!

    // JavaScript의 Promise와 Rust의 Thread 비교:
    //
    // JavaScript:
    //   const promise = new Promise(resolve => {
    //     setTimeout(() => resolve(42), 1000);
    //   });
    //   const result = await promise;
    // Rust:
    //   let handle = std::thread::spawn(|| {
    //     std::thread::sleep(Duration::from_secs(1));
    //     42;
    //   });
    //   let result = handle.join().unwrap();
    // Promise.all() → join()
    // await → .join().unwrap()
}

// ============================================================
// Section 22: Cargo - Rust Package Manager (캐르고 - 패키지 매니저)
// ============================================================
//
// Cargo는 Rust의 내장 패키지 매니저로, npm/yarn/pnpm과 유사합니다.
//
// JavaScript vs Rust 도구 비교:
// ┌──────────────────────────┬─────────────────────────────────────────┐
// │ JavaScript (npm/yarn)    │ Rust (Cargo)                           │
// ├──────────────────────────┼─────────────────────────────────────────┤
// │ package.json             │ Cargo.toml                             │
// │ node_modules/            │ ~/.cargo/registry/                     │
// │ npm install              │ cargo build                            │
// │ npm install express      │ cargo add express                      │
// │ npm start                │ cargo run                              │
// │ npm test                 │ cargo test                             │
// │ npm run build            │ cargo build                            │
// │ npm run lint             │ cargo clippy                           │
// │ npm run format           │ cargo fmt                              │
// │ npm outdated             │ cargo outdated                         │
// │ npm update               │ cargo update                           │
// │ npm publish              │ cargo publish                          │
// │ npm ls                   │ cargo tree                             │
// │ https://npmjs.com        │ https://crates.io                      │
// └──────────────────────────┴─────────────────────────────────────────┘
//
// Cargo.toml vs package.json:
//   JavaScript: JSON 형식, 동적 타입
//   Rust: TOML 형식, 정적 타입
//   둘 다 프로젝트 설정과 의존성을 관리하지만,
//   Rust의 Cargo는 빌드 시스템까지 포함합니다.

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
    //
    // 주요 차이점:
    //   - Cargo.toml은 TOML 형식 (JSON보다 읽기 쉬움)
    //   - Cargo.toml에는 scripts가 없음 (cargo 서브커맨드로 대체)
    //   - Cargo.toml에는 edition (Rust 버전)이 필요

    // JavaScript의 npm 명령어와 Rust의 Cargo 명령어 비교:
    //
    // npm/yarn/pnpm              Cargo                  설명
    // ────────────────────────── ────────────────────── ─────────────────────────────
    // npm init                    cargo init             새 프로젝트 시작
    // npm init my-app/            cargo new my-app       새 프로젝트 생성 (폴더 포함)
    // npm install                 cargo build            의존성 설치 및 빌드
    // npm install express         cargo add express      패키지 설치
    // npm start                   cargo run              프로젝트 실행
    // npm test                    cargo test             테스트 실행
    // npm run build               cargo build            빌드 (실제 실행 파일 생성)
    // npm run lint                cargo clippy           코드 품질 검사
    // npm run format              cargo fmt              코드 포맷팅
    // npm outdated                cargo outdated         오래된 의존성 확인
    // npm update                  cargo update               의존성 업데이트
    // npm pack                    cargo package          패키지 압축 (crates.io 업로드)
    // npm publish                 cargo publish          crates.io에 공개
    // npm ls                      cargo tree             의존성 트리 표시
    // npx                         cargo-expand           패키지 실행
    //
    // npm install vs cargo build:
    //   JavaScript: npm install → node_modules/에 패키지 설치
    //   Rust: cargo build → ~/.cargo/registry/에서 패키지 다운로드 + 빌드
    //   Rust는 패키지를 컴파일해야 하므로 빌드가 느립니다.

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
    //
    // 버전 범위:
    //   "1.0"       → 1.0.x (마이너 버전 자동 업데이트)
    //   "^1.0.0"    → 1.0.0 ~ 1.999.999 (메이너 버전 자동 업데이트)
    //   "~1.0.0"    → 1.0.0 ~ 1.0.999 (패치 버전 자동 업데이트)
    //   "1.0.0"     → 정확히 1.0.0
    //   JavaScript: "^1.0.0" → npm의 기본 버전 범위

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
    //
    // JavaScript의 monorepo와 Rust의 workspace는 동일한 개념입니다.
    // 여러 패키지를 하나의 저장소에서 관리합니다.

    // crates.io (JavaScript의 npm registry)
    //
    // JavaScript: https://www.npmjs.com/
    // Rust: https://crates.io/
    //
    // JavaScript: `npm search express`
    // Rust: `cargo search express` (하지만 Rust에는 express가 없음, Rust의 web framework는 axum, actix-web)
    //
    // npm과 crates.io의 차이:
    //   npm: 수백만 패키지 (JavaScript 생태계 크기)
    //   crates.io: 수만 crate (Rust 생태계 크기)
    //   Rust는 생태계가 작지만, 품질이 높습니다.

    // Cargo의 빌드 프로세스 (JavaScript의 build pipeline)
    //
    // JavaScript:
    //   1. npm install (의존성 설치)
    //   2. npm run build (빌드)
    //   3. node dist/main.js (실행)
    // Rust:
    //   1. cargo build (빌드 + 의존성 설치)
    //   2. cargo run (빌드 + 실행)
    //   3. cargo check (빠른 체크, 컴파일만)
    //
    // JavaScript의 TypeScript: tsc (컴파일) → node (실행)
    // Rust: cargo build (컴파일 + 링크) → 실행 파일 직접 실행
    // Rust는 컴파일러가 native 코드를 생성하므로 별도의 런타임이 필요 없습니다.
    // JavaScript는 Node.js 런타임이 필요하지만,
    // Rust는 컴파일된 실행 파일만 있으면 됩니다.

    // Cargo의 Features (JavaScript의 npm optional dependencies / peer dependencies)
    //
    // JavaScript: "react": {"optional": true}
    // Rust: serde = { version = "1.0", features = ["derive"] }
    //
    // Feature는 옵션 기능입니다.
    // JavaScript의 optional dependencies와 유사하지만,
    // Rust의 feature는 빌드 시 선택합니다.
    //   serde = { version = "1.0", features = ["derive", "rc"] }
    //   → derive와 rc feature 활성화
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
    //
    // clippy vs ESLint:
    //   ESLint: 런타임에 동작, 코드 스타일 검사
    //   clippy: 컴파일타임에 동작, 코드 품질 + 스타일 검사
    //   clippy는 더 깊이 있는 분석을 제공합니다.

    // JavaScript의 Node.js 실행 = Rust의 컴파일된 실행 파일
    //
    // JavaScript:
    //   node app.js              → JavaScript 코드를 Node.js가 실행
    //   node --inspect app.js    → 디버거 연결
    // Rust:
    //   ./target/debug/my-app    → 네이티브 바이너리 직접 실행
    //   ./target/release/my-app  → 최적화된 네이티브 바이너리
    //
    // JavaScript: npm run dev (dev server)
    // Rust: cargo run (개발 서버)
    // Rust: cargo build --release (production 빌드)
    //
    // 성능 비교:
    //   JavaScript: Node.js 런타임 오버헤드가 있음
    //   Rust: 네이티브 바이너리 → 런타임 오버헤드 없음
    //   Rust는 JavaScript보다 일반적으로 10-100배 빠릅니다.
}

// ============================================================
// 메인 함수 - 모든 섹션 실행
// ============================================================
//
// 이 main 함수는 22개의 섹션을 순차적으로 실행합니다.
//
// #[tokio::main] 어노테이션:
//   Rust의 async 함수는 tokio 런타임 위에서 실행됩니다.
//   #[tokio::main]은 tokio의 multi-thread 런타임을 시작합니다.
//   JavaScript: Node.js가 이벤트 루프를 자동으로 시작
//   Rust: #[tokio::main]이 tokio 런타임을 시작
//
// Result<(), Box<dyn std::error::Error>>:
//   JavaScript: function main() { ... } (에러 처리 없음)
//   Rust: main 함수도 Result를 반환할 수 있습니다.
//   Ok(()) → 성공
//   Err(e) → 실패 (에러 메시지 출력 후 종료)

#[tokio::main]
// #[tokio::main] → tokio 런타임을 main 함수에 연결
// JavaScript: Node.js가 자동으로 이벤트 루프 시작
// Rust: #[tokio::main]이 tokio 런타임을 시작해야 async 함수 사용 가능
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Result<(), Box<dyn std::error::Error>>:
    //   () → 반환값 없음 (JavaScript: void)
    //   Box<dyn std::error::Error> → 모든 에러 타입을 담을 수 있는 박스
    //   JavaScript: process.exit(0) 또는 throw new Error()
    println!("Rust Tutorial - JavaScript 개발자를 위한 Rust 문법 가이드");
    println!("============================================================\n");
    // JavaScript: console.log("Rust Tutorial - JavaScript 개발자를 위한 Rust 문법 가이드");

    // 섹션 1~15는 동기 함수이므로 직접 호출
    //
    // JavaScript:
    //   function runAll() {
    //     section1();
    //     section2();
    //     ...
    //   }
    // Rust:
    section_1_variable_declarations();
    // Section 1: 변수 선언 (let/mut, 타입 추론)
    // JavaScript: const/let, dynamic typing
    section_2_primitive_types();
    // Section 2: 기본 타입 (i32, f64, bool, char)
    // JavaScript: number, string, boolean
    section_3_strings();
    // Section 3: 문자열 (String, &str)
    // JavaScript: string (불변)
    section_4_functions();
    // Section 4: 함수 (fn, 클로저, 반환 타입)
    // JavaScript: function, arrow function
    section_5_ownership();
    // Section 5: 소유권 (Move, Clone, Copy)
    // JavaScript: GC (가비지 컬렉션)
    section_6_references_and_borrowing();
    // Section 6: 참조 & 빌림 (&T, &mut T)
    // JavaScript: 객체 참조
    section_7_structs();
    // Section 7: 구조체 (struct, 필드 접근)
    // JavaScript: Object
    section_8_enums_and_match();
    // Section 8: Enum & Match (열거형, 패턴 매칭)
    // JavaScript: switch, null/undefined
    section_9_pattern_matching();
    // Section 9: 패턴 매칭 (range, destructuring)
    // JavaScript: switch, destructuring
    section_10_collections();
    // Section 10: 컬렉션 (Vec, HashMap)
    // JavaScript: Array, Map
    section_11_closures();
    // Section 11: 클로저 (|x| x * 2)
    // JavaScript: (x) => x * 2
    section_12_traits();
    // Section 12: 트레이트 (trait, impl)
    // JavaScript: interface, class
    section_13_error_handling();
    // Section 13: 에러 처리 (Result, Option, ?)
    // JavaScript: try/catch
    section_14_generics();
    // Section 14: 제네릭 (fn identity<T>(value: T) -> T)
    // JavaScript: 타입 없음 (TypeScript: 제네릭)
    section_15_lifetimes();
    // Section 15: 라이프타임 ('a)
    // JavaScript: GC (라이프타임 없음)

    // 섹션 16은 async 함수이므로 await 필요
    //
    // JavaScript:
    //   async function main() {
    //     await asyncSection();
    //   }
    // Rust:
    //   async fn main() {
    //     async_section().await;
    //   }
    section_16_async_example();
    // async fn → .await로 실행
    // JavaScript: async function → await 사용

    // 섹션 17~22는 동기 함수이므로 직접 호출
    //
    // JavaScript:
    //   modulesSection();
    //   fileIOSection();
    //   testingSection();
    //   iteratorSection();
    //   concurrencySection();
    //   cargoSection();
    // Rust:
    section_17_modules_and_crates();
    // Section 17: 모듈 & 크레이트 (mod, pub, use)
    // JavaScript: import/export, npm
    section_18_file_io();
    // Section 18: 파일 입출력 (std::fs)
    // JavaScript: fs 모듈 (Node.js)
    section_19_testing();
    // Section 19: 테스트 (#[test], assert_eq!)
    // JavaScript: Jest, Mocha
    section_20_iterators();
    // Section 20: 이터레이터 (iter, map, filter, fold)
    // JavaScript: Array.map, Array.filter
    section_21_concurrency();
    // Section 21: 동시성 (thread, channel, Arc<Mutex<T>>)
    // JavaScript: Worker Threads, SharedArrayBuffer
    section_22_cargo();
    // Section 22: Cargo (패키지 매니저)
    // JavaScript: npm/yarn/pnpm

    println!("\n============================================================");
    println!("All 22 sections complete! Rust tutorial finished.");
    println!("Hope this helps JavaScript developers understand Rust's core concepts!");
    // JavaScript:
    //   console.log("\n============================================================");
    //   console.log("All 22 sections complete! Rust tutorial finished.");
    //   console.log("Hope this helps JavaScript developers understand Rust's core concepts!");

    Ok(())
    // JavaScript: return; (또는 return undefined;)
    // Rust: Ok(()) → 성공을 나타내는 Result 값
}
