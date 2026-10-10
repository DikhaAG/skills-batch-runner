use indexmap::IndexSet;
use std::fs;
use std::path::Path;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTask {
    pub repo_url: String,
    pub skill_name: String,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Gagal membaca file masukan: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format baris tidak valid pada baris {line_no}: \"{line}\"")]
    InvalidLineFormat { line_no: usize, line: String },

    #[error("URL repositori tidak valid pada baris {line_no}: \"{url}\" ({reason})")]
    InvalidUrl {
        line_no: usize,
        url: String,
        reason: String,
    },

    #[error("File masukan kosong atau tidak memuat instruksi skill yang valid")]
    EmptyTaskList,
}

/// Membaca file dan mem-parse menjadi daftar tugas SkillTask yang terdeduplikasi (mempertahankan urutan kemunculan)
pub fn parse_skills_file(path: &Path) -> Result<Vec<SkillTask>, ParserError> {
    let content = fs::read_to_string(path)?;
    parse_skills_content(&content)
}

pub fn parse_skills_content(content: &str) -> Result<Vec<SkillTask>, ParserError> {
    let mut tasks = IndexSet::new();

    for (index, raw_line) in content.lines().enumerate() {
        let line_no = index + 1;
        let line = raw_line.trim();

        // Abaikan baris kosong dan komentar
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let task = parse_line(line, line_no)?;
        tasks.insert(task);
    }

    if tasks.is_empty() {
        return Err(ParserError::EmptyTaskList);
    }

    Ok(tasks.into_iter().collect())
}

fn parse_line(line: &str, line_no: usize) -> Result<SkillTask, ParserError> {
    // Normalisasi whitespace ganda
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // Skenario 1: Format mentah CLI
    // Contoh: npx skills add https://github.com/org/repo --skill cargo-workspace
    if tokens.first().copied() == Some("npx") {
        if tokens.len() >= 6 && tokens[1] == "skills" && tokens[2] == "add" {
            let repo_url = tokens[3];
            // Cari posisi bendera --skill pada baris utuh untuk menjaga argumen berkutip
            if let Some(pos) = line.find("--skill") {
                let after_flag = line[pos + "--skill".len()..].trim();
                let skill_name = if (after_flag.starts_with('"') && after_flag.ends_with('"'))
                    || (after_flag.starts_with('\'') && after_flag.ends_with('\''))
                {
                    if after_flag.len() >= 2 {
                        &after_flag[1..after_flag.len() - 1]
                    } else {
                        after_flag
                    }
                } else {
                    after_flag.split_whitespace().next().unwrap_or(after_flag)
                };

                if !skill_name.is_empty() {
                    validate_url(repo_url, line_no)?;
                    return Ok(SkillTask {
                        repo_url: repo_url.to_string(),
                        skill_name: skill_name.to_string(),
                    });
                }
            }
        }
        return Err(ParserError::InvalidLineFormat {
            line_no,
            line: line.to_string(),
        });
    }

    // Skenario 2: Format pasangan Repo dan Skill Name
    // Contoh: https://github.com/org/repo cargo-workspace
    if tokens.len() == 2 {
        let repo_url = tokens[0];
        let skill_name = tokens[1];
        validate_url(repo_url, line_no)?;
        return Ok(SkillTask {
            repo_url: repo_url.to_string(),
            skill_name: skill_name.to_string(),
        });
    }

    Err(ParserError::InvalidLineFormat {
        line_no,
        line: line.to_string(),
    })
}

fn validate_url(raw_url: &str, line_no: usize) -> Result<(), ParserError> {
    match Url::parse(raw_url) {
        Ok(parsed) => {
            if parsed.scheme() != "http" && parsed.scheme() != "https" {
                return Err(ParserError::InvalidUrl {
                    line_no,
                    url: raw_url.to_string(),
                    reason: "Scheme harus http atau https".to_string(),
                });
            }
            if parsed.host_str().is_none() {
                return Err(ParserError::InvalidUrl {
                    line_no,
                    url: raw_url.to_string(),
                    reason: "Host URL tidak valid".to_string(),
                });
            }
            Ok(())
        }
        Err(e) => Err(ParserError::InvalidUrl {
            line_no,
            url: raw_url.to_string(),
            reason: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mixed_formats_and_deduplication() {
        let input = r#"
        # Komentar pembuka
        npx skills add https://github.com/getcargohq/cargo-skills --skill cargo-workspace-management
        
        # Duplikat dengan format pasangan
        https://github.com/getcargohq/cargo-skills cargo-workspace-management

        # Skill baru
        https://github.com/svssdeva/agentic-skills cargo-workspace-scaffold
        "#;

        let tasks = parse_skills_content(input).expect("Harus sukses mem-parse");
        assert_eq!(tasks.len(), 2, "Harus terdeduplikasi menjadi 2 tugas unik");

        assert_eq!(
            tasks[0],
            SkillTask {
                repo_url: "https://github.com/getcargohq/cargo-skills".to_string(),
                skill_name: "cargo-workspace-management".to_string(),
            }
        );
        assert_eq!(
            tasks[1],
            SkillTask {
                repo_url: "https://github.com/svssdeva/agentic-skills".to_string(),
                skill_name: "cargo-workspace-scaffold".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_invalid_url() {
        let input = "not-a-valid-url my-skill";
        let res = parse_skills_content(input);
        assert!(matches!(res, Err(ParserError::InvalidUrl { .. })));
    }

    #[test]
    fn test_parse_empty_content() {
        let input = "# Hanya komentar\n\n   \n";
        let res = parse_skills_content(input);
        assert!(matches!(res, Err(ParserError::EmptyTaskList)));
    }
}
