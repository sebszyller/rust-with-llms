use std::sync::Arc;

use axum::{
    routing::{get, post},
    Json, Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u8,
}

enum DbOperation {
    INSERT(String, u8),
    SELECT(tokio::sync::oneshot::Sender<Vec<User>>),
}
#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);

    let sender_shared = Arc::new(Mutex::new(sender));

    tokio::spawn(async move {
        let conn = match Connection::open("rustllms.db") {
            Ok(conn) => conn,
            Err(err) => panic!["tf is this error {err}"],
        };
        db_task(conn, receiver).await;
    });

    let app = Router::new()
        .route("/users", get(list_users))
        .route("/users", post(insert_user))
        .layer(axum::Extension(sender_shared));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn db_task(conn: Connection, mut receiver: mpsc::Receiver<DbOperation>) {
    while let Some(op) = receiver.recv().await {
        match op {
            DbOperation::INSERT(name, age) => {
                conn.execute(
                    "INSERT INTO user (name, age) VALUES (?1, ?2)",
                    params![name, age],
                )
                .unwrap();
            }
            DbOperation::SELECT(respond_to) => {
                let mut stmt = conn.prepare("SELECT name, age FROM user").unwrap();
                let users_iter = stmt
                    .query_map(params![], |row| {
                        Ok(User {
                            name: row.get(0)?,
                            age: row.get(1)?,
                        })
                    })
                    .unwrap();

                let users: Vec<User> = users_iter.filter_map(Result::ok).collect();
                respond_to.send(users).unwrap();
            }
        }
    }
}

// #[axum_macros::debug_handler]
async fn insert_user(
    axum::Extension(sender_shared): axum::Extension<Arc<Mutex<Sender<DbOperation>>>>,
    Json(user): Json<User>,
) {
    let sender = sender_shared.lock().await;
    sender
        .send(DbOperation::INSERT(user.name, user.age))
        .await
        .unwrap();
}

async fn list_users(
    axum::Extension(sender_shared): axum::Extension<Arc<Mutex<Sender<DbOperation>>>>,
) -> Json<Vec<User>> {
    let (respond_to, receiver) = tokio::sync::oneshot::channel();
    let sender = sender_shared.lock().await;

    sender.send(DbOperation::SELECT(respond_to)).await.unwrap();
    let users = receiver.await.unwrap();

    Json(users)
}
