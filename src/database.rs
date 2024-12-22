use rusqlite::{params, Connection, Result};
use crate::CreatePost;
use crate::Post;

const DB: &'static str = "db.sqlite3";

enum CreationError{
    Empty,
}

pub fn create_database_if_inexistant(){
    let conn = Connection::open(DB).unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT);
    ";
    conn.execute(query, ()).unwrap();
}

pub fn insert_post_in_database(post: CreatePost) -> Result<Post, rusqlite::Error>{
    let conn = Connection::open(DB).unwrap();
    conn.execute(
        "INSERT INTO posts (text) VALUES (?1)",
        (post.text.clone(),),
    )?;

    let last_id = conn.last_insert_rowid();
    Ok((Post{id: last_id as u64, text: post.text}))
}