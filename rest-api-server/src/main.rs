//! Rust Axum 프레임워크를 이용한 간단한 REST API 서버 예제입니다.
//! 이 예제는 JavaScript/Java 개발자가 Rust의 타입 시스템과 비동기 처리를
//! 이해하는 것을 목표로 합니다.

use axum::{
    // 상태(State) 공유 핸들링을 위한 기능
    extract::{Path, State},
    // JSON 응답 및 요청 처리를 위한 추출(Extractor)
    routing::{get, post},
    // JSON 데이터 추출기
    Json,
    Router,
};
// 비동기 런타임 실행을 위한 Tokio
use tokio::net::TcpListener;
// Serde를 이용한 데이터 직렬화/역직렬화 매크로
use serde::{Deserialize, Serialize};
// 공유 상태를 관리하기 위한 원자적 참조 카운터 (Java의 Thread-safe object와 유사)
use std::sync::Arc;
// 멀티스레드 안전한 비동기 잠금 위한 Mutex
use tokio::sync::Mutex;

// --- 1. 데이터 모델 정의 ---

/// 사용자 정보를 나타내는 구조체입니다.
/// #[derive(Serialize, Deserialize)]는 Rust의 강력한 매크로 기능으로,
/// 이 구조체를 JSON으로 변환(Serialize)하거나 JSON을 이 구조체로
/// 변환(Deserialize)하는 코드를 자동으로 생성합니다.
/// Java의 Jackson이나 JS의 JSON.parse/stringify와 유사한 역할을 합니다.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
}

/// 서버의 공유 상태를 정의합니다.
/// Axum에서는 애플리케이션 전체에서 공유해야 할 데이터(DB 커넥션 풀, 메모리 DB 등)를
/// State로 전달하여 사용할 수 있습니다.
struct AppState {
    // 사용자 목록을 저장하는 Mutex를 포함한 벡터입니다.
    // Mutex는 여러 스레드에서 동시에 안전하게 데이터를 수정할 수 있도록 보장합니다.
    users: Mutex<Vec<User>>,
}

// --- 2. 핸들러 (Controller 역할 수행) ---

/// 모든 사용자를 조회하는 핸들러입니다.
/// `State` 추출기를 사용하여 공유된 `AppState`에 접근합니다.
async fn list_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    // Mutex를 잠가서 데이터를 가져옵니다.
    // lock()은 비동기적으로 작동하여 데이터에 접근할 수 있게 해줍니다.
    let users = state.users.lock().await;
    // Vec<User>를 JSON 응답으로 변환하여 반환합니다.
    Json(users.clone())
}

/// 특정 ID의 사용자를 조회하는 핸들러입니다.
/// `Path` 추출기를 사용하여 URL 경로의 변수(`:id`)를 가져옵니다.
async fn get_user(
    Path(id): Path<u64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>, String> {
    let users = state.users.lock().await;

    // Rust의 `iter().find()`와 `Option` 타입 활용
    // JS/Java의 find()와 비슷하지만, 결과가 반드시 존재하거나
    // Option<T> 타입으로 반환되어 null 처리를 안전하게 강제합니다.
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        // `ok_or_else`를 사용하여 Option을 Result로 변환하여 에러 처리를 유연하게 합니다.
        .ok_or_else(|| format!("User with id {} not found", id))
}

/// 새로운 사용자를 생성하는 핸들러입니다.
/// `Json<User>` 추출기를 사용하여 요청 본문의 JSON을 구조체로 자동 변환합니다.
async fn create_user(State(state): State<Arc<AppState>>, Json(new_user): Json<User>) -> Json<User> {
    let mut users = state.users.lock().await;
    users.push(new_user.clone());
    Json(new_user)
}

// --- 3. 메인 함수 (Server Setup) ---

#[tokio::main]
async fn main() {
    // 공유 상태 초기화
    // Arc(Atomic Reference Counted)는 여러 스레드 간 소유권을
    // 안전하게 공유하기 위해 사용합니다.
    let shared_state = Arc::new(AppState {
        users: Mutex::new(vec![User {
            id: 1,
            username: "rust_ace".to_string(),
            email: "ace@example.com".to_string(),
        }]),
    });

    // 라우터 설정
    // Axum은 매크로 없이 사용하여 매우 명시적으로 라우트를 정의합니다.
    let app = Router::new()
        // GET /users -> list_users 호출
        .route("/users", get(list_users))
        // POST /users -> create_user 호출
        .route("/users", post(create_user))
        // GET /users/:id -> get_user 호출
        .route("/users/:id", get(get_user))
        // 서버 전체에서 공유할 상태를 등록합니다.
        .with_state(shared_state);

    // 서버 실행 환경 설정
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 REST API 서버가 http://localhost:3000 에서 시작되었습니다.");
    println!("   - GET  /users (사용자 목록 조회)");
    println!("   - POST /users (사용자 생성)");
    println!("   - GET  /users/:id (특정 사용자 조회)");

    // 서버 시작
    axum::serve(listener, app).await.unwrap();
}
