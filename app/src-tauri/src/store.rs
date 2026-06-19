//! Local SQLite persistence for sessions and settings.
//!
//! The store is the device-local source of truth (the "local-first" core: it
//! works fully offline). The schema is deliberately small for the v1 slice but
//! shaped so a future sync engine can adopt it: stable text ids and timestamps,
//! no auto-increment surrogate keys on synced rows.

use rusqlite::Connection;
use serde::Serialize;

/// A persisted agent session record.
#[derive(Debug, Clone, Serialize)]
pub struct SessionRecord {
    pub id: String,
    pub agent: String,
    pub cwd: String,
    pub title: String,
    pub created_at: i64,
}

/// Open (creating if needed) the database at `path` and ensure the schema.
pub fn open(path: &std::path::Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    init_schema(&conn)?;
    Ok(conn)
}

/// Open an in-memory database (used by tests).
#[cfg(test)]
pub fn open_in_memory() -> rusqlite::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    init_schema(&conn)?;
    Ok(conn)
}

fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sessions (
            id         TEXT PRIMARY KEY,
            agent      TEXT NOT NULL,
            cwd        TEXT NOT NULL,
            title      TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )
}

/// Insert or replace a session record.
pub fn upsert_session(conn: &Connection, rec: &SessionRecord) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO sessions (id, agent, cwd, title, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            agent = excluded.agent,
            cwd = excluded.cwd,
            title = excluded.title",
        rusqlite::params![rec.id, rec.agent, rec.cwd, rec.title, rec.created_at],
    )?;
    Ok(())
}

/// List sessions, most recent first.
pub fn list_sessions(conn: &Connection) -> rusqlite::Result<Vec<SessionRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, agent, cwd, title, created_at FROM sessions ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SessionRecord {
            id: row.get(0)?,
            agent: row.get(1)?,
            cwd: row.get(2)?,
            title: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_and_list_round_trip() {
        let conn = open_in_memory().unwrap();
        upsert_session(
            &conn,
            &SessionRecord {
                id: "s1".into(),
                agent: "claude_code".into(),
                cwd: "/tmp/proj".into(),
                title: "First".into(),
                created_at: 100,
            },
        )
        .unwrap();
        upsert_session(
            &conn,
            &SessionRecord {
                id: "s2".into(),
                agent: "claude_code".into(),
                cwd: "/tmp/proj2".into(),
                title: "Second".into(),
                created_at: 200,
            },
        )
        .unwrap();

        let all = list_sessions(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].id, "s2"); // most recent first
        assert_eq!(all[1].title, "First");
    }

    #[test]
    fn upsert_is_idempotent_on_id() {
        let conn = open_in_memory().unwrap();
        let mut rec = SessionRecord {
            id: "s1".into(),
            agent: "claude_code".into(),
            cwd: "/a".into(),
            title: "Old".into(),
            created_at: 1,
        };
        upsert_session(&conn, &rec).unwrap();
        rec.title = "New".into();
        upsert_session(&conn, &rec).unwrap();
        let all = list_sessions(&conn).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "New");
    }
}
