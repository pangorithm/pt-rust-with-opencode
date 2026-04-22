// build.rs는 cargo가 컴파일될 때 실행되는 스크립트입니다.
// tonic-build를 사용하여 .proto 파일을 Rust 코드로 자동 생성합니다.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // rust-analyzer는 OUT_DIR 환경 변수를 설정하지 않습니다.
    // OUT_DIR가 없으면 빌드 스크립트를 건너뛰고 rust-analyzer에서 에러가 발생하지 않도록 합니다.
    if std::env::var("OUT_DIR").is_err() {
        eprintln!("tonic-build: OUT_DIR not set (rust-analyzer mode). Skipping proto compilation.");
        return Ok(());
    }

    // proto 파일의 경로를 지정하여 코드를 생성합니다.
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
