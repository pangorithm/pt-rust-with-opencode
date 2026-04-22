// # Rust Tutorial - JavaScript Developers' Rust Syntax Guide
//
// This project helps JavaScript (ES6+) developers quickly learn Rust's core syntax and concepts.
// Each section is a standalone function called sequentially from main().
//
// Most important differences for JavaScript developers:
// - Rust determines types and memory management at **COMPILE TIME** (JavaScript uses runtime)
// - Rust guarantees memory safety through its **Ownership system** without Garbage Collection
// - Rust handles **all values with defined types**, but has strong Type Inference

// ============================================================
// Section 1: Variable Declarations
// ============================================================

// Rust's `let`/`mut` corresponds to JavaScript's `let`/`const`
// - JavaScript: `const` cannot be reassigned, `let` can
// - Rust: `let` is immutable by default, use `mut` for mutable
// - Rust's Type Inference is powerful - types are inferred automatically when omitted

fn section_1_variable_declarations() {
    // Similar to JavaScript's `const`: cannot be reassigned
    let immutable_value: i32 = 42;
    // immutable_value = 100; // ERROR! Cannot reassign

    // JavaScript's `let`-like reassignment with `mut`
    let mut mutable_value: i32 = 42;
    mutable_value = 100; // OK! Can reassign
    println!("Section 1 - Variable Declarations: mutable = {}", mutable_value);

    // Type Inference - Rust automatically infers types when not specified
    let inferred_number = 42;        // Automatically i32
    let inferred_float = 3.14_f64;   // Explicitly f64
    let inferred_string = "hello";   // Automatically &str (string slice)
    println!("   Type Inference: {} (i32), {} (f64), {} (&str)", inferred_number, inferred_float, inferred_string);

    // If you always want immutable like JavaScript's `const`, don't add `mut`
    // This is Rust's philosophy: immutable by default, mutable only when explicitly declared
}

// ============================================================
// Section 2: Primitive Types
// ============================================================

// JavaScript has `number` (all numbers are float64), `string`, `boolean`, `null`, `undefined`, etc.
// Rust provides a much more clear type system:
// - Integers: i8, i16, i32, i64, i128, isize (signed)
// - Unsigned integers: u8, u16, u32, u64, u128, usize
// - Floating point: f32, f64
// - Boolean: bool
// - Character: char (Unicode scalar value, 1 character)

fn section_2_primitive_types() {
    // Integer types - unlike JavaScript's `number`, you specify the size
    let signed: i32 = -100;           // -2^31 to 2^31-1 (similar range to JavaScript's number)
    let unsigned: u32 = 100;          // 0 to 2^32-1 (cannot be negative)
    let big: i128 = 999_999_999_999;  // underscore for readability (also possible in JavaScript)
    println!("Section 2 - Integers: {} (i32), {} (u32), {} (i128)", signed, unsigned, big);

    // Floating point - JavaScript's `number` is all float64, but Rust lets you choose
    let float32: f32 = 3.14;          // 32-bit float
    let float64: f64 = 3.141592653589793; // 64-bit (same as JavaScript's number)
    println!("   Float: {} (f32), {} (f64)", float32, float64);

    // Boolean - same as JavaScript's `true`/`false`
    let is_rust_great: bool = true;  // Same as JavaScript's `true`
    let is_javascript_cool: bool = false;
    println!("   Boolean: {} (Is Rust great? {})", is_javascript_cool, is_rust_great);

    // Character - JavaScript's character is String, but Rust has a separate char type
    // JavaScript: `'a'` and `"a"` are both String (no difference)
    // Rust: `'a'` is char (Unicode scalar, 4 bytes)
    let first_letter: char = 'R';
    let emoji: char = '🦀';            // Rust's crab mascot!
    println!("   Char: '{}' (char), '{}' (emoji)", first_letter, emoji);
}

// ============================================================
// Section 3: Strings
// ============================================================

// JavaScript's String and Rust's String are fundamentally different:
// - JavaScript: String is immutable, method calls always return new String
// - Rust: `String` is mutable (Heap allocated), `&str` is immutable (Immutable slice)
// - Rust clearly distinguishes two types for memory efficiency

fn section_3_strings() {
    // &str: Immutable string slice known at compile time (string reference)
    // Closest concept to JavaScript's String, but the "slice" distinction matters
    let string_slice: &str = "Hello, Rust!";
    println!("Section 3 - &str: {}", string_slice);

    // String: Mutable String on Heap
    // JavaScript's String is immutable, but Rust's String is mutable
    let mut heap_string = String::from("Hello, ");
    heap_string.push_str("World!");   // Add string with push_str (corresponds to JavaScript's `+=`)
    println!("   String: {}", heap_string);

    // JavaScript's `+` for string concatenation is inefficient (creates new object)
    // Rust's `format!` macro is similar to JavaScript's template literals
    let name = "Rust";
    let combined = format!("Hello, {}!", name);  // Similar to JavaScript's ``Hello, ${name}!``
    println!("   format!: {}", combined);

    // String concatenation (corresponds to JavaScript's `+`)
    let mut string_concat = String::from("Hello");
    string_concat.push('!');  // OK! Push character
    println!("   push!: {}", string_concat);
}

// ============================================================
// Section 4: Functions
// ============================================================

// Key differences between JavaScript functions and Rust functions:
// - JavaScript: All values are implicitly returned (explicit return needed)
// - Rust: Last expression is implicit return, `return` is used explicitly
// - JavaScript: `=>` (arrow function) for anonymous functions
// - Rust: `|args| { ... }` for closures (covered in separate section)

fn section_4_functions() {
    // Corresponds to JavaScript's `function add(a, b)`
    // Unlike JavaScript, you must specify parameter and return types
    fn add(a: i32, b: i32) -> i32 {
        a + b  // ← Last expression is implicit return value (same as JavaScript's `return a + b`)
    }
    println!("   add(2, 3) = {}", add(2, 3));

    // You can also use explicit `return` like JavaScript
    fn greet(name: &str) -> String {
        return format!("Hello, {}!", name);  // Explicit return
    }
    println!("   greet: {}", greet("JavaScript developer"));

    // Corresponds to JavaScript's arrow function `const double = (x) => x * 2;`
    // Can also define functions in Rust in the same form (inferred when return type omitted)
    let double_fn = |x: i32| -> i32 { x * 2 };  // This is a closure (detailed in separate section)
    println!("   Closure double(21) = {}", double_fn(21));

    // Corresponds to JavaScript's `void` function - return type is `()` (Unit type)
    fn log_message(message: &str) {  // -> () can be omitted (Unit type)
        println!("   [LOG] {}", message);
    }
    log_message("Function section example");

    // Corresponds to JavaScript's `default parameters` - Rust has no default values, but
    // overloading is not possible, so handle with different function names or Option
    fn optional_greet(name: Option<&str>) -> String {
        match name {
            Some(n) => format!("Hello, {}!", n),
            None => "Hello, World!".to_string(),
        }
    }
    println!("   Optional: {}", optional_greet(Some("Alice")));
    println!("   Optional: {}", optional_greet(None));  // Similar to JavaScript's undefined
}

// ============================================================
// Section 5: Ownership (소유권)
// ============================================================

// **The most important concept in Rust** - The part JavaScript developers struggle with most
//
// JavaScript automatically manages memory with Garbage Collection (GC).
// Rust has no GC, but guarantees memory safety with its Ownership system.
//
// Three rules:
// 1. Each value has one **Owner**
// 2. When the owner goes out of scope, the value is **dropped** (deleted)
// 3. A value that has been moved cannot be used

fn section_5_ownership() {
    // String is a type that stores data on Heap
    // JavaScript: All String are stored on Heap
    let s1 = String::from("hello");  // s1 is the Owner of "hello"
    let s2 = s1;                     // s1's value is **moved** to s2
    // println!("{}", s1);            // ERROR! s1 is no longer valid (moved)
    println!("Section 5 - Ownership: s2 = {}", s2);  // OK! s2 is Owner
    // For JavaScript developers: The thing JS's GC does for you, you manage directly in Rust

    // Clone for explicit copying
    let s3 = String::from("world");
    let s4 = s3.clone();             // OK! Explicit copy - Heap memory also copied
    println!("   Clone: s3 = {}, s4 = {}", s3, s4);  // Both can be used
    // For JavaScript developers: Similar to JavaScript's `JSON.parse(JSON.stringify(obj))`, but
    // in Rust you must explicitly call clone() (for performance consideration)

    // Basic types like i32 are stored on Stack, so Move is not Copy occurs
    let x = 42;
    let y = x;                       // OK! x is copied (Copy trait implemented type)
    println!("   Copy: x = {}, y = {}", x, y);  // Both can be used
    // For JavaScript developers: JavaScript's number are all primitive, so
    // this is most similar to JavaScript's behavior
}

// ============================================================
// Section 6: References & Borrowing (참조 & 빌림)
// ============================================================

// As an alternative to Ownership, you can use values without owning them
// - `&T`: Immutable reference (immutable borrow) - multiple allowed
// - `&mut T`: Mutable reference (mutable borrow) - only one allowed (at a time)
// Similar to JavaScript's references, but rules are enforced at compile time

fn section_6_references_and_borrowing() {
    // Immutable reference - pass reference only without copying value
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1: Immutable reference (borrowing)
    println!("Section 6 - Borrowing: Length is {}, s1 still valid: {}", len, s1);
    // For JavaScript developers: Similar to JavaScript's object reference, but
    // Rust guarantees at compile time "this reference won't change the value"

    // Mutable reference - reference that can change the value
    let mut s2 = String::from("hello");
    change_string(&mut s2);            // &mut s2: Mutable reference (mutable borrowing)
    println!("   Mutable borrow: {}", s2);
    // For JavaScript developers: JavaScript's objects are mutable by default.
    // In Rust you must explicitly request `&mut` to change

    // ✅ Multiple immutable references allowed at the same time
    let _r1 = &s1;
    let _r2 = &s1;
    let _r3 = &s1;  // Infinitely many immutable references allowed

    // ❌ Cannot have mutable and immutable references at the same time
    // let _r4 = &mut s2;  // ERROR! Cannot have mutable reference while immutable reference exists
}

fn calculate_length(s: &String) -> usize {
    s.len()  // Corresponds to JavaScript's `string.length`
}

fn change_string(s: &mut String) {
    s.push_str(", world!");  // Corresponds to JavaScript's `string += ", world!"`
}

// ============================================================
// Section 7: Structs (구조체)
// ============================================================

// JavaScript Object and Rust Struct comparison
// - JavaScript: `{ name: "Alice", age: 30 }` - dynamic, flexible
// - Rust: `struct` - static, type determined at compile time
// Similar to Java's Class, but holds only data without methods

fn section_7_structs() {
    // JavaScript's `class User { constructor(name, age) { this.name = name; this.age = age; } }`
    // Rust's struct is very similar to Java's class
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Instance creation - corresponds to JavaScript's `new User("alice", ...)`
    let user1 = User {
        username: String::from("alice_dev"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("Section 7 - Struct: {} ({})", user1.username, user1.email);

    // Field access - corresponds to JavaScript's `user1.username`
    println!("   Active: {}", user1.active);

    // JavaScript's objects are mutable, but Rust struct instances are immutable by default
    // You need to add `mut` to change
    let mut user2 = User {
        username: String::from("bob_dev"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("bob_new@example.com");  // OK! Can change with mutable
    println!("   After change: {}", user2.email);

    // Struct update syntax (similar to spread operator)
    let user3 = User {
        ..user2  // Copy the rest of user2's fields
    };
    // Similar to JavaScript's `{...user2, email: "new"}`
    println!("   Spread: {}", user3.email);
}

// ============================================================
// Section 8: Enums & Match (열거형 & 패턴 매칭)
// ============================================================

// JavaScript has no Enum (uses Symbol or string constants).
// Rust's Enum is similar to TypeScript's Union Type, but much more powerful.
// Java's enum is just a set of constants, but Rust's Enum can hold data in each variant.

fn section_8_enums_and_match() {
    // JavaScript's `const Direction = { UP: 'up', DOWN: 'down', LEFT: 'left', RIGHT: 'right' };`
    // But Rust Enum can hold data!
    enum Message {
        Quit,                              // Variant without data
        Move { x: i32, y: i32 },           // Object data (JavaScript's `{ x: 1, y: 2 }`)
        Write(String),                      // String data
        ChangeColor(i32, i32, i32),         // Tuple data (similar to Java's Tuple)
    }

    // JavaScript uses `switch (msg)` for processing
    // Rust uses `match` for pattern matching - must be exhaustive (handle all cases)
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("   Quit message"),
        Message::Move { x, y } => println!("   Move: ({}, {})", x, y),
        Message::Write(text) => println!("   Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("   ChangeColor: RGB({}, {}, {})", r, g, b),
    }
    // Similar to JavaScript's switch, but Rust must handle all cases (exhaustive match)

    // Option - Enum for handling JavaScript's `null`/`undefined`
    // Very similar to Java's Optional
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    println!("Section 8 - Option: {:?}, {:?}", some_number, no_number);

    // JavaScript's null handling: `x !== null ? x : x.toString()`
    // Rust: Safe handling with match
    match some_number {
        Some(n) => println!("   Some: {}", n),
        None => println!("   None (corresponds to null/undefined)"),
    }
}

// ============================================================
// Section 9: Pattern Matching (패턴 매칭)
// ============================================================

// Much more powerful than JavaScript's `switch`.
// - JavaScript: `switch(value)` - only simple value matching
// - Rust: Pattern matching - struct destructuring, ranges, conditional matching

fn section_9_pattern_matching() {
    // Corresponds to JavaScript's switch
    let number = 42;
    match number {
        0 => println!("   0"),
        1 | 2 | 3 => println!("   1, 2, or 3"),  // Match multiple values with |
        10..=20 => println!("   10~20 range"),   // Range matching (impossible in JavaScript)
        21..=100 => println!("   21~100 range"),
        _ => println!("   Other number"),          // _ = default case (JavaScript's switch default)
    }

    // Destructuring - similar to JavaScript's destructuring assignment
    let point = (3, 5);  // JavaScript: const point = [3, 5];
    let (x, y) = point;  // JavaScript: const [x, y] = point;
    println!("   Destructuring: ({}, {})", x, y);

    // Object destructuring - similar to JavaScript's `const { name, age } = user;`
    struct Point2D {
        x: i32,
        y: i32,
    }
    let p = Point2D { x: 42, y: 99 };
    let Point2D { x: px, y: py } = p;  // JavaScript: const { x: px, y: py } = p;
    println!("   Object destructuring: ({}, {})", px, py);

    // if let - corresponds to JavaScript's `if (condition)`
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("   Favorite color: {}", color);
    } else {
        println!("   No favorite color");
    }
    // Similar to JavaScript: `if (favoriteColor) { console.log(favoriteColor); }`
}

// ============================================================
// Section 10: Collections (컬렉션)
// ============================================================

// Rust collections corresponding to JavaScript's Array, Object (=Map)
// - JavaScript's Array and Rust's Vec are similar, but size is not fixed
// - JavaScript's Object(Map) and Rust's HashMap are similar, but type-safe

fn section_10_collections() {
    // Vec (Vector) corresponding to JavaScript's Array
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];  // JavaScript: let numbers = [1, 2, 3, 4, 5];
    println!("Section 10 - Vec: {:?}", numbers);

    // Corresponds to JavaScript's array.push
    numbers.push(6);
    println!("   After push: {:?}", numbers);

    // Corresponds to JavaScript's array.forEach
    for num in &numbers {
        println!("   num: {}", num);
    }

    // Corresponds to JavaScript's array.map
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   map(x*2): {:?}", doubled);

    // Corresponds to JavaScript's array.filter
    let evens: Vec<i32> = numbers.iter().filter(|x| **x % 2 == 0).copied().collect();
    println!("   filter(even): {:?}", evens);

    // HashMap corresponding to JavaScript's Map
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Rust".to_string(), 95);   // JavaScript: scores.set("Rust", 95);
    scores.insert("JavaScript".to_string(), 85);
    scores.insert("Python".to_string(), 90);

    // JavaScript: scores.get("Rust")
    if let Some(score) = scores.get("Rust") {
        println!("   Rust score: {}", score);
    }

    // Similar to JavaScript's Object.keys
    for (language, score) in &scores {
        println!("   {} : {}", language, score);
    }
}

// ============================================================
// Section 11: Closures (클로저)
// ============================================================

// JavaScript's arrow function (`=>`) and Rust's closure (`|...|`)
// - JavaScript: `(x) => x * 2`
// - Rust: `|x| x * 2`
// - Both are "First-class functions" that can be passed as arguments to other functions

fn section_11_closures() {
    // Corresponds to JavaScript's `const double = (x) => x * 2;`
    let double_fn = |x| x * 2;  // Parameter type and return type are inferred
    println!("Section 11 - Closure: double(21) = {}", double_fn(21));

    // Explicitly specify types
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    println!("   add(3, 4) = {}", add(3, 4));

    // Corresponds to JavaScript's `arr.map(x => x * 2)`
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("   map(x*x): {:?}", squared);

    // Corresponds to JavaScript's `arr.filter(x => x > 2)`
    let greater_than_two: Vec<i32> = numbers.iter().filter(|x| **x > 2).copied().collect();
    println!("   filter(> 2): {:?}", greater_than_two);

    // Corresponds to JavaScript's `arr.reduce((acc, x) => acc + x, 0)`
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("   fold(sum): {}", sum);

    // Closures can capture variables from the surrounding scope
    // Similar to JavaScript's nested functions accessing outer variables
    let multiplier = 10;
    let multiply_by_ten = |x| x * multiplier;  // Captures multiplier
    println!("   captured: multiply_by_ten(5) = {}", multiply_by_ten(5));
}

// ============================================================
// Section 12: Traits (트레이트)
// ============================================================

// Similar to JavaScript's Duck Typing ("If it quacks like a duck, it is a duck"), but type-safe.
// Very similar to Java's Interface.
// - Java: `interface Printable { void print(); }`
// - Rust: `trait Printable { fn print(&self); }`
// JavaScript doesn't have Interface, but TypeScript does. Rust's Trait is most similar to TypeScript Interface.

fn section_12_traits() {
    // Corresponds to Java's `interface Drawable { void draw(); }`
    trait Drawable {
        fn draw(&self) -> String;  // JavaScript: `draw() { return '...'; }`
    }

    // Implement Trait on struct (corresponding to JavaScript's class)
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

    // Corresponds to JavaScript's `function drawAll(shape)`
    // But Rust is type-safe (can receive any Drawable implementor)
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];

    for shape in &shapes {
        println!("   {}", shape.draw());
    }

    // In JavaScript, to call `obj.toString()`, `obj` must have toString method
    // In Rust, implementing Trait automatically adds `to_string()`!
    let circle = Circle { radius: 5.0 };
    println!("   Circle: {:?}", circle);  // Output with Debug trait
}

// ============================================================
// Section 13: Error Handling (에러 처리)
// ============================================================

// Rust's Result<T, E> corresponding to JavaScript's try/catch
// - JavaScript: try/catch/finally
// - Rust: Result<T, E> enum (success: Ok(T), failure: Err(E))
// - Similar to Java's Checked Exception, but uses Result<T, E> enum.

fn section_13_error_handling() {
    // JavaScript: `try { parseInt("42"); } catch(e) { console.log(e); }`
    // Rust: Handle errors as values with `Result<T, E>`
    let parsed: Result<i32, std::num::ParseIntError> = "42".parse();
    match parsed {
        Ok(number) => println!("OK - Result: {}", number),
        Err(e) => println!("Error: {:?}", e),
    }

    // Rust's ? operator corresponding to JavaScript's `try { ... } catch(e) { return; }`
    fn safe_parse(s: &str) -> Result<i32, std::num::ParseIntError> {
        // JavaScript: try { return parseInt(s); } catch(e) { throw e; }
        // Rust: The `?` operator exits the function if an error occurs
        let num = s.parse::<i32>()?;  // If this value is Err, function immediately returns error
        Ok(num)  // If OK, wrap in Ok and return
    }

    // println!("safe_parse('456'): {:?}", safe_parse("456"));

    // Option<T> - type for handling JavaScript's null/undefined
    // Very similar to Java's Optional
    let maybe_name: Option<String> = Some("Alice".to_string());
    // JavaScript: const name = maybeName || 'Guest';
    // Rust: Provide default value with unwrap_or()
    let name = maybe_name.unwrap_or("Guest".to_string());
    println!("   name: {}", name);

    // ? operator - immediately returns None if None
    // fn demo() { let n = maybe_name?; } // Returns None immediately if None
}

// ============================================================
// Section 14: Generics (제네릭스)
// ============================================================

// JavaScript has no Generics (TypeScript does).
// Rust's Generics are very similar to TypeScript's Generics.

fn section_14_generics() {
    // JavaScript: function identity(x) { return x; }  // No type
    // Rust: Can explicitly define types with Generics
    fn identity<T>(value: T) -> T {
        value  // Works with any type
    }
    println!("Section 14 - Generics: identity(42) = {}", identity(42));
    println!("   identity(\"hello\") = {}", identity("hello"));

    // JavaScript: [1, "hello", true]  // Array elements can have different types
    // Rust: All elements in a generic array must be the same type
    let numbers: Vec<i32> = vec![1, 2, 3];
    println!("   Generic array: {:?}", numbers);

    // Rust's generic function corresponding to JavaScript's Array.map
    fn first<T>(slice: &[T]) -> &T {
        &slice[0]  // Return the first element
    }
    let numbers2: &[i32] = &[10, 20, 30];
    let first_num = first(numbers2);
    println!("   first([10, 20, 30]) = {}", first_num);
}

// ============================================================
// Section 15: Lifetimes (라이프타임)
// ============================================================

// Rust's Lifetime is a concept that doesn't exist in JavaScript.
// JavaScript automatically clears memory with GC, but Rust has no GC.
// Lifetime guarantees at compile time "how long this reference is valid".
// For JavaScript developers: "The thing JavaScript's GC does for you, you manage directly in Rust"

fn section_15_lifetimes() {
    // In most cases, the compiler infers even without Lifetime
    // For JavaScript developers: "The thing JavaScript's GC manages for you, in Rust it's explicit"

    // Simple example: Return the longer of two strings
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let result = longest("hello", "world!");
    println!("Section 15 - Lifetime: longest('hello', 'world!') = {}", result);
    // JavaScript: `const longest = (a, b) => a.length > b.length ? a : b;` same logic
    // But Rust guarantees at compile time "result won't outlive s1/s2"

    // Lifetime inference - in most cases, you don't need to specify
    fn get_first(s: &str) -> &str {
        // The compiler automatically infers the lifetime
        s
    }
    let word = get_first("hello lifetime");
    println!("   Auto inference: {}", word);
}

// ============================================================
// Section 16: Async/Await (비동기 처리)
// ============================================================

// Rust's async processing corresponding to JavaScript's `async/await`
// - JavaScript: `async function fetch() { const res = await fetch(url); }`
// - Rust: `async fn fetch() { let res = tokio::...await; }`
// - JavaScript uses event loop, Rust uses tokio runtime for async processing

async fn section_16_async_example() {
    // JavaScript's `const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));`
    // Similar to Rust's `tokio::time::sleep`
    println!("Section 16 - Async: Async processing example");
    // JavaScript: `setTimeout(() => console.log('after 100ms'), 100)`
    // Rust: `tokio::time::sleep(Duration::from_millis(100)).await;`
    println!("   Rust uses the same async/await pattern as JavaScript!");
}

// ============================================================
// Section 17: Modules & Crates (모듈 & 크레이트)
// ============================================================

// Rust's Module system corresponds to JavaScript's ES6 Module (import/export)
// - JavaScript: `import { foo } from './bar.js'` / `export function baz() {}`
// - Rust: `use crate::foo::bar;` / `pub fn baz() {}`
// - Rust's Crate = JavaScript's npm Package
// - Module = File/Folder structure that organizes code
// - `pub` keyword = JavaScript's `export` (controls visibility)

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
    // JavaScript의 `import`에 해당하는

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
            Err("Division by zero!".to_string())  // JavaScript: throw new Error("Division by zero!")
        } else {
            Ok(a as f64 / b as f64)  // JavaScript: return a / b
        }
    }

    // JavaScript의 `import`에 해당하는 것이 Rust의 `use` 키워드
    //   JavaScript: import { add, multiply } from './math.js';
    //   Rust: use module_name::{add, multiply};

    // 모듈 내의 함수 호출
    let result_add = add(5, 3);
    let result_multiply = multiply(4, 7);
    let result_divide = divide(10, 2).unwrap_or(0.0);
    println!("   add(5, 3) = {}, multiply(4, 7) = {}, divide(10, 2) = {:.1}",
        result_add, result_multiply, result_divide);

    // JavaScript의 `export default`에 해당하는 패턴
    // Rust에는 `export default`가 없지만, pub struct로 대체 가능합니다
    struct Calculator {
        history: Vec<String>,  // JavaScript: private field처럼 사용 (pub 없으면 외부 접근 불가)
    }

    impl Calculator {
        fn new() -> Self {
            // JavaScript: `constructor() { this.history = []; }`
            Calculator { history: Vec::new() }
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

// Rust's File I/O corresponds to JavaScript's `fs` module (Node.js)
// - JavaScript: `const fs = require('fs'); fs.readFileSync('file.txt', 'utf8');`
// - Rust: `std::fs::read_to_string("file.txt")`
// - Rust's error handling with Result<T, E> instead of try/catch
// - Similar to Node.js's `fs.promises.readFile()` but with Rust's type safety

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
        },
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
        for entry in entries.take(5) {  // 최대 5개만 표시 (JavaScript: files.slice(0, 5))
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
        println!("   수정 시간: {:?}", metadata.modified().unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH));
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

// Rust's Test system corresponds to JavaScript's Jest/Mocha/Vitest
// - JavaScript: `test('adds 1 + 2', () => { expect(add(1, 2)).toBe(3); })`
// - Rust: `#[test] fn test_add() { assert_eq!(add(1, 2), 3); }`
// - Rust tests run with `cargo test` (JavaScript: `npm test`)
// - Same-file testing (JavaScript: separate __tests__ files)

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
    // JavaScript의 `expect(actual).not.toBe()`에 해당하는 해당하는 것이 Rust의 `assert_ne!`

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
    println!("   is_even_for_test(4) = {} (expect: true)", is_even_for_test(4));

    // JavaScript: expect(isEven(3)).toBeFalsy();
    // Rust: assert!(!is_even_for_test(3));
    println!("   is_even_for_test(3) = {} (expect: false)", is_even_for_test(3));

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
        panic!("This test should panic");  // JavaScript: throw new Error("test");
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

// Rust's Iterator trait corresponds to JavaScript's array methods and Generators
// - JavaScript: `arr.map(x => x * 2)` / `for (const x of arr)`
// - Rust: `arr.iter().map(|x| x * 2)` / `for x in arr.iter()`
// - Rust Iterators are Lazy (JavaScript의 Generator처럼 즉시 실행 안함)
// - JavaScript의 `for...of` = Rust의 `for x in iterator`
// - Rust의 Iterator는 성능이 매우 우수함 (컴파일타임 최적화)

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
    println!();  // 줄바꿈 (JavaScript: console.log()와 동일)

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
    let chained: Vec<i32> = numbers.iter()
        .filter(|x| **x > 2)      // JavaScript: filter(x => x > 2)
        .map(|x| *x * 2)           // JavaScript: map(x => x * 2)
        .take(3)                   // JavaScript: slice(0, 3)
        .collect();
    println!("   chain pipeline: {:?}", chained);

    // Iterator는 JavaScript의 Array method보다 더 많은 옵션을 제공합니다:
    // JavaScript: map, filter, reduce, find, some, every, forEach, includes, indexOf, etc.
    // Rust: map, filter, fold, find, any, all, for_each, contains, position, etc. + take, skip, step_by, zip, etc.
}

// ============================================================
// Section 21: Concurrency (동시성)
// ============================================================

// Rust's Concurrency corresponds to JavaScript's Worker Threads and Web Workers
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
        42  // Return value (JavaScript: `self.postMessage(42)`)
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
            tx.send(i).unwrap();  // JavaScript: `postMessage(i)`
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

// Cargo is Rust's built-in package manager, similar to npm/yarn/pnpm
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
// Main function - executes all sections
// ============================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust Tutorial - Rust Syntax Guide for JavaScript Developers");
    println!("============================================================\n");

    // Sections 1~15 are synchronous functions, call directly
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

    // Section 16 is async function, needs await
    section_16_async_example().await;

    // Sections 17~22 are synchronous functions, call directly
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
