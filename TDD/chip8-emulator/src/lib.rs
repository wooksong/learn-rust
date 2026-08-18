//! chip8-emulator — Rust + TDD learning track
//!
//! Mission 00: lib.rs(코어)와 main.rs(thin) 분할. 현재는 하니스 스텁.

/// 트랙 버전 식별자. Mission 00의 첫 테스트 대상.
pub fn chip8_version() -> &'static str {
    "chip8-emulator"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert_eq!(chip8_version(), "chip8-emulator");
    }
}
