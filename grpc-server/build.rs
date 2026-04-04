// build.rs는 cargo가 컴파일될 때 실행되는 스크립트입니다.
// tonic-build를 사용하여 .proto 파일을 Rust 코드로 자동 생성합니다.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // proto 파일의 경로를 지정하여 코드를 생성합니다.
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
