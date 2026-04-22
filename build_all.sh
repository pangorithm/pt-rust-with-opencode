#!/bin/bash
# Cargo 사용 가이드 - Rust 패키지 매니저 & 빌드 도구
# 이 스크립트는 cargo 명령어를 설명하고, 프로젝트별 빌드/테스트를 실행합니다.

set -euo pipefail

SCRIPT_DIR="$(dirname "$0")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

print_subheader() {
    echo -e "${CYAN}  ── $1 ──${NC}"
    echo ""
}

print_usage() {
    print_header "📖 Cargo 사용 가이드"
    
    echo -e "${YELLOW}이 스크립트는 Rust의 패키지 매니저 'Cargo'의 주요 명령어를${NC}"
    echo -e "${YELLOW}설명하고, 이 프로젝트에서 실제로 빌드·실행합니다.${NC}"
    echo ""
    echo -e "${GREEN}사용법:${NC}"
    echo -e "  chmod +x build_all.sh    # 실행 권한 부여 (Linux/macOS)"
    echo -e "  ./build_all.sh           # 전체 실행"
    echo -e "  ./build_all.sh --help    # 이 도움말 표시"
    echo -e "  ./build_all.sh grpc      # grpc-server만 빌드·실행"
    echo -e "  ./build_all.sh --build   # 빌드만 (실행 없이)"
    echo -e "  ./build_all.sh --test    # 테스트 실행"
    echo ""
}

show_basic_concepts() {
    print_header "📦 1. Cargo 기본 개념"
    
    print_subheader "Cargo란?"
    echo -e "  ${GREEN}Cargo${NC}는 Rust의 패키지 매니저이자 빌드 도구입니다."
    echo -e "  npm(Node.js), Gradle(Java), Maven(Java)와 유사합니다."
    echo ""
    echo -e "  ${YELLOW}핵심 파일:${NC}"
    echo -e "    Cargo.toml    → 의존성 및 프로젝트 설정 (package.json + pom.xml)"
    echo -e "    Cargo.lock    → 고정된 의존성 버전 (package-lock.json)"
    echo -e "    src/main.rs   → 메인 실행 파일"
    echo -e "    src/lib.rs    → 라이브러리 코드"
    echo ""
    echo -e "  ${YELLOW}프로젝트 구조:${NC}"
    echo "    my_project/"
    echo "    ├── Cargo.toml        # 프로젝트 설정"
    echo "    ├── Cargo.lock        # 고정된 의존성"
    echo "    ├── src/"
    echo "    │   ├── main.rs       # 실행 파일 진입점"
    echo "    │   └── lib.rs        # 라이브러리"
    echo "    └── target/           # 빌드 산출물 (Git에 커밋 X)"
    echo ""
}

show_commands() {
    print_header "🔧 2. 주요 Cargo 명령어"
    
    print_subheader "🚀 빌드 & 실행"
    echo -e "  ${GREEN}cargo build${NC}"
    echo -e "    → 프로젝트 빌드 (target/debug/에 바이너리 생성)"
    echo ""
    echo -e "  ${GREEN}cargo run${NC}"
    echo -e "    → 빌드 후 바로 실행"
    echo -e "    → 이 프로젝트: 각 하위 프로젝트에서 cargo run"
    echo ""
    echo -e "  ${GREEN}cargo run --release${NC}"
    echo -e "    → 최적화 빌드 후 실행 (target/release/에 바이너리 생성)"
    echo -e "    → 배포용, 빌드 속도는 느리지만 실행 속도는 빠름"
    echo ""
    
    print_subheader "📦 의존성 관리"
    echo -e "  ${GREEN}cargo add <crate>${NC}"
    echo -e "    → 의존성 추가 (예: cargo add serde)"
    echo -e "    → npm install <package>와 유사"
    echo ""
    echo -e "  ${GREEN}cargo remove <crate>${NC}"
    echo -e "    → 의존성 제거"
    echo -e "    → npm uninstall <package>와 유사"
    echo ""
    echo -e "  ${GREEN}cargo update${NC}"
    echo -e "    → Cargo.lock의 의존성 버전 업데이트"
    echo ""
    
    print_subheader "🧪 테스트"
    echo -e "  ${GREEN}cargo test${NC}"
    echo -e "    → 모든 테스트 실행"
    echo ""
    echo -e "  ${GREEN}cargo test <test_name>${NC}"
    echo -e "    → 특정 테스트만 실행"
    echo ""
    
    print_subheader "⚡ 성능"
    echo -e "  ${GREEN}cargo check${NC}"
    echo -e "    → 컴파일 체크만 (빌드 X). 빠르게 오류 발견."
    echo -e "    → npm run lint와 유사한 빠른 피드백"
    echo ""
    echo -e "  ${GREEN}cargo clippy${NC}"
    echo -e "    → 정적 분석 및 코드 개선 제안"
    echo -e "    → eslint/prettier와 유사한 린터"
    echo ""
    
    print_subheader "📋 기타 유용한 명령어"
    echo -e "  ${GREEN}cargo doc --open${NC}"
    echo -e "    → 문서 생성 + 브라우저에서 열기"
    echo ""
    echo -e "  ${GREEN}cargo fmt${NC}"
    echo -e "    → 코드 포맷팅 (Rust 표준 포맷터)"
    echo -e "    → Prettier와 유사"
    echo ""
    echo -e "  ${GREEN}cargo tree${NC}"
    echo -e "    → 의존성 트리 출력"
    echo ""
    echo -e "  ${GREEN}  cargo clean${NC}"
    echo -e "    → target/ 디렉토리 전체 삭제 (빌드 캐시 초기화)"
    echo ""
}

check_rust_installed() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo가 설치되지 않았습니다.${NC}"
        echo -e "${YELLOW}  Rust 설치: https://rustup.rs/${NC}"
        echo -e "${YELLOW}  (Windows: winget install rustup || choco install rustup)${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Cargo 설치 확인: $(cargo --version)${NC}"
}

build_project() {
    local project_name="$1"
    local project_path="${SCRIPT_DIR}/${project_name}"
    
    print_subheader "${project_name} 빌드 및 실행"
    
    if [ ! -d "$project_path" ]; then
        echo -e "${RED}  ❌ 프로젝트 폴더가 없습니다: ${project_path}${NC}"
        return 1
    fi
    
    if [ ! -f "${project_path}/Cargo.toml" ]; then
        echo -e "${RED}  ❌ Cargo.toml이 없습니다: ${project_path}/Cargo.toml${NC}"
        return 1
    fi
    
    echo -e "  📂 ${project_path}"
    echo ""
    
    (cd "$project_path" && cargo build 2>&1)
    
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        echo -e "  ${GREEN}  ✅ 빌드 성공!${NC}"
        echo ""
        echo -e "  ${GREEN}  실행: cd ${project_name} && cargo run${NC}"
    else
        echo -e "  ${RED}  ❌ 빌드 실패 (exit code: ${exit_code})${NC}"
        return 1
    fi
    
    echo ""
}

run_all_projects() {
    print_header "🏗️ 프로젝트 빌드 & 실행"
    
    check_rust_installed
    
    echo -e "${YELLOW}현재 프로젝트:${NC}"
    echo "  ├── grpc-server/        # gRPC 서버 (tonic)"
    echo "  ├── rest-api-server/    # REST API 서버 (Axum)"
    echo "  └── websocket-server/   # WebSocket 서버 (tokio-tungstenite)"
    echo ""
    
    local all_success=true
    
    for project in grpc-server rest-api-server websocket-server; do
        if ! build_project "$project"; then
            all_success=false
        fi
    done
    
    if [ "$all_success" = true ]; then
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}  ✅ 모든 프로젝트 빌드 성공!${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo -e "각 프로젝트를 실행하려면:"
        echo -e "  cd grpc-server        && cargo run"
        echo -e "  cd rest-api-server    && cargo run"
        echo -e "  cd websocket-server   && cargo run"
    else
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${RED}  ❌ 일부 프로젝트 빌드에 실패했습니다.${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo -e "${YELLOW}빌드 오류가 있는 프로젝트에서 아래 명령을 실행하세요:${NC}"
        echo -e "  cd <프로젝트명> && cargo build"
        echo ""
        exit 1
    fi
}

run_single_project() {
    local project_name="$1"
    
    print_header "🏗️ ${project_name} 빌드 및 실행"
    
    check_rust_installed
    
    local project_path="${SCRIPT_DIR}/${project_name}"
    
    if [ ! -d "$project_path" ]; then
        echo -e "${RED}❌ 프로젝트 폴더가 없습니다: ${project_path}${NC}"
        exit 1
    fi
    
    if [ ! -f "${project_path}/Cargo.toml" ]; then
        echo -e "${RED}❌ Cargo.toml이 없습니다: ${project_path}/Cargo.toml${NC}"
        exit 1
    fi
    
    echo -e "  📂 ${project_path}"
    echo ""
    
    (cd "$project_path" && cargo build 2>&1)
    
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}  ✅ 빌드 성공!${NC}"
        echo ""
        echo -e "  실행: cd ${project_name} && cargo run"
    else
        echo -e "${RED}  ❌ 빌드 실패 (exit code: ${exit_code})${NC}"
        exit 1
    fi
}

build_only() {
    print_header "🔨 빌드만 실행 (실행 없이)"
    
    check_rust_installed
    
    for project in grpc-server rest-api-server websocket-server; do
        build_project "$project"
    done
    
    echo -e "${GREEN}✅ 모든 프로젝트 빌드 완료 (target/에 바이너리 생성)${NC}"
}

run_tests() {
    print_header "🧪 테스트 실행"
    
    check_rust_installed
    
    for project in grpc-server rest-api-server websocket-server; do
        local project_path="${SCRIPT_DIR}/${project}"
        
        if [ -d "$project_path" ] && [ -f "${project_path}/Cargo.toml" ]; then
            echo -e "  🧪 ${project} 테스트..."
            (cd "$project_path" && cargo test 2>&1)
            echo ""
        fi
    done
    
    echo -e "${GREEN}✅ 테스트 완료${NC}"
}

show_cargo_toml_example() {
    print_header "📝 Cargo.toml 예시"
    
    echo -e "${YELLOW}rest-api-server/Cargo.toml 예시 (Axum):${NC}"
    echo ""
    echo '    [package]'
    echo '    name = "rest-api-server"'
    echo '    version = "0.1.0"'
    echo '    edition = "2021"'
    echo ''
    echo '    [dependencies]'
    echo '    axum = "0.8"'
    echo '    tokio = { version = "1", features = ["full"] }'
    echo '    serde = { version = "1", features = ["derive"] }'
    echo '    serde_json = "1"'
    echo '    tower = "0.5"'
    echo '    tower-http = { version = "0.6", features = ["cors"] }'
    echo '    tracing = "0.1"'
    echo '    tracing-subscriber = "0.3"'
    echo ''
    
    echo -e "${YELLOW}grpc-server/Cargo.toml 예시 (tonic):${NC}"
    echo ""
    echo '    [package]'
    echo '    name = "grpc-server"'
    echo '    version = "0.1.0"'
    echo '    edition = "2021"'
    echo ''
    echo '    [dependencies]'
    echo '    tonic = "0.12"'
    echo '    tokio = { version = "1", features = ["full"] }'
    echo '    tokio-stream = "0.1"'
    echo '    prost = "0.13"'
    echo '    prost-types = "0.13"'
    echo ''
    
    echo -e "${YELLOW}websocket-server/Cargo.toml 예시 (tokio-tungstenite):${NC}"
    echo ""
    echo '    [package]'
    echo '    name = "websocket-server"'
    echo '    version = "0.1.0"'
    echo '    edition = "2021"'
    echo ''
    echo '    [dependencies]'
    echo '    tokio-tungstenite = "0.26"'
    echo '    tokio = { version = "1", features = ["full"] }'
    echo '    tokio-stream = "0.1"'
    echo '    futures-util = "0.3"'
    echo '    tracing = "0.1"'
    echo ''
}

main() {
    local action="all"
    local build_only=false
    local test_only=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                print_usage
                exit 0
                ;;
            --build)
                build_only=true
                shift
                ;;
            --test)
                test_only=true
                shift
                ;;
            grpc-server|grpc)
                action="grpc-server"
                shift
                ;;
            rest-api-server|rest-api|rest)
                action="rest-api-server"
                shift
                ;;
            websocket-server|websocket|ws)
                action="websocket-server"
                shift
                ;;
            *)
                echo -e "${RED}❌ Unknown argument: $1${NC}"
                echo ""
                print_usage
                exit 1
                ;;
        esac
    done
    
    show_basic_concepts
    show_commands
    show_cargo_toml_example
    
    echo ""
    print_header "🚀 실행"
    
    if [ "$test_only" = true ]; then
        run_tests
        exit 0
    fi
    
    if [ "$build_only" = true ]; then
        build_only
        exit 0
    fi
    
    if [ "$action" != "all" ]; then
        run_single_project "$action"
        exit 0
    fi
    
    run_all_projects
}

main "$@"
