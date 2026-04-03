//! gRPC 서버 예제 (tonic)
//!
//! 이 파일은 Rust 로 gRPC 서버를 구축하는 방법을 보여줍니다.
//! Java 의 grpc-java 나 JavaScript 의 @grpc/grpc-js 와 비교한 개념입니다.

// ============================================================================
// 1. IMPORT
// ============================================================================

// tonic: gRPC 프레임워크
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tokio::io::AsyncWriteExt;

// anyhow: 에러 처리
use anyhow::Result;

// 표준
use std::time::SystemTime;

// ============================================================================
// 2. 생성된 코드 import
// ============================================================================
//
// tonic-build 가 proto 파일로부터 컴파일하여 생성한 모듈
// 이 코드는 build.rs 실행 시 자동으로 생성됩니다
//
// tonic::hello: proto 의 package hello; 에 해당
// - HelloRequest, HelloResponse: proto 의 message 정의
// - HelloService: proto 의 service 정의
// - HelloServiceServer: 서버 구현을 위한 trait
// - HelloServiceSvc: 서비스 구현을 위한 타입체
mod hello {
    tonic::include_proto!("hello");
}

// ============================================================================
// 3. 서버 구현 (Service Implementation)
// ============================================================================
//
// HelloService: proto 파일에서 정의한 서비스
// Rust 의 trait 구현 Java 의 interface 와 유사
//
// #[tonic::async_trait]: async 메서드를 trait 에 정의할 수 있게 함
// Rust 의 async trait 는 아직 안정 실험적 기능이므로, tonic 는 어트리뷰트를 사용
#[tonic::async_trait]
impl hello::HelloService for HelloServiceImpl {
    // ------------------------------------------------------------------------
    // 1. unary RPC: Greet
    // ------------------------------------------------------------------------
    //
    // Java: public Future<HelloResponse> greet(HelloRequest request)
    // JavaScript: async greet(request, callback) { ... }

    async fn greet(
        &self,
        request: Request<hello::HelloRequest>,
    ) -> Result<Response<hello::HelloResponse>, Status> {
        // request.into_inner(): tonic 의 Request<T>에서 T 를 추출
        // Java 의 request.getBody()나 JavaScript 의 request.value 와 유사
        let req = request.into_inner();

        // tracing: 로깅 (Java 의 logger.info()나 JavaScript 의 console.log 와 유사)
        tracing::info!("greet 호출됨: name={}, age={}", req.name, req.age);

        // 인사 메시지 생성
        let greeting = format!(
            "Hello, {}!{}이/가 Rust gRPC 서버입니다.",
            req.name,
            if req.age > 0 { format!(" ({} 세)", req.age) } else { "".to_string() }
        );

        // Unix timestamp: 1970 년 1 월 1 일부터의 초 수
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        // Ok(Response::new(...)): 성공 응답 (Java 의 CompletableFuture.completedFuture()와 유사)
        Ok(Response::new(hello::HelloResponse {
            greeting,
            timestamp,
            server_name: "Rust-Tonic-Server".to_string(),
        }))
    }

    // ------------------------------------------------------------------------
    // 2. Server streaming RPC: GreetStream
    // ------------------------------------------------------------------------
    //
    // Java: public Flux<HelloResponse> greetStream(HelloRequest request)
    // JavaScript: async function* greetStream(request) { yield response1; yield response2; }
    //
    // ServerStreamingResponse: 서버가 여러 여러 응답을 스트
    // tokio::io::empty(): 비 스트림 생성
    async fn greet_stream(
        &self,
        request: Request<hello::HelloRequest>,
    ) -> Result<Response<tonic::Streaming<hello::HelloResponse>>, Status> {
        let req = request.into_inner();
        tracing::info!("greet_stream 호출됨: name={}", req.name);

        // --------------------------------------------------------------------
        // 채널 (channel) 생성
        // --------------------------------------------------------------------
        //
        // tokio::sync::mpsc: Multi-Producer, Single-Consumer 채널
        // - Sender: 메시지 전송 (여러 곳에서 사용 가능)
        // - Receiver: 메시지 수신 (한 곳에서만 사용)
        // - Java 의 BlockingQueue 나 JavaScript 의 MessageChannel 와 유사
        //
        // 인자 (8): 채널 버퍼 크기 (동시에 저장할 수 있는 메시지 수)
        let (mut tx, rx) = tokio::sync::mpsc::channel(8);

        // --------------------------------------------------------------------
        // 비동기 태스크 생성 (tokio::spawn)
        // --------------------------------------------------------------------
        //
        // 이 태스크는 백가 여러 응답을 생성하여 채널 채널을 통해 전송
        // 클라이언트는 이 스트을 통해 응답을 수신
        let name = req.name.clone();
        tokio::spawn(async move {
            for i in 0..5 {
                let message = format!("Stream 메시지 message {} from {}", i + 1, name);
                tracing::info!("스트림 메시지 전송: {}", message);

                // 메시지 메시지 전송 (1 초 간격)
                let _ = tx.send(hello::HelloResponse {
                    greeting: message,
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0),
                    server_name: "Rust-Tonic-Server".to_string(),
                }).await;

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        // ServerStreamingResponse: tonic 이 제공하는 스트림 응답
        // rx (Receiver) 를로 변환
        Ok(Response::new(tonic::Streaming::new(rx)))
    }

    // ------------------------------------------------------------------------
    // 3. Client streaming RPC: GreetClientStream
    // ------------------------------------------------------------------------
    //
    // Java: public CompletableFuture<HelloResponse> greetClientStream(Flux<HelloRequest> requestStream)
    // JavaScript: async greetClientStream(call) { const requests = []; for await (const req of call) { requests.push(req); } ... }
    //
    // Streaming<T>: 클라이언트가 스트 스트림
    // Java 의 Flux<T>나 JavaScript 의 async iterable 과 유사
    async fn greet_client_stream(
        &self,
        request: Request<Streaming<hello::HelloRequest>>,
    ) -> Result<Response<hello::HelloResponse>, Status> {
        tracing::info!("greet_client_stream 호출됨");

        let mut request_stream = request.into_inner();
        let mut total_count = 0usize;
        let mut messages = Vec::new();

        // --------------------------------------------------------------------
        // 스트림에서 메시지 수신
        // --------------------------------------------------------------------
        //
        // .next().await: 스트림에서 다음 메시지 수신
        // None 이 반환될 스트림이 종료됨
        while let Some(req) = request_stream.message().await? {
            total_count += 1;
            messages.push(req.name);
            tracing::info!("클라이언트 스트 수신: name={}", req.name);
        }

        Ok(Response::new(hello::HelloResponse {
            greeting: format!(
                "Received {} messages from: {}",
                total_count,
                messages.join(", ")
            ),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            server_name: "Rust-Tonic-Server".to_string(),
        }))
    }

    // ------------------------------------------------------------------------
    // 4. Bidirectional streaming RPC: GreetBidirectional
    // ------------------------------------------------------------------------
    //
    // Java: public Flux<HelloResponse> greetBidirectional(Flux<HelloRequest> requestStream)
    // JavaScript: async function* greetBidirectional(call) { for await (const req of call) { yield response; } }
    //
    // 양방향 스트림: 클라이언트와 서버가 동시에 메시지 송
    // WebSocket 의 양방향 통신과 유사
    async fn greet_bidirectional(
        &self,
        request: Request<Streaming<hello::HelloRequest>>,
    ) -> Result<Response<tonic::Streaming<hello::HelloResponse>>, Status> {
        tracing::info!("greet_bidirectional 호출됨");

        let request_stream = request.into_inner();

        // 채널 채널 생성 (응답 전송림용)
        let (mut tx, rx) = tokio::sync::mpsc::channel(8);

        // 비동기 태스크 생성
        tokio::spawn(async move {
            // 클라이언트에서 온 메시지를 받아하고, 응답을 전송
            for await req in request_stream {
                match req {
                    Ok(req) => {
                        tracing::info!("양방향 메시지 수신: name={}", req.name);

                        // 에코 응답 (클 메시지를 다시 보냄)
                        let _ = tx.send(hello::HelloResponse {
                            greeting: format!("Echo: {}", req.name),
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .map(|d| d.as_secs() as i64)
                                .unwrap_or(0),
                            server_name: "Rust-Tonic-Server".to_string(),
                        }).await;
                    },
                    Err(e) => {
                        tracing::error!("양방향 스트 수신 에러: {:?}", e);
                        break;
                    },
                }
            }
        });

        Ok(Response::new(tonic::Streaming::new(rx)))
    }
}

// ============================================================================
// 4. 서버 구현체 (Struct)
// ============================================================================
//
// Java 의 @Service public class HelloServiceImpl implements HelloService
// JavaScript 의 class HelloServiceImpl implements HelloService
#[derive(Default)]
struct HelloServiceImpl;

// ============================================================================
// 5. MAIN 함수
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // ========================================================================
    // tracing 초기화 (로깅 설정)
    // ========================================================================
    //
    // Java 의 Log4j/SLF4J 설정이나 JavaScript 의 winston 설정과 유사
    tracing_subscriber::fmt::init();

    // ========================================================================
    // 서버 주소 설정
    // ========================================================================
    //
    // [::1]: IPv6 의 localhost (0.0.0.0 과 모든 인터페이스)
    // 50051: gRPC 의 기본 포트 (HTTP/2 사용)
    let addr = "[::1]:50051".parse()?;

    println!("🚀 gRPC 서버를 시작합니다...");
    println!("📝 서버 주소: {}", addr);

    // ========================================================================
    // 서버 빌드 및 실행
    // ========================================================================
    //
    // Server::builder(): gRPC 서버 빌더 (Java 의 ServerBuilder 와 유사)
    // .add_service(): 서비스 등록 추가 (proto 에서 정의한 서비스 구현 구현체 연결)
    // .serve(): 서버 시작 (비동기)
    Server::builder()
        .add_service(hello::HelloServiceServer::new(HelloServiceImpl::default()))
        .serve(addr)
        .await?;

    println!("✅ gRPC 서버가 실행되었습니다 중입니다!");

    Ok(())
}