mod cli;
mod config;
mod parser;
mod reporter;
mod runner;

use clap::Parser;
use cli::Cli;
use console::style;
use runner::RunnerOptions;
use std::process;

fn main() {
    let cli = Cli::parse();

    // 1. Pre-flight Check
    if let Err(e) = config::preflight_check(&cli.file, &cli.target) {
        eprintln!("{} {}", style("[Pre-Flight Error]").red().bold(), e);
        process::exit(1);
    }

    // 2. Parsing input file
    let tasks = match parser::parse_skills_file(&cli.file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} {}", style("[Parser Error]").red().bold(), e);
            process::exit(1);
        }
    };

    println!(
        "{} Ditemukan {} tugas skill unik untuk dieksekusi di target: {}\n",
        style("[INFO]").cyan().bold(),
        style(tasks.len()).bold().green(),
        style(cli.target.display()).bold()
    );

    // 3. Eksekusi runner subproses
    let options = RunnerOptions {
        dry_run: cli.dry_run,
        continue_on_error: cli.continue_on_error,
        verbose: cli.verbose,
    };

    let results = runner::execute_batch(&tasks, &cli.target, &options);

    // 4. Pelaporan ringkasan
    reporter::print_summary(&results);

    // Exit code 1 jika ada yang gagal dan tidak diabaikan
    let has_failure = results
        .iter()
        .any(|r| matches!(r.status, runner::TaskStatus::Failed(_)));
    if has_failure {
        process::exit(1);
    }
}
