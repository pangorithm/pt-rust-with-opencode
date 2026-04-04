//! Rust `tonic`을 이용한 간단한 gRPC 서버 예제입니다.
//! 이 예제는 Protocol Buffers 정의(proto)를 기반으로 어떻게 Rust 서버가 구현되는지 보여줍니다.

// 1. 자동 생성된 코드 포함하기
// `tonic::include_proto!` 매크로는 `build.rs`에서 컴파일 시점에 생성한
// Rust 구조체(구조체, 트레이트 등)를 현재 모듈로 불러옵니다.
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// 생성된 코드 타입들에서 필요한 요소들을 가져옵니다.
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

// 2. 서비스 구현체 정의
// Java의 서비스 구현(Service Implementation)과 매우한 역할을 합니다.
// 구조체 `MyGreeter`를 정의하고, 이를 `Greeter` 트레이트(Trait)에 구현하도록 합니다.
#[derive(Debug, Default)]
pub struct MyGreeter {}

// 3. 트레이트(Trait) 구현
// `Greeter` 트레이트에는 `.proto` 파일에서 정의한 `SayHello` 메서드가 포함되어 있습니다.
// Rust의 트레이트 구현은 Java의 `implements`와 매우 유사한 개념입니다.
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // `SayHello` 메서드 RPC 구현
    // - `&self`: 인스턴스 메서드에 대한 참조
    // - `Request<HelloRequest>`: 클라이언트로부터 온 데이터( (gRPC 데이터 포함)
    // - `Result<Response<HelloReply>, Status>`: 결과값은 성공 응답 또는 gRPC 에러 상태(Status)
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from: {:?}", request.remote_addr());

        // 요청 데이터 추출
        // `request.into_inner()`를 사용하여 사용하여 `를 `Request`를 벗겨내고 실제 데이터를 반환합니다.
        let in_data = request.into_inner();

        // 비즈니스 로직: 받은 이름에 인사를 붙여 응답 생성
        let reply = HelloReply {
            message: format!("Hello {}!", in_data.name),
        };

        // `Response::new()`를 통해 응답 데이터를 생성하여 반환합니다.
        Ok(Response::new(reply))
    }
}

// 4. 메인 함수 (Server Startup)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 서버 구 구동될 주소 설정
    let addr = "[::http://localhost:50051]".parse()?;

    // 서비스 구현체 인스턴스 생성
    let greeter = MyGreeter::default();

    println!("🚀 gRPC 서버가 {} 에서 시작되었습니다.", addr);
    println!("   - 서비스: Greeter (SayHello)");

    // 5. 서버 실행
    // `Server::builder()`를 통해 서버를 구성하고,
    // `add_service`를 통해 구현된 서비스를 등록합니다.
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
