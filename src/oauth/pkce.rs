use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// PKCE-related errors
#[derive(Debug, Error)]
pub enum PKCEError {
    #[error("Failed to generate random bytes: {0}")]
    #[allow(dead_code)]
    RandomGeneration(String),

    #[error("Failed to encode challenge: {0}")]
    #[allow(dead_code)]
    Encoding(String),

    #[error("State mismatch")]
    StateMismatch,
}

/// PKCE Challenge containing verifier, challenge, and state
#[derive(Debug, Clone)]
pub struct PKCEChallenge {
    /// PKCE code verifier (43 characters, cryptographically random)
    pub verifier: String,

    /// PKCE code challenge (SHA-256 of verifier, Base64URL encoded)
    pub challenge: String,

    /// OAuth state parameter (32 bytes random, hex encoded)
    pub state: String,
}

impl PKCEChallenge {
    /// Generate a new PKCE challenge
    ///
    /// Creates:
    /// - verifier: 43 cryptographically random characters
    /// - challenge: SHA-256 hash of verifier, Base64URL encoded
    /// - state: 32 random bytes, hex encoded
    pub fn generate() -> Result<Self, PKCEError> {
        let verifier = generate_pkce_verifier()?;
        let challenge = generate_pkce_challenge(&verifier)?;
        let state = generate_state()?;

        Ok(PKCEChallenge {
            verifier,
            challenge,
            state,
        })
    }

    /// Verify that the provided state matches the stored state
    ///
    /// # Arguments
    /// * `state` - The state parameter received from OAuth callback
    ///
    /// # Returns
    /// `Ok(())` if states match, `Err(PKCEError::StateMismatch)` otherwise
    #[allow(dead_code)]
    pub fn verify_state(&self, state: &str) -> Result<(), PKCEError> {
        if self.state == state {
            Ok(())
        } else {
            Err(PKCEError::StateMismatch)
        }
    }
}

/// Generate a PKCE code verifier
///
/// Creates a 43-character cryptographically random string
/// using the unreserved characters: [A-Z] [a-z] [0-9] - . _ ~
///
/// Uses rejection sampling to ensure uniform distribution of characters,
/// avoiding the modulo bias that would otherwise occur.
///
/// # Returns
/// A 43-character random string suitable for PKCE verifier
pub fn generate_pkce_verifier() -> Result<String, PKCEError> {
    // PKCE verifier must be between 43 and 128 characters
    // We use 43 characters (minimum length for maximum compatibility)
    const VERIFIER_LENGTH: usize = 43;

    // Unreserved characters per RFC 7613
    const UNRESERVED_CHARS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let mut rng = rand::thread_rng();
    let mut verifier = String::with_capacity(VERIFIER_LENGTH);

    for _ in 0..VERIFIER_LENGTH {
        verifier.push(select_uniform_char(&mut rng, UNRESERVED_CHARS) as char);
    }

    Ok(verifier)
}

/// Select a uniformly random character from the given character set.
///
/// Uses rejection sampling to avoid modulo bias. When using modulo,
/// characters would not be uniformly distributed if the character set size
/// doesn't evenly divide 256.
///
/// For example, with 66 characters: 256 % 66 = 58, so characters 0-57 would
/// appear 4 times in the mapping (0, 66, 132, 198) while characters 58-65
/// would only appear 3 times, creating a ~33% bias.
///
/// Rejection sampling discards values that would cause bias and retries,
/// ensuring each character has equal probability of being selected.
fn select_uniform_char(rng: &mut impl RngCore, chars: &[u8]) -> u8 {
    // Calculate the threshold: largest multiple of chars.len() <= 256
    // Values >= threshold would cause bias if used with modulo
    let threshold = (u8::MAX as usize + 1) - (u8::MAX as usize + 1) % chars.len();

    loop {
        let b = rng.next_u32() as u8;
        // Accept only values below threshold to ensure uniform distribution
        if (b as usize) < threshold {
            return chars[(b as usize) % chars.len()];
        }
        // Reject values that would cause bias and retry
    }
}

/// Generate a PKCE code challenge from a verifier
///
/// Creates a SHA-256 hash of the verifier and encodes it using Base64URL
/// without padding
///
/// # Arguments
/// * `verifier` - The PKCE code verifier string
///
/// # Returns
/// Base64URL-encoded SHA-256 hash of the verifier
pub fn generate_pkce_challenge(verifier: &str) -> Result<String, PKCEError> {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();

    Ok(URL_SAFE_NO_PAD.encode(hash))
}

/// Generate a random state parameter
///
/// Creates a 32-byte cryptographically random value encoded as hexadecimal
///
/// # Returns
/// A 64-character hex-encoded string (32 bytes = 64 hex chars)
pub fn generate_state() -> Result<String, PKCEError> {
    const STATE_BYTES: usize = 32;

    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; STATE_BYTES];
    rng.fill_bytes(&mut bytes);

    Ok(hex::encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_verifier_length() {
        let verifier = generate_pkce_verifier().unwrap();
        assert!(
            verifier.len() >= 43,
            "Verifier should be at least 43 characters"
        );
        assert!(
            verifier.len() <= 128,
            "Verifier should be at most 128 characters"
        );
    }

    #[test]
    fn test_pkce_verifier_characters() {
        let verifier = generate_pkce_verifier().unwrap();
        // All characters should be from the unreserved set
        for ch in verifier.chars() {
            assert!(
                ch.is_ascii_alphanumeric() || ch == '-' || ch == '.' || ch == '_' || ch == '~',
                "Invalid character in verifier: {}",
                ch
            );
        }
    }

    #[test]
    fn test_pkce_challenge_deterministic() {
        let verifier = generate_pkce_verifier().unwrap();
        let challenge1 = generate_pkce_challenge(&verifier).unwrap();
        let challenge2 = generate_pkce_challenge(&verifier).unwrap();
        assert_eq!(
            challenge1, challenge2,
            "Same verifier should produce same challenge"
        );
    }

    #[test]
    fn test_pkce_challenge_different_verifiers() {
        let verifier1 = generate_pkce_verifier().unwrap();
        let verifier2 = generate_pkce_verifier().unwrap();
        let challenge1 = generate_pkce_challenge(&verifier1).unwrap();
        let challenge2 = generate_pkce_challenge(&verifier2).unwrap();
        assert_ne!(
            challenge1, challenge2,
            "Different verifiers should produce different challenges"
        );
    }

    #[test]
    fn test_state_length() {
        let state = generate_state().unwrap();
        // 32 bytes = 64 hex characters
        assert_eq!(
            state.len(),
            64,
            "State should be 64 hex characters (32 bytes)"
        );
    }

    #[test]
    fn test_state_hex_encoding() {
        let state = generate_state().unwrap();
        // Should be valid hex
        assert!(
            state.chars().all(|c| c.is_ascii_hexdigit()),
            "State should be hex encoded"
        );
    }

    #[test]
    fn test_state_uniqueness() {
        let state1 = generate_state().unwrap();
        let state2 = generate_state().unwrap();
        assert_ne!(state1, state2, "Each state should be unique");
    }

    #[test]
    fn test_pkce_challenge_generate() {
        let pkce = PKCEChallenge::generate().unwrap();

        // Verify lengths
        assert!(pkce.verifier.len() >= 43);
        assert!(!pkce.challenge.is_empty());
        assert_eq!(pkce.state.len(), 64);

        // Verify challenge matches verifier
        let expected_challenge = generate_pkce_challenge(&pkce.verifier).unwrap();
        assert_eq!(pkce.challenge, expected_challenge);
    }

    #[test]
    fn test_verify_state_success() {
        let pkce = PKCEChallenge::generate().unwrap();
        let result = pkce.verify_state(&pkce.state);
        assert!(
            result.is_ok(),
            "State verification should succeed with matching state"
        );
    }

    #[test]
    fn test_verify_state_failure() {
        let pkce = PKCEChallenge::generate().unwrap();
        let result = pkce.verify_state("invalid_state_12345");
        assert!(
            result.is_err(),
            "State verification should fail with non-matching state"
        );
        assert!(matches!(result.unwrap_err(), PKCEError::StateMismatch));
    }

    #[test]
    fn test_verifier_uniqueness() {
        let verifier1 = generate_pkce_verifier().unwrap();
        let verifier2 = generate_pkce_verifier().unwrap();
        assert_ne!(verifier1, verifier2, "Each verifier should be unique");
    }

    #[test]
    fn test_challenge_base64url_encoding() {
        let verifier = "test_verifier_1234567890_test_verifier_12345";
        let challenge = generate_pkce_challenge(verifier).unwrap();

        // Base64URL should not contain '+' or '/' and no padding
        assert!(!challenge.contains('+'), "Challenge should not contain '+'");
        assert!(!challenge.contains('/'), "Challenge should not contain '/'");
        assert!(
            !challenge.contains('='),
            "Challenge should not contain padding '='"
        );
    }
}
