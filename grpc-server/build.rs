// ============================================================================
// build.rs: 빌 파일
// ============================================================================
//
// Rust 의 빌드 스크립트 (Java 의 pom.xml 의드 플러그인이나
// JavaScript 의 build script 와 유사)
//
// 이 파일은 cargo build 실행 시 자동으로 실행됩니다.
// tonic-build 를 사용하여 .proto 파일을 컴파일하여 Rust 코드를 생성합니다

fn main() {
    // ========================================================================
    // tonic_build::compile_protos() 실행
    // ========================================================================
    //
    // compile_protos: 하나 파일 컴파일 함수
    // 인자: 컴파일할 .proto 파일 경로 (glob 패턴 지원)
    //
    // 이 함수는 다음 작업을 수행합니다
    // 1. .proto 파일을 파싱
    // 2. Protocol Buffers 코드 구조 -> Rust struct 생성
    // 3. gRPC 서비스 정의 -> Rust trait 생성
    // 4. 생성된 코드를 build.rs 의 출력 디렉토리에 저장
    //
    // Java 의 protoc --java_out=...와 유사한 개념

    tonic_build::compile_protos("proto/hello.proto").expect("Failed to compile proto file");

    println!("✅ Proto 파일이 성공파일되었습니다!");
}
