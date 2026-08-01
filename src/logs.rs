use crate::config::GallopConfig;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String, // "INFO", "WARN", "ERROR", "DEBUG"
    pub message: String,
    pub source: String,
}

pub struct LogManager {
    log_file: PathBuf,
}

impl LogManager {
    pub fn new() -> Self {
        let log_file = GallopConfig::log_file();
        Self { log_file }
    }

    #[allow(dead_code)]
    pub fn read_entries(&self, filter_level: &str, search_query: &str) -> Vec<LogEntry> {
        let mut entries = Vec::new();

        if !self.log_file.exists() {
            entries.push(LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: "INFO".to_string(),
                message: "Gallop log system initialized.".to_string(),
                source: "gallop::main".to_string(),
            });
            return entries;
        }

        if let Ok(file) = fs::File::open(&self.log_file) {
            let reader = BufReader::new(file);
            for line_res in reader.lines() {
                if let Ok(line) = line_res {
                    if let Some(entry) = parse_log_line(&line) {
                        if matches_filter(&entry, filter_level, search_query) {
                            entries.push(entry);
                        }
                    }
                }
            }
        }

        if entries.is_empty() {
            entries.push(LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: "INFO".to_string(),
                message: "Gallop session active. No log entries matched current filter."
                    .to_string(),
                source: "gallop::logging".to_string(),
            });
        }

        entries
    }

    pub fn clear(&self) -> Result<(), String> {
        if self.log_file.exists() {
            fs::write(&self.log_file, "").map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn export(&self, export_path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = export_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if self.log_file.exists() {
            fs::copy(&self.log_file, export_path).map_err(|e| e.to_string())?;
        } else {
            fs::write(export_path, "Gallop Log Export\nSession initialized.\n")
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let timestamp = if parts[0].contains('T') {
            parts[0]
                .split('T')
                .nth(1)
                .unwrap_or(parts[0])
                .split('.')
                .next()
                .unwrap_or(parts[0])
                .to_string()
        } else {
            parts[0].to_string()
        };
        let level = parts[1].to_string();
        let rest = parts[2..].join(" ");
        let (source, message) = if let Some(idx) = rest.find(": ") {
            (rest[..idx].to_string(), rest[idx + 2..].trim().to_string())
        } else {
            ("gallop".to_string(), rest)
        };

        return Some(LogEntry {
            timestamp,
            level,
            message,
            source,
        });
    }

    None
}

#[allow(dead_code)]
fn matches_filter(entry: &LogEntry, filter_level: &str, search_query: &str) -> bool {
    let level_match = match filter_level.to_lowercase().as_str() {
        "all" | "" => true,
        "info" => entry.level.to_lowercase() == "info",
        "warning" | "warn" => {
            entry.level.to_lowercase() == "warn" || entry.level.to_lowercase() == "warning"
        }
        "error" => entry.level.to_lowercase() == "error" || entry.level.to_lowercase() == "err",
        "debug" => entry.level.to_lowercase() == "debug",
        _ => true,
    };

    let query_match = if search_query.is_empty() {
        true
    } else {
        let q = search_query.to_lowercase();
        entry.message.to_lowercase().contains(&q) || entry.source.to_lowercase().contains(&q)
    };

    level_match && query_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_line() {
        let line = "2026-08-01T13:31:33.483705Z  INFO gallop::app: System info populated";
        let entry = parse_log_line(line);
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.level, "INFO");
        assert_eq!(e.source, "gallop::app");
        assert_eq!(e.message, "System info populated");
    }

    #[test]
    fn test_log_manager_read() {
        let mgr = LogManager::new();
        let entries = mgr.read_entries("all", "");
        assert!(!entries.is_empty());
    }
}
