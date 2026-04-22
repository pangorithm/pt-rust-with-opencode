# Rust 예제 프로젝트 가이드

## 프로젝트 목표

JavaScript/Java 개발자가 Rust를 배우기 위한 예제 프로젝트입니다.
각 프로젝트는 실제 웹 서버 구현을 통해 Rust의 핵심 개념을 학습합니다.

## 프로젝트 구조

```
pt-rust-with-opencode/
├── grpc-server/        # gRPC 서버 (tonic)
├── rest-api-server/    # REST API 서버 (Axum)
├── websocket-server/   # WebSocket 서버 (tokio-tungstenite)
└── PROJECT_GUIDE.md    # 이 파일
```

## 각 프로젝트의 핵심 개념

### grpc-server
- **Framework**: tonic
- **핵심 개념**: Protocol Buffers, gRPC, 비동기 RPC
- **학습 목표**: `.proto` 파일 → Rust 코드 생성, Unary/Streaming RPC
- **실행**: `cargo run`

### rest-api-server
- **Framework**: Axum
- **핵심 개념**: HTTP 라우팅, JSON 직렬화/역직렬화, 상태 공유
- **학습 목표**: REST API 설계, `serde`, `Arc`, `Mutex`, `Result` 에러 처리
- **실행**: `cargo run`

### websocket-server
- **Framework**: tokio-tungstenite
- **핵심 개념**: WebSocket 프로토콜, Stream/Sink, 비동기 통신
- **학습 목표**: 실시간 양방향 통신, TCP 스트림 → WebSocket 업그레이드
- **실행**: `cargo run`

## 개발 가이드

### 공통 의존성
모든 프로젝트에서 `tokio` 비동기 런타임을 사용합니다.

### 에러 처리
- `unwrap()` 대신 `?` 연산자 사용
- `Result` 반환으로 에러 전파

### 코드 스타일
- 주석은 JavaScript/Java의 유사 개념과 비교 설명
- 한국어 주석 사용
- 구조체/트레이트는 Java의 class/interface와 비교 설명

### 주의사항
- `target/` 디렉토리는 Git에 커밋하지 않음
- `Cargo.lock`은 각 프로젝트마다 별도 관리

---

# AI 어시스턴트 영구 지시사항

이 파일은 AI 어시스턴트(Sisyphus)에게 내려지는 영구적인 프로젝트 목표와 지시사항을 포함합니다.

## 프로젝트 성격
- **학습 목적**: 실제 웹 서버 구현을 통해 Rust의 핵심 개념을 배우는 예제 프로젝트
- **대상 독자**: JavaScript/Java 배경 지식을 가진 개발자
- **코드 퀄리티**: 학습용으로 충분하되, 모범 사례를 따름 (모범 예제)

## 코드 작성 원칙
1. `unwrap()` 대신 `?` 연산자 사용
2. `Result` 반환으로 에러 전파
3. 주석은 한국어로 작성
4. JavaScript/Java의 유사 개념과 비교 설명 포함
5. 구조체/트레이트는 Java의 class/interface와 비교 설명

## 영구 동작 규칙
- 모든 작업은 이 프로젝트의 학습 목적에 부합해야 함
- 코드는 실제 동작해야 하며, 컴파일/실행 가능한 상태여야 함
- 변경 시에는 minimal하게, 학습에 방해되지 않는 방향으로 수정
