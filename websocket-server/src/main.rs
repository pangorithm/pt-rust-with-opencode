//! WebSocket 서버 예제 (tokio-tungstenite)
//!
//! 이 파일은 Rust 로 실시간 WebSocket 서버를 구축하는 방법을 보여줍니다.
//! JavaScript 의 ws 나 Java 의 Spring WebSocket 과 비교한 개념입니다.

// ============================================================================
// 1. IMPORT
// ============================================================================

use axum::{
    // extract: 요청에서 데이터를 추출하는 기능
    extract::{ws::WebSocketUpgrade, State},
    // response: 응답 응답 타입
    response::IntoResponse,
    // Router: 라우팅
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::RwLock;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

// ============================================================================
// 2. STRUCT 정의
// ============================================================================

/// 클라이언트로부터 메시지 (클라이언트 -> 서버)
///
/// #[derive(Deserialize)]: JSON 을 Rust 구조체로 변환
#[derive(Debug, Deserialize)]
struct ClientMessage {
    // 메시지 타입: "chat", "ping", "join" 등
    #[serde(rename = "type")]
    msg_type: String,

    // 메시지 내용 (선택 필드)
    #[serde(default)]
    content: String,

    // 클라이언트 ID (선택 필드)
    #[serde(default)]
    client_id: String,
}

/// 서버의 메시지 (서버 -> 클라이언트)
///
/// #[derive(Serialize)]: Rust 구조체를 JSON 으로 변환
#[derive(Debug, Serialize)]
struct ServerMessage {
    #[serde(rename = "type")]
    msg_type: String,
    content: String,
    timestamp: u64,
}

/// 애플리케이션 상태
///
/// broadcast::Sender: 1 대 N 메시지 브로드캐스트를 위한 채널
/// - Java 의 Publisher 나 JavaScript 의 EventEmitter 와 유사
/// - 하나의 한 메시지를 여러 구독자에게 동시에 전송
#[derive(Default)]
struct AppState {
    // 브 클라이언트에 메시지를 브로드캐스트하는 채널
    broadcast_tx: Sender<Message>,

    // 연결된 클라이언트 수
    client_count: RwLock<usize>,
}

// ============================================================================
// 3. HANDLER
// ============================================================================

/// WebSocket 업그레이 핸들러
///
/// WebSocketUpgrade: axum 이 제공하는 HTTP 연결을 WebSocket 으로 업그레이드
/// State<AppState>: 애플리케이션 상태
async fn ws_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // on_upgrade: WebSocket 업그레이드가 완료된 후 실행되는 콜 콜 콜
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// WebSocket 소켓 처리 함수
///
/// WebSocketStream: WebSocket 연결 스트림
///
/// Rust 의 비동기 스트림은 Java 의 Flux 나 JavaScript 의 async iterable 과 유사
async fn handle_socket(
    socket: WebSocketStream,
    state: Arc<AppState>,
) {
    println!("🔌 클라이언트 연결됨!");

    // 클라이언트 수 증가
    *state.client_count.write().await += 1;

    // ========================================================================
    // 스트림 분리 (Split)
    // ========================================================================

    // tokio_tungstenite::WebSocketStream 은 Read/Write 스트림
    // split()으로 읽기/쓰기를 분리 (Java 의 BufferedReader/Writer 분리 개념)
    let (mut read_stream, mut write_stream) = socket.split();

    // broadcast 채널의 구독자 (receiver) 생성
    // 이 receiver 는 서버에서 브로드캐스트한 메시지를 받음
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    // ========================================================================
    // 비동기 태스크 병렬 실행 (tokio::spawn)
    // ========================================================================

    // tokio::spawn: 새로운 비동기 태스크를 생성
    // JavaScript 의 setImmediate() 나 Java 의 CompletableFuture.runAsync()와 유사
    // 두 두 태 태스크를 병 실행:
    // 1. 클라이언트에서 온 메시지 읽기
    // 2. 서버에서 브로드캐스트한 메시지를 읽

    // 태스크 1: 클라이언트 메시지 읽기
    let read_handle = tokio::spawn({
        let broadcast_tx = state.broadcast_tx.clone();

        async move {
            loop {
                match read_stream.next().await {
                    // 클라이언트에서 메시지 수신
                    Some(Ok(Message::Text(text))) => {
                        println!("📨 메시지: {}", text);

                        // 메시지를 모든 클라이언트에 브로드캐스트
                        // broadcast 는 send 는 Ok/Err 를 반환
                        // Err(Disconnected): 구독자가 모두 해제 경우
                        if broadcast_tx.send(Message::Text(text)).is_err() {
                            println!("❌ 브로드캐스트 채널이 닫혔습니다.");
                            break;
                        }
                    },
                    // 이진 메시지 처리
                    Some(Ok(Message::Binary(bytes))) => {
                        println!("📦 이진 메시지 수신: {} bytes", bytes.len());
                    },
                    // 클라이언트가 연결 종료
                    Some(Ok(Message::Close(_))) => {
                        println!("👋 클라이언트가 연결을 종료했습니다.");
                        break;
                    },
                    // 에러 처리
                    Some(Ok(Message::Ping(_))) => {
                        // Ping 메시 Pong 은 자동 처리
                    },
                    Some(Ok(Message::Pong(_))) => {
                        // Pong 수신
                    },
                    Some(Err(e)) => {
                        eprintln!("❌ 읽기 에러: {}", e);
                        break;
                    },
                    // 스트림 종료
                    None => {
                        println!("🔚 스트림이 종료혔습니다.");
                        break;
                    },
                }
            }
        }
    });

    // 태스크 2: 브로드캐스트 메시지 쓰기
    let write_handle = tokio::spawn({
        async move {
            loop {
                // broadcast receiver 에서 메시지 수신
                match broadcast_rx.recv().await {
                    Ok(message) => {
                        // 클라이언트에 메시지 전송
                        if write_stream.send(message).await.is_err() {
                            println!("❌ 쓰기 에러: 클라이언트 연결이 끊어졌습니다.");
                            break;
                        }
                    },
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // 버퍼가  고 메시지를 놓친
                        // receiver 가 메시지를 너무지 못했을 경우
                        println!("⚠️ 브 놓쳤습니다 (버퍼 포만)");
                    },
                    Err(broadcast::error::RecvError::Closed) => {
                        // 브로드캐스트 채널이 닫힘
                        println!("🔚 브로드캐스트 채널이 닫혔습니다.");
                        break;
                    },
                }
            }
        }
    });

    // ========================================================================
    // 두 태스크가 모두 완료될 때까지 대기림
    // ========================================================================

    // tokio::join: 여러 비동기 태스크를 동시에 실행하고 결과 수집림
    // JavaScript 의 Promise.all()과 유사
    let (read_result, write_result) = tokio::join!(read_handle, write_handle);

    // 태스크 결과 확인 (에러가 있는지)
    if let Err(e) = read_result {
        eprintln!("❌ 읽기 태스크 에러: {:?}", e);
    }
    if let Err(e) = write_result {
        eprintln!("❌ 쓰기 태스크 에러: {:?}", e);
    }

    // 클라이언트 수 감소
    *state.client_count.write().await -= 1;
    println!("📉 연결 연결된 클라이언트: {}", *state.client_count.read().await);
}

/// 홈 엔드포인트
async fn home() -> &'static str {
    "WebSocket 서버가 실행 중입니다! /ws 엔드포인트로 연결하세요."
}

// ============================================================================
// 4. MAIN 함수
// ============================================================================

#[tokio::main]
async fn main() {
    println!("🚀 WebSocket 서버를 시작합니다...");

    // ========================================================================
    // broadcast 채널 생성
    // ========================================================================

    // broadcast::channel: 1 대 N 브로드캐스트 채널 생성
    // 첫 번째 인자 (16): 버퍼 크기 (버퍼에 저장할 수 있는 최대 메시지 수)
    // 반환값: (Sender, Receiver)
    // - Sender: 메시지 전송 (AppState 에 저장)
    // - Receiver: 메시지 수신 (각하지 않음, subscribe() 로 새 구독 생성
    let (broadcast_tx, _) = broadcast::channel(16);

    // ========================================================================
    // 애플리케이션 상태 생성
    // ========================================================================

    let state = Arc::new(AppState {
        broadcast_tx,
        client_count: RwLock::new(0),
    });

    // ========================================================================
    // 라우터 설정
    // ========================================================================

    let app = Router::new()
        // GET / - 홈
        .route("/", get(home))
        // GET /ws - WebSocket 엔드포인트
        .route("/ws", get(ws_handler))
        // 상태 주입
        .with_state(state);

    // ========================================================================
    // 서버 시작
    // ========================================================================

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    println!("🚀 WebSocket 서버가 3001 포트에서 실행 중입니다!");
    println!("📝 사용 방법:");
    println!("   - 홈: ws://localhost:3001/ws");
    println!("   - WebSocket 클라이언트로 /ws 엔드포인트에 연결하세요.");

    axum::serve(listener, app).await.unwrap();
}