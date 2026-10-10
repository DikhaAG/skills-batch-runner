use crate::parser::SkillTask;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Success,
    Failed(String),
    Skipped(String),
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task: SkillTask,
    pub status: TaskStatus,
}

pub struct RunnerOptions {
    pub dry_run: bool,
    pub continue_on_error: bool,
    pub verbose: bool,
}

/// Menjalankan batch eksekusi SkillTask secara sekuensial dengan isolasi working directory
pub fn execute_batch(
    tasks: &[SkillTask],
    target_dir: &Path,
    options: &RunnerOptions,
) -> Vec<TaskResult> {
    let mut results = Vec::with_capacity(tasks.len());
    let total = tasks.len();

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );

    let mut aborted = false;

    for task in tasks {
        if aborted {
            results.push(TaskResult {
                task: task.clone(),
                status: TaskStatus::Skipped("Dilewati karena tugas sebelumnya gagal".to_string()),
            });
            pb.inc(1);
            continue;
        }

        pb.set_message(format!("{}/{}", task.skill_name, task.repo_url));

        if options.dry_run {
            println!(
                "[DRY-RUN] cd {} && npx skills add {} --skill {} -y",
                target_dir.display(),
                task.repo_url,
                task.skill_name
            );
            results.push(TaskResult {
                task: task.clone(),
                status: TaskStatus::Success,
            });
            pb.inc(1);
            continue;
        }

        let run_result = run_single_task(task, target_dir, options.verbose);
        match run_result {
            Ok(_) => {
                results.push(TaskResult {
                    task: task.clone(),
                    status: TaskStatus::Success,
                });
            }
            Err(err_msg) => {
                results.push(TaskResult {
                    task: task.clone(),
                    status: TaskStatus::Failed(err_msg),
                });

                if !options.continue_on_error {
                    aborted = true;
                }
            }
        }

        pb.inc(1);
    }

    pb.finish_and_clear();
    results
}

fn run_single_task(task: &SkillTask, target_dir: &Path, verbose: bool) -> Result<(), String> {
    let mut cmd = Command::new("npx");
    cmd.current_dir(target_dir)
        .args([
            "skills",
            "add",
            &task.repo_url,
            "--skill",
            &task.skill_name,
            "-y",
        ]);

    if verbose {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    }

    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => return Err(format!("Gagal meluncurkan subproses npx: {}", e)),
    };

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let details = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            // Ambil baris-baris pesan error penting dari stdout npx
            let error_lines: Vec<&str> = stdout
                .lines()
                .filter(|line| {
                    let l = line.trim();
                    l.contains("No valid skills")
                        || l.contains("Error")
                        || l.contains("error")
                        || l.starts_with('└')
                        || l.contains("not found")
                })
                .collect();

            if !error_lines.is_empty() {
                error_lines.join(" | ")
            } else {
                stdout.lines().last().unwrap_or(&stdout).trim().to_string()
            }
        } else {
            format!("Subproses keluar dengan kode: {:?}", output.status.code())
        };

        Err(details)
    }
}
