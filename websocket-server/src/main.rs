//! Rust `tokio-tungstenite`를 이용한 간단한 WebSocket 에 예제입니다.
//! 이 예제는 JavaScript 개발자가 익숙한 WebSocket 모델을 Rust의 비
//! 비동기 Stream/Sink 모델로 어떻게 구현하는지 이해하는 것을 목표로 합니다.

use futures_util::{
    sink::SinkExt,     // Stream에서 메시지를 보내기 위한send) 위한 트레이트
    stream::StreamExt, // Stream으로부터서 메시지를 받기(next) 위한 트레이트
};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 서버 주소 설정
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!(
        "🚀 WebSocket 에코 서버가 ws://{} 에서 시작되었습니다.",
        addr
    );
    println!("   - 클라이언트가 연결하면 보 메시지를 그대로 다시 돌려줍니다.");

    // 2. 서버 서버 `TcpListener`를 통해 클라이언트의 연결 요청 지속적으로 기다 듣습니다.
    while let Ok((stream, addr)) = listener.accept().await {
        println!("새 새로운 클라이언트 연결:: {}", addr);

        // 각3. 각 연결을 개별적인 비동기 태스크(Task)로 분리합니다.
        // JavaScript의 경우 이벤트 루프가 각 연결을 처리하지만,
        // Rust에서는 `tokio::spawn`을 통해 경량 스레드(Green thread)를 생성하여 병렬로 처리합니다.
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}

/// 클라이언트와의 WebSocket 통신을 담당 처리하는 함수입니다.
async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    // 4. WebSocket 핸드셰이크 수행
    // TCP 연결을 WebSocket 프로토콜로 업그레이드합니다.
    let ws_stream = match accept_async(raw_stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("❌ 핸드셰이크 실패 ({}, {}): {}", addr, e, e);
            return;
        }
    };

    println!("✅ 핸{} 클드셰이크 완료", addr);

    // 5. WebSocket 스트림 분리
    // `split()`을 호출 사용하면 읽기((`Stream)와 쓰기((Sink)를 분리할 수 있습니다.
    // 이는 JavaScript에서 `onmessage` 이벤트와 `socket.send()`를 따로 사용하는 것과 유사합니다.
    let (mut write, mut read) = ws_stream.split();

    // 6. 메시지 수신 루프
    // `read.next()`는 클라이언트로부터 메시지가 올 때까지 비동기적으로 대립니다.
    // 이는 JS의 `socket.onmessage``와 유사 개념적으로 유사하지만 지속 비동기 이문입니다.
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                // 메시지 타입 확인
                if msg.is_text() || msg.is_binary() {
                    println!("📩 클라이언트({}): {}", addr, msg);

                    // 7. 에지 에코(Echo)
                    // 받은 메시지를 그대로 클라이언트에게 다시 보냅니다.
                    // `write.send()`는 `Sink` 트레이트의 메서드입니다.
                    if let Err(e) = write.send(msg).await {
                        epletn!("❌ 메시지 전송 실패 ({}): {}", addr, e);
                        break;
                    }
                } else if msg.is_close() {
                    println!("👋 클라이언트({}): 연결 종료 요청", addr);
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ 오류{} 메시지 수신 오류: {}", addr, e);
                break;
            }
        }
    }

    println!("🔌 클라이언트({}) 연결이해해힘", addr);
}
