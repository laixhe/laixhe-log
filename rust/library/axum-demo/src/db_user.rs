use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub uid: i64,
    pub uname: String,
    pub age: i32,
    pub text: Option<String>, // 使用 Option 来处理潜在的空值情况
    pub create_at: String,
    pub updated_at: String,
}

pub async fn insert_user(mysql_pool: &MySqlPool, user: User) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::error::Error> {

    let sql = String::from("INSERT INTO `user`(uname, age, create_at)VALUES(?, ?, ?)");

    let result = sqlx::query(sql.as_str())
        .bind(user.uid)
        .bind(user.uname)
        .bind(user.age)
        .bind(user.text)
        .bind(user.create_at)
        .execute(mysql_pool).await?;

        Ok(result)
}
