use lazy_static::lazy_static;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref DB: Arc<Mutex<Connection>> = {
        let conn = Connection::open_in_memory().expect("Fail to open Sqlite connection!");

        conn.execute(
            "CREATE TABLE songs (
                      id              INTEGER PRIMARY KEY,
                      path            TEXT NOT NULL,
                      title           TEXT NOT NULL,
                      artist          TEXT,
                      album_id        INTEGER
                      )",
            params![],
        )
        .expect("fail to create table!");
        conn.execute(
            "CREATE TABLE albums (
                      id              INTEGER PRIMARY KEY,
                      title           TEXT NOT NULL,
                      artist          TEXT,
                      cover           BLOB
                      )",
            params![],
        )
        .expect("fail to create table!");
        Arc::new(Mutex::new(conn))
    };
}
