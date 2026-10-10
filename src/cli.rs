use std::path::PathBuf;
use clap::Parser;

/// Skill Batch Runner (`sbr`)
/// Utilitas CLI untuk menjalankan instalasi batch skills ke direktori target secara deterministik.
#[derive(Parser, Debug, Clone)]
#[command(name = "sbr", version, about = "Batch runner for npx skills add", long_about = None)]
pub struct Cli {
    /// Jalur ke file .txt berisi daftar instruksi atau URL skill
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    pub file: PathBuf,

    /// Jalur direktori proyek tujuan instalasi
    #[arg(short = 't', long = "target", value_name = "DIR")]
    pub target: PathBuf,

    /// Memvalidasi dan menampilkan daftar tugas tanpa mengeksekusi subproses
    #[arg(short = 'd', long = "dry-run")]
    pub dry_run: bool,

    /// Batas proses konkuren (default: 1 untuk mencegah konflik berkas kunci)
    #[arg(short = 'c', long = "concurrency", default_value_t = 1, value_name = "NUM")]
    pub concurrency: usize,

    /// Tetap melanjutkan sisa entri jika salah satu instalasi gagal
    #[arg(long = "continue-on-error")]
    pub continue_on_error: bool,

    /// Menampilkan output stdout dan stderr mentah dari subproses npx
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_required_flags() {
        let args = vec!["sbr", "--file", "skills.txt", "--target", "./my-project"];
        let cli = Cli::try_parse_from(args).expect("Should parse required flags successfully");

        assert_eq!(cli.file, PathBuf::from("skills.txt"));
        assert_eq!(cli.target, PathBuf::from("./my-project"));
        assert!(!cli.dry_run);
        assert_eq!(cli.concurrency, 1);
        assert!(!cli.continue_on_error);
        assert!(!cli.verbose);
    }

    #[test]
    fn test_cli_parsing_all_flags() {
        let args = vec![
            "sbr",
            "-f",
            "skills.txt",
            "-t",
            "./my-project",
            "-d",
            "-c",
            "4",
            "--continue-on-error",
            "-v",
        ];
        let cli = Cli::try_parse_from(args).expect("Should parse all flags successfully");

        assert_eq!(cli.file, PathBuf::from("skills.txt"));
        assert_eq!(cli.target, PathBuf::from("./my-project"));
        assert!(cli.dry_run);
        assert_eq!(cli.concurrency, 4);
        assert!(cli.continue_on_error);
        assert!(cli.verbose);
    }

    #[test]
    fn test_cli_missing_required_flag() {
        let args = vec!["sbr", "--file", "skills.txt"];
        assert!(Cli::try_parse_from(args).is_err());
    }
}
