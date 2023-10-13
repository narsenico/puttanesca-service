use crate::{models::Match, hunters::Hunter};
use crate::Result;
use async_trait::async_trait;
use rusqlite::{Connection, OpenFlags, OptionalExtension};

use super::Processor;

const VERSION: usize = 1;

pub struct SqliteProcessor {
    db_name: String,
}

impl SqliteProcessor {
    pub fn new(db_name: String) -> Result<Self> {
        // TODO: check valid file name
        Ok(Self { db_name })
    }
}

#[async_trait]
impl Processor for SqliteProcessor {
    fn name(&self) -> String {
        String::from("Sqlite Processor")
    }

    async fn process(&self, hunter: std::sync::Arc<dyn Hunter>) -> Result<()> {
        let conn = create_conn(&self.db_name, true)?;
        check_valid_connection(&conn)?;

        if let Some(current_version) = get_version(&conn)? {
            exec_migration(&conn, current_version)?;
        } else {
            prepare_db(&conn)?;
        }

        let matches = hunter.find_matches().await?;
        for m in matches {
            upsert_match(&conn, &m)?;
        }

        Ok(())
    }
}

fn create_conn(path: &str, create: bool) -> Result<Connection> {
    let flags = if create {
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_CREATE
    } else {
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX
    };

    let conn = Connection::open_with_flags(path, flags)?;
    Ok(conn)
}

/// sqlite opens databases lazily, so [`Connection::open_with_flags`] does not produce any error with invalid database file.
/// Executing a pragma function forces the database to open and check if it is valid.
fn check_valid_connection(conn: &Connection) -> Result<()> {
    conn.pragma_query(None, "schema_version", |_| Ok(()))?;
    Ok(())
}

fn prepare_db(conn: &Connection) -> Result<()> {
    let sql_tversion_table = r#"
        CREATE TABLE IF NOT EXISTS tVersion (
          version INTEGER NOT NULL
        )
    "#;
    conn.execute(sql_tversion_table, ())?;

    let sql_tversion_insert = r#"
        INSERT INTO tVersion (version) VALUES (?)
    "#;
    conn.execute(sql_tversion_insert, [VERSION])?;

    let sql_tmatches_table = r#"
        CREATE TABLE IF NOT EXISTS tMatches (
          team1 TEXT NOT NULL,
          team2 TEXT NOT NULL,
          match_day INTEGER NOT NULL,
          match_date TEXT NOT NULL,
          team1_score INTEGER NULL,
          team2_score INTEGER NULL,
          PRIMARY KEY(team1, team2)
        )    
    "#;
    conn.execute(sql_tmatches_table, ())?;

    Ok(())
}

fn get_version(conn: &Connection) -> Result<Option<usize>> {
    let sql_tversion_exists =
        r#"SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type="table" AND name="tVersion")"#;
    let missing_table = conn.query_row(sql_tversion_exists, [], |row| row.get::<_, usize>(0))? == 0;
    if missing_table {
        return Ok(None);
    }

    let res = conn
        .query_row("SELECT version FROM tVersion", (), |row| {
            row.get::<_, usize>(0)
        })
        .optional()?;

    Ok(res)
}

fn exec_migration(_conn: &Connection, _current_version: usize) -> Result<()> {
    // exec migration of database from current_version to VERSION
    Ok(())
}

fn upsert_match(conn: &Connection, match_: &Match) -> Result<()> {
    let sql_match_insert = r#"
        INSERT OR REPLACE INTO tMatches (match_day,match_date,team1,team2,team1_score,team2_score) VALUES (?,?,?,?,?,?);
    "#;
    conn.execute(
        sql_match_insert,
        (
            match_.match_day,
            &match_.date,
            &match_.team1,
            &match_.team2,
            match_.team1_score,
            match_.team2_score,
        ),
    )?;

    Ok(())
}
