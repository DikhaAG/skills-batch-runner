use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Biner `npx` tidak ditemukan pada $PATH sistem. Pastikan Node.js & npm terpasang.")]
    NpxNotFound,

    #[error("File input tidak ditemukan: {0}")]
    FileNotFound(PathBuf),

    #[error("File input bukan berkas reguler: {0}")]
    NotAFile(PathBuf),

    #[error("Direktori target tidak ditemukan: {0}")]
    TargetNotFound(PathBuf),

    #[error("Jalur target bukan direktori: {0}")]
    TargetNotADirectory(PathBuf),

    #[error("Tidak memiliki izin tulis pada direktori target: {0}")]
    TargetNotWritable(PathBuf),
}

/// Menjalankan validasi pra-eksekusi (pre-flight check) terhadap lingkungan dan argumen
pub fn preflight_check(file_path: &Path, target_dir: &Path) -> Result<(), ConfigError> {
    // 1. Cek biner npx di PATH
    which::which("npx").map_err(|_| ConfigError::NpxNotFound)?;

    // 2. Cek file input
    if !file_path.exists() {
        return Err(ConfigError::FileNotFound(file_path.to_path_buf()));
    }
    if !file_path.is_file() {
        return Err(ConfigError::NotAFile(file_path.to_path_buf()));
    }

    // 3. Cek direktori target
    if !target_dir.exists() {
        return Err(ConfigError::TargetNotFound(target_dir.to_path_buf()));
    }
    if !target_dir.is_dir() {
        return Err(ConfigError::TargetNotADirectory(target_dir.to_path_buf()));
    }

    // 4. Cek izin tulis direktori target
    validate_target_writable(target_dir)?;

    Ok(())
}

fn validate_target_writable(target_dir: &Path) -> Result<(), ConfigError> {
    // Uji kemampuan membuat file temporer di target dir
    let test_file = target_dir.join(".sbr_write_test.tmp");
    match fs::write(&test_file, b"test") {
        Ok(_) => {
            let _ = fs::remove_file(&test_file);
            Ok(())
        }
        Err(_) => Err(ConfigError::TargetNotWritable(target_dir.to_path_buf())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_preflight_valid_inputs() {
        let temp_dir = std::env::temp_dir().join("sbr_test_preflight_valid");
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test_skills.txt");
        File::create(&test_file).unwrap();

        let result = preflight_check(&test_file, &temp_dir);
        assert!(result.is_ok());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_preflight_missing_file() {
        let temp_dir = std::env::temp_dir().join("sbr_test_preflight_missing_file");
        fs::create_dir_all(&temp_dir).unwrap();

        let non_existent_file = temp_dir.join("ghost.txt");
        let result = preflight_check(&non_existent_file, &temp_dir);

        assert!(matches!(result, Err(ConfigError::FileNotFound(_))));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_preflight_missing_target() {
        let temp_dir = std::env::temp_dir().join("sbr_test_preflight_missing_target");
        let test_file = temp_dir.join("input.txt");
        fs::create_dir_all(&temp_dir).unwrap();
        File::create(&test_file).unwrap();

        let non_existent_target = temp_dir.join("no_such_target_dir");
        let result = preflight_check(&test_file, &non_existent_target);

        assert!(matches!(result, Err(ConfigError::TargetNotFound(_))));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
