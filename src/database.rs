use rusqlite::{params, Connection, Result};
use crate::CreatePost;

const DB: &'static str = "db.sqlite3";

pub fn create_database_if_inexistant(){
    let conn = Connection::open(DB).unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT);
    ";
    conn.execute(query, ()).unwrap();
}

pub fn insert_post_in_database(post: CreatePost) -> Result<()>{
    let conn = Connection::open(DB).unwrap();
    conn.execute(
        "INSERT INTO posts (text) VALUES (?1)",
        (post.text,),
    )?;
    Ok(())
}