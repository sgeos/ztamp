//! rztamp - Portable Rust library for TANF job search automation.
//!
//! This crate contains shared logic used by both the NIF bindings (called from
//! Elixir) and standalone CLI tools. It is designed to be `no_std` compatible
//! where feasible, enabling use in constrained environments.

/// Verify the library is accessible.
///
/// Returns `true`. Used by downstream crates to confirm the dependency link.
pub fn alive() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_is_alive() {
        assert!(alive());
    }
}
