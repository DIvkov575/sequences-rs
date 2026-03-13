use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: u64,
    pub score: u32,
    pub correct: u32,
    pub total: u32,
    pub difficulty: String,
    pub time_limit_secs: u64,
    pub duration_secs: u64,
}

fn history_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".sequences-rs").join("history.csv")
}

pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn format_ago(ts: u64) -> String {
    let now = now_secs();
    let secs_ago = now.saturating_sub(ts);
    if secs_ago < 60 { format!("{}s ago", secs_ago) }
    else if secs_ago < 3600 { format!("{}m ago", secs_ago / 60) }
    else if secs_ago < 86400 { format!("{}h ago", secs_ago / 3600) }
    else { format!("{}d ago", secs_ago / 86400) }
}

pub fn load() -> Vec<HistoryEntry> {
    let path = history_path();
    let file = match fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let reader = io::BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.splitn(7, ',').collect();
        if parts.len() != 7 { continue; }
        let Ok(timestamp)       = parts[0].parse::<u64>() else { continue };
        let Ok(score)           = parts[1].parse::<u32>() else { continue };
        let Ok(correct)         = parts[2].parse::<u32>() else { continue };
        let Ok(total)           = parts[3].parse::<u32>() else { continue };
        let Ok(time_limit_secs) = parts[5].parse::<u64>() else { continue };
        let Ok(duration_secs)   = parts[6].parse::<u64>() else { continue };
        entries.push(HistoryEntry {
            timestamp,
            score,
            correct,
            total,
            difficulty: parts[4].to_string(),
            time_limit_secs,
            duration_secs,
        });
    }
    // most recent first
    entries.reverse();
    entries
}

pub fn append(entry: &HistoryEntry) -> io::Result<()> {
    let path = history_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(
        file,
        "{},{},{},{},{},{},{}",
        entry.timestamp, entry.score, entry.correct, entry.total,
        entry.difficulty, entry.time_limit_secs, entry.duration_secs
    )
}
