# 质量门（本机代替 CI）：tiny_skia 版本锁 + fmt + clippy + test
# 用法：CARGO_HOME=... ./scripts/check.sh
set -e
cd "$(dirname "$0")/.."

# iced_tiny_skia 0.14.0 有多画布裁剪缺陷（#3452），必须 ≥ 0.14.1（计划书 2.1 / R11）
grep -A1 'name = "iced_tiny_skia"' Cargo.lock | grep -q 'version = "0.14.1"' || {
    echo "ERROR: Cargo.lock 中 iced_tiny_skia 不是 0.14.1（多画布缺陷 #3452，须 >= 0.14.1）" >&2
    exit 1
}

cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
