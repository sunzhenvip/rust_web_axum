#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

// use crate::entities::users;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, Database, DbConn, DbErr, NotSet};

const DATABASE_URL: &str = "mysql://root:root@localhost:3306/seaorm";

async fn run() -> Result<DbConn, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    if let Ok(db) = run().await {
        println!("链接成功")
    } else {
        println!("链接失败")
    }
}
