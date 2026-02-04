//! Zero-copy scanner functions using byte slicing

use crate::constants::MAX_COMMENT_LENGTH;

/// Maximum comment length to prevent denial-of-service from malicious input
/// Comments longer than this will cause a parse error instead of being processed
/// Note: This constant is defined in src/constants.rs as MAX_COMMENT_LENGTH

/// Skip ASCII whitespace using byte slice (no allocation)
#[inline]
pub fn skip_whitespace_bytes(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Skip whitespace and block comments using byte slice (no allocation)
///
/// This function includes protection against malicious input that attempts
/// to cause denial-of-service through malformed or unterminated comments.
pub fn skip_whitespace_and_comments_bytes(bytes: &[u8], mut pos: usize) -> usize {
    loop {
        pos = skip_whitespace_bytes(bytes, pos);
        // Check for block comment start (/*)
        if pos + 1 < bytes.len() && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            // Skip block comment
            pos += 2;
            let comment_start = pos;

            // Look for comment end (*/), with protection against malformed comments
            while pos + 1 < bytes.len() {
                // Check for maliciously long comments that could cause DoS
                if pos - comment_start > MAX_COMMENT_LENGTH {
                    // Return an error position that the parser can handle
                    // We set pos to bytes.len() to signal an error condition
                    // The parser will detect this and return an appropriate error
                    return bytes.len(); // Signal error condition
                }

                if bytes[pos] == b'*' && bytes[pos + 1] == b'/' {
                    pos += 2; // Skip the closing */
                    break;
                }
                pos += 1;
            }

            // If we reached the end without finding a closing */, that's an error
            // But we don't panic - we return the position and let the parser handle it
            if pos >= bytes.len() || pos + 1 >= bytes.len() {
                // Unterminated comment - return current position
                // The parser will detect this as unexpected end of input
                return pos;
            }
        } else {
            break;
        }
    }
    pos
}
