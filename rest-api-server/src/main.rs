//! REST API 서버 예제 (axum 프레임워크)
//!
//! 이 파일은 Rust 의 웹 서버를 구축하는 방법을 보여를 보여줍니다.
//! JavaScript 의 Express.js 나 Java 의 Spring Boot 와 비교한 개념입니다.

// ============================================================================
// 1. IMPORT (Java 의 import 나 JavaScript 의 import 와 유사)
// ============================================================================

// axum 에서 제공하는 웹 타입들
use axum::{
    // extract: 요청에서 데이터를 추출하는 기능 (예: JSON 본문
    extract::Path,
    // Handler: 핸    // Request: HTTP 요청 객체
    http::StatusCode,
    // Router: 라우팅을 위한 핵심 타입 (Express 의 app.get(), app.post() 등을 정의)
    routing::{get, post},
    // Json: JSON 요청/응답을 자동으로 처리하는 타입
    Json,
    // Router: 라우트를을 정의하는 타입
    Router,
};

// tokio: 비동기 런타임
use tokio::net::TcpListener;

// serde: 직렬화/역직직렬화
use serde::{Deserialize, Serialize};

// 표준 라이브러리
use std::collections::HashMap;

// ============================================================================
// 2. STRUCT 정의
// ============================================================================

/// 상태 관리
///
/// Rust 의 struct 는 Java 의 class 나 JavaScript 의 object 와 유사합니다.
/// 하지만 Rust 의 struct 는 더 가볍고, 필드는 기본적으로 public 이 아닙니다.
///
/// #[derive(...)] 는 Rust 의 "프로시저 매크로" 기능으로,
/// 지정파일러가 자동으로 코드를 생성하게
/// - Serialize: Rust 데이터를 JSON 으로 변환하는 코드 자동 생성
/// - Deserialize: JSON 을 Rust 데이터로 변환하는 코드 자동 생성
#[derive(Debug, Serialize, Deserialize)]
struct User {
    // id: 숫 ID (Java 의 long 이나 JavaScript 의 number)
    // serde 의 rename: JSON 필드 이름을 "user_id"로 매핑
    #[serde(rename = "user_id")]
    id: u64,

    // name: 사용자 이름 (String 은 할당된 문자열, &str 은 문자열 참조)
    name: String,

    // email: 이메일 주소
    email: String,
}

/// POST 요청을 위한 입력 데이터
///
/// User 구조 유사하지만, ID 는 생성 시에 부여되므로 입력하지 않습니다.
#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

/// API 응답의 표준 포맷
///
/// Java 의 ResponseEntity<T>나 JavaScript 의 res.json()과 유사한 개념
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

// ============================================================================
// 3. APPLICATION STATE (Java 의 @Scope("singleton") 빈과 유사)
// ============================================================================

/// 애플리케이션 상태
///
/// Rust 의 Arc<AtomicU64>는 여러
/// - Arc: "Atomic Reference Counted"의 약자로. 여러 스레드에서 안전하게 공유할 수 있는 참조
///   (Java 의 AtomicInteger 나 JavaScript 의 global 변수와 유사한 개념)
/// - AtomicU64: 원자적인 64 비트 정수 (동시성 처리 시 안전한 증 연산 지원
///
/// 이 구조는 모든 요청에서 공유되어
#[derive(Default)]
struct AppState {
    // 사용자 저장소 (실제 애플리케이션에서는 데이터베이스 사용)
    // RwLock: 읽기/쓰기 잠금 (여러 스레드에서 안전하게 읽기 가능, 쓰기는 배타적)
    users: tokio::sync::RwLock<HashMap<u64, User>>,

    // 사용자 증가하는 ID 생성기
    next_id: tokio::sync::atomic::AtomicU64,
}

// ============================================================================
// 4. HANDLERS (Java 의 @Controller 나 JavaScript 의 route handler 와 유사)
// ============================================================================

/// GET / - 루 엔드포인트
///
/// async fn: 비동기 함수 (JavaScript 의 async function 나 Java 의 CompletableFuture 와 유사)
/// -> impl: 반환 타입을 명시하지 않고 컴
/// axum 은 반환 타입을 자동으로 추론합니다
///
/// State<AppState>: 의존성 주입 (DI). axum 이 자동으로 AppState 객체를 전달
async fn home() -> &'static str {
    // 문자열 리터럴을 반환 (axum 이 자동으로 HTTP 응답으로 변환)
    "Hello, Rust REST API!"
}

/// GET /users - 모든 사용자 목록
///
/// State<AppState>: 애플리케이션 상태에 접근
async fn list_users(
    // State<T>: axum 이 제공하는 주입하는 의존성 (Spring 의 @Autowired 와 유사)
    // extract::State 는
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    // .await: 비동기 작업을
    // Rust 의 .await 는 JavaScript 의 await 나 Java 의 .join()과 유사
    // 읽
    let users = state.users.read().await;

    // .values(): HashMap 의 값들만 추출
    // .cloned(): 복사를 실제 값으로 복사
    // .collect(): Iterator 를 Vec 으로 변환 (Stream API 의 collect 와 유사)
    let users_list = users.values().cloned().collect::<Vec<User>>();

    // Json<T>: axum 이 자동으로 JSON 으로 변환렬화하고
    Json(users_list)
}

/// GET /users/:id - 특정 사용자 조회
///
/// Path<T>: URL 경로에서 파라미터를 추출 (Express 의 req.params.id 나 Spring 의 @PathVariable)
async fn get_user(
    Path(user_id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;

    // .get(): HashMap 에서 값 조회 (Option<T> 반환)
    // .ok_or(): Option 을 Result 로 변환 (없
    // - Some(value) -> Ok(value)
    // - None -> Error(StatusCode)
    users
        .get(&user_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
        .map(Json)
}

/// POST /users - 새로운 생성
///
/// Json<T>: 요청 본문문을 자동으로 역 직렬화 (Jackson 의 @RequestBody 나 express 의 req.body)
async fn create_user(
    Json(create_request): Json<CreateUserRequest>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    // AtomicU64::fetch_add: 원자적으로 값을 증가하고 이전 값을 반환
    // Ordering::SeqCst: 가장 강력한 메모리 순서 보장 (동시성 처리용)
    let id = state
        .next_id
        .fetch_add(1, tokio::sync::atomic::Ordering::SeqCst);

    let new_user = User {
        id,
        name: create_request.name,
        email: create_request.email,
    };

    // .write(): 쓰기 잠금 획득 (RwLock 의 쓰기
    let mut users = state.users.write().await;
    users.insert(id, new_user.clone());

    // Ok(Json(...)): 성공 응답 (Java 의 ResponseEntity.ok()과 유사)
    Ok(Json(ApiResponse {
        success: true,
        message: "User created successfully".to_string(),
        data: Some(new_user),
    }))
}

// ============================================================================
// 5. MAIN 함수 (Java 의 main() 나 JavaScript 의 entry point)
// ============================================================================

#[tokio::main]
// #[tokio::main]: tokio 런타임을 사용하도록 지정
// 이 어트리뷰트 없이는 async fn main이 컴파일되지 않습니다.
// (JavaScript 의 경우 별도의 설정 없이 async function 사용 가능)
async fn main() {
    // ========================================================================
    // 애플리케이션 상태 초기화
    // ========================================================================

    let state = AppState {
        users: tokio::sync::RwLock::new(HashMap::new()),
        next_id: tokio::sync::atomic::AtomicU64::new(1),
    };

    // ========================================================================
    // 라우터 설정 (Express 의 app.get(), app.post() 와 유사)
    // ========================================================================

    let app = Router::new()
        // GET / - 홈 엔드포인트
        .route("/", get(home))
        // GET /users - 모든 사용자 조회
        .route("/users", get(list_users))
        // POST /users - 사용자 생성
        .route("/users", post(create_user))
        // GET /users/:id - 특정 사용자 조회 (:id 는 경로 파라미터)
        .route("/users/:id", get(get_user))
        // 상태 객체를 모든 핸들러에 주입 (Spring 의 @Bean 과 유사)
        .with_state(state);

    // ========================================================================
    // 서버 시작
    // ========================================================================

    // 0.0.0.0:3000: 모든 네트워크 인터페이스의 3000 포트
    // (Java 의 server.port=3000 나 JavaScript 의 app.listen(3000))
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 REST API 서버가 3000 포트에서 실행 중입니다!");
    println!("📝 다음 엔드포인트를 사용할 수 있습니다:");
    println!("   GET  /                    - 홈");
    println!("   GET  /users               - 모든 사용자 조회");
    println!("   GET  /users/:id           - 특정 사용자 조회");
    println!("   POST /users               - 사용자 생성");

    // axum::serve: axum 의 서버 시작 함수
    // .await: 비동기적으로 서버를 시작
    // .unwrap(): 결과에서 Ok 값을 추출, Error 발생 시 panic
    axum::serve(listener, app).await.unwrap();
}
