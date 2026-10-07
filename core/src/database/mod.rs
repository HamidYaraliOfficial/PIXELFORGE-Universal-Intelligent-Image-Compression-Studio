use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS schema_migrations(version INTEGER PRIMARY KEY);
            CREATE TABLE IF NOT EXISTS jobs(
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                started_at INTEGER,
                finished_at INTEGER,
                error TEXT
            );
            CREATE TABLE IF NOT EXISTS files(
                id TEXT PRIMARY KEY,
                job_id TEXT,
                input_path TEXT NOT NULL,
                output_path TEXT,
                original_size INTEGER NOT NULL,
                final_size INTEGER,
                sha256 TEXT,
                status TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS analysis_results(
                file_id TEXT PRIMARY KEY,
                json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS compression_runs(
                id TEXT PRIMARY KEY,
                file_id TEXT,
                candidate_json TEXT NOT NULL,
                metrics_json TEXT NOT NULL,
                elapsed_ms INTEGER NOT NULL,
                selected INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS candidates(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_id TEXT,
                candidate_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS presets(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS rules(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                expression TEXT NOT NULL,
                action TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS settings(
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS history(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT,
                created_at INTEGER NOT NULL,
                snapshot_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS plugins(
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                path TEXT NOT NULL,
                enabled INTEGER NOT NULL
            );
        "#)?;
        Ok(())
    }

    pub fn create_job(&self) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO jobs(id,status,created_at) VALUES(?1,'queued',strftime('%s','now'))",
            params![id],
        )?;
        Ok(id)
    }

    pub fn store_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn backup(&self, destination: &Path) -> Result<()> {
        let out = Connection::open(destination)?;
        self.conn.backup(rusqlite::DatabaseName::Main, &out, None)?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct DatabaseStatus {
    pub path: String,
    pub wal: bool,
}
