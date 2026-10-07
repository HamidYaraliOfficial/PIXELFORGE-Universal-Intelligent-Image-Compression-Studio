use anyhow::{anyhow, Result};
use chrono::{Datelike, Local, NaiveTime, Timelike, Duration};
use serde::{Deserialize, Serialize};
use crate::SchedulerConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEstimate {
    pub now: String,
    pub inside_window: bool,
    pub seconds_to_window_end: i64,
    pub next_window_start: String,
    pub seconds_to_next_window: i64,
    pub estimated_job_seconds: u64,
}

pub fn validate(config: &SchedulerConfig) -> Result<()> {
    for w in &config.windows {
        NaiveTime::parse_from_str(&w.start, "%H:%M").map_err(|_| anyhow!("invalid start time {}", w.start))?;
        NaiveTime::parse_from_str(&w.end, "%H:%M").map_err(|_| anyhow!("invalid end time {}", w.end))?;
        if w.weekday > 6 { return Err(anyhow!("weekday must be 0..6")); }
    }
    Ok(())
}

pub fn estimate(config: &SchedulerConfig, estimated_job_seconds: u64) -> Result<ScheduleEstimate> {
    validate(config)?;
    let now = Local::now();
    let weekday = now.weekday().num_days_from_monday() as u8;
    let nt = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).ok_or_else(|| anyhow!("invalid local time"))?;
    for w in &config.windows {
        if w.weekday != weekday { continue; }
        let start = NaiveTime::parse_from_str(&w.start, "%H:%M")?;
        let end = NaiveTime::parse_from_str(&w.end, "%H:%M")?;
        if nt >= start && nt <= end {
            let secs = (end - nt).num_seconds().max(0);
            return Ok(ScheduleEstimate {
                now: now.to_rfc3339(),
                inside_window: true,
                seconds_to_window_end: secs,
                next_window_start: format!("today {}", w.start),
                seconds_to_next_window: 0,
                estimated_job_seconds,
            });
        }
    }
    for offset in 0..8 {
        let day = (weekday + offset) % 7;
        for w in &config.windows {
            if w.weekday != day { continue; }
            let start = NaiveTime::parse_from_str(&w.start, "%H:%M")?;
            let delta = if offset == 0 && start > nt {
                (start - nt).num_seconds()
            } else if offset > 0 {
                (Duration::days(offset as i64) + (start - nt)).num_seconds()
            } else { continue };
            return Ok(ScheduleEstimate {
                now: now.to_rfc3339(),
                inside_window: false,
                seconds_to_window_end: 0,
                next_window_start: format!("weekday {} {}", day, w.start),
                seconds_to_next_window: delta.max(0),
                estimated_job_seconds,
            });
        }
    }
    Err(anyhow!("no schedule windows configured"))
}
