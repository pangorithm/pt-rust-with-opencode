# Cargo 사용 가이드 - Rust 패키지 매니저 & 빌드 도구

이 가이드는 Rust의 패키지 매니저 `Cargo`의 주요 명령어를 설명하고, 이 프로젝트에서 실제로 빌드·실행하는 방법을 안내합니다.

---

## 📦 1. Cargo 기본 개념

### Cargo란?

**Cargo**는 Rust의 패키지 매니저이자 빌드 도구입니다.

| Rust (Cargo) | JavaScript (npm) | Java (Maven/Gradle) |
|---|---|---|
| Cargo.toml | package.json | pom.xml / build.gradle |
| Cargo.lock | package-lock.json | (version lock 있음) |
| crates.io | npm registry | Maven Central |

### 핵심 파일

| 파일 | 역할 | 유사 개념 |
|---|---|---|
| `Cargo.toml` | 의존성 및 프로젝트 설정 | package.json + pom.xml |
| `Cargo.lock` | 고정된 의존성 버전 | package-lock.json |
| `src/main.rs` | 메인 실행 파일 진입점 | - |
| `src/lib.rs` | 라이브러리 코드 | - |

### 프로젝트 구조

```
my_project/
├── Cargo.toml        # 프로젝트 설정
├── Cargo.lock        # 고정된 의존성
├── src/
│   ├── main.rs       # 실행 파일 진입점
│   └── lib.rs        | 라이브러리
└── target/           # 빌드 산출물 (Git에 커밋 X)
```

---

## 🔧 2. 주요 Cargo 명령어

### 🚀 빌드 & 실행

| 명령어 | 설명 |
|---|---|
| `cargo build` | 프로젝트 빌드 (`target/debug/`에 바이너리 생성) |
| `cargo run` | 빌드 후 바로 실행 |
| `cargo run --release` | 최적화 빌드 후 실행 (`target/release/`에 바이너리 생성). 배포용, 빌드 속도는 느리지만 실행 속도는 빠름 |

### 📦 의존성 관리

| 명령어 | 설명 | 유사 개념 |
|---|---|---|
| `cargo add <crate>` | 의존성 추가 (예: `cargo add serde`) | `npm install <package>` |
| `cargo remove <crate>` | 의존성 제거 | `npm uninstall <package>` |
| `cargo update` | `Cargo.lock`의 의존성 버전 업데이트 | `npm update` |

### 🧪 테스트

| 명령어 | 설명 |
|---|---|
| `cargo test` | 모든 테스트 실행 |
| `cargo test <test_name>` | 특정 테스트만 실행 |

### ⚡ 성능 & 품질

| 명령어 | 설명 | 유사 개념 |
|---|---|---|
| `cargo check` | 컴파일 체크만 (빌드 X). 빠르게 오류 발견. | `npm run lint`와 유사한 빠른 피드백 |
| `cargo clippy` | 정적 분석 및 코드 개선 제안 | `eslint` / `prettier` |
| `cargo fmt` | 코드 포맷팅 (Rust 표준 포맷터) | `Prettier` |

### 📋 기타 유용한 명령어

| 명령어 | 설명 |
|---|---|
| `cargo doc --open` | 문서 생성 + 브라우저에서 열기 |
| `cargo tree` | 의존성 트리 출력 |
| `cargo clean` | `target/` 디렉토리 전체 삭제 (빌드 캐시 초기화) |

---

## 📝 3. Cargo.toml 예시

### rest-api-server (Axum 기반 REST API)

```toml
[package]
name = "rest-api-server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### grpc-server (tonic 기반 gRPC 서버)

```toml
[package]
name = "grpc-server"
version = "0.1.0"
edition = "2021"

[dependencies]
tonic = "0.12"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
prost = "0.13"
prost-types = "0.13"
```

### websocket-server (tokio-tungstenite 기반 WebSocket 서버)

```toml
[package]
name = "websocket-server"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio-tungstenite = "0.26"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
futures-util = "0.3"
tracing = "0.1"
```

---

## 🏗️ 4. 프로젝트 빌드 & 실행

### 현재 프로젝트 구조

```
pt-rust-with-opencode/
├── grpc-server/        # gRPC 서버 (tonic)
├── rest-api-server/    # REST API 서버 (Axum)
└── websocket-server/   # WebSocket 서버 (tokio-tungstenite)
```

### 전체 프로젝트 빌드

각 프로젝트에서 아래 명령을 실행하여 빌드합니다:

```bash
# 전체 빌드 (3개 프로젝트 모두)
cd grpc-server        && cargo build
cd rest-api-server    && cargo build
cd websocket-server   && cargo build
```

### 개별 프로젝트 빌드

```bash
# grpc-server만 빌드
cd grpc-server && cargo build

# rest-api-server만 빌드
cd rest-api-server && cargo build

# websocket-server만 빌드
cd websocket-server && cargo build
```

### 프로젝트 실행

```bash
# 각 프로젝트 실행
cd grpc-server        && cargo run
cd rest-api-server    && cargo run
cd websocket-server   && cargo run
```

### 테스트 실행

```bash
# 각 프로젝트 테스트
cd grpc-server        && cargo test
cd rest-api-server    && cargo test
cd websocket-server   && cargo test
```

---

## 💡 5. Cargo 설치 확인

```bash
# Cargo 버전 확인
cargo --version

# Rust 버전 확인
rustc --version

# 설치가 안 된 경우
# Rust 설치: https://rustup.rs/
# Windows: winget install rustup 또는 choco install rustup
```

---

## 🚀 6. 자주 사용하는 워크플로우

### 빠른 시작

```bash
# 1. clone 후 바로 빌드
git clone <repo>
cd pt-rust-with-opencode/grpc-server
cargo build
cargo run
```

### 개발 중 빠른 피드백

```bash
# 컴파일 체크만 (빠름)
cargo check

# 린터 (클리포)
cargo clippy

# 코드 포맷팅
cargo fmt
```

### 의존성 추가

```bash
# 의존성 추가
cargo add <crate-name>

# 의존성 트리 확인
cargo tree

# 의존성 업데이트
cargo update
```

### 빌드 캐시 초기화

```bash
# target/ 전체 삭제
cargo clean
# 다시 빌드
cargo build
```
