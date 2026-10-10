use crate::runner::{TaskResult, TaskStatus};
use console::style;

pub fn print_summary(results: &[TaskResult]) {
    let total = results.len();
    let mut success_count = 0;
    let mut failed_count = 0;
    let mut skipped_count = 0;

    println!("\n{}", style("=== RINGKASAN EKSEKUSI BATCH SKILLS ===").bold().cyan());

    for (idx, res) in results.iter().enumerate() {
        let num = idx + 1;
        match &res.status {
            TaskStatus::Success => {
                success_count += 1;
                println!(
                    " {:>2}. [{}] {} ({})",
                    num,
                    style("SUKSES").green().bold(),
                    style(&res.task.skill_name).bold(),
                    res.task.repo_url
                );
            }
            TaskStatus::Failed(err) => {
                failed_count += 1;
                println!(
                    " {:>2}. [{}] {} ({})\n     └── {}",
                    num,
                    style("GAGAL").red().bold(),
                    style(&res.task.skill_name).bold(),
                    res.task.repo_url,
                    style(err).red()
                );
            }
            TaskStatus::Skipped(reason) => {
                skipped_count += 1;
                println!(
                    " {:>2}. [{}] {} ({})\n     └── {}",
                    num,
                    style("DILEWATI").yellow().bold(),
                    style(&res.task.skill_name).dim(),
                    res.task.repo_url,
                    style(reason).dim()
                );
            }
        }
    }

    println!("{}", style("----------------------------------------").dim());
    println!(
        "Total: {} | Sukses: {} | Gagal: {} | Dilewati: {}",
        style(total).bold(),
        style(success_count).green().bold(),
        style(failed_count).red().bold(),
        style(skipped_count).yellow().bold()
    );
    println!("{}\n", style("========================================").dim());
}
