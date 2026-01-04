//! Asset processing and hashing.
//!
//! Handles copying and fingerprinting of static assets.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha384};

/// An asset with its content hash for cache busting.
#[derive(Debug, Clone)]
pub struct HashedAsset {
    /// Original asset path
    pub source: PathBuf,

    /// Output path with hash in filename
    pub output: PathBuf,

    /// SHA-384 hash for SRI
    pub integrity: String,

    /// Short hash for filename (xxHash)
    pub short_hash: String,
}

/// Compute SHA-384 hash of content for SRI.
///
/// # Arguments
/// * `content` - The file content
///
/// # Returns
/// Base64-encoded hash prefixed with "sha384-"
pub fn compute_integrity(content: &[u8]) -> String {
    let mut hasher = Sha384::new();
    hasher.update(content);
    let hash = hasher.finalize();
    format!(
        "sha384-{}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash)
    )
}

/// Compute short hash for filename using xxHash.
///
/// # Arguments
/// * `content` - The file content
///
/// # Returns
/// 8-character hex hash
pub fn compute_short_hash(content: &[u8]) -> String {
    let hash = xxhash_rust::xxh3::xxh3_64(content);
    format!("{:016x}", hash)[..8].to_string()
}

/// Generate hashed asset path.
///
/// Transforms `style.css` to `style.abc12345.css`.
///
/// # Arguments
/// * `path` - Original file path
/// * `hash` - Short hash to insert
///
/// # Returns
/// Path with hash inserted before extension
pub fn hashed_path(path: &Path, hash: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().unwrap_or_default().to_string_lossy();

    let new_name = if ext.is_empty() {
        format!("{}.{}", stem, hash)
    } else {
        format!("{}.{}.{}", stem, hash, ext)
    };

    path.with_file_name(new_name)
}

/// Process an asset file, computing hashes and generating output path.
///
/// # Arguments
/// * `source` - Source file path
/// * `content` - File content
///
/// # Returns
/// HashedAsset with computed values
pub fn process_asset(source: &Path, content: &[u8]) -> HashedAsset {
    let integrity = compute_integrity(content);
    let short_hash = compute_short_hash(content);
    let output = hashed_path(source, &short_hash);

    HashedAsset {
        source: source.to_path_buf(),
        output,
        integrity,
        short_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashed_path() {
        let path = Path::new("assets/style.css");
        let result = hashed_path(path, "abc12345");
        assert_eq!(result, PathBuf::from("assets/style.abc12345.css"));
    }

    #[test]
    fn test_short_hash_length() {
        let content = b"test content";
        let hash = compute_short_hash(content);
        assert_eq!(hash.len(), 8);
    }

    #[test]
    fn test_integrity_format() {
        let content = b"test content";
        let integrity = compute_integrity(content);
        assert!(integrity.starts_with("sha384-"));
    }
}
