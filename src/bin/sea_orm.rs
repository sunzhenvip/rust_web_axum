#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

// use crate::entities::users; // 这么写有问题
use axum_weibo::entities::wb_user; // 可以这么写引入路径
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, Database, DbConn, DbErr, NotSet};

// const DATABASE_URL: &str = "mysql://root:root@localhost:3306/seaorm";
const DATABASE_URL: &str = "mysql://root:UUff98Y97hj@v@192.168.2.226:42730/test1";

async fn run() -> Result<DbConn, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    if let Ok(db) = run().await {
        let res = add_user(&db).await;
        println!("{:?}", res);
        println!("链接成功")
    } else {
        println!("链接失败")
    }
}

// 新增数据到数据库
async fn add_user(db: &DbConn) -> Result<(), DbErr> {
    let user = wb_user::ActiveModel {
        uid: NotSet,
        phone: Set("15346231050".to_string()),
        password: Set("123456".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    };
    let res = user.insert(db).await?;
    println!("{:?}", res);
    Ok(())
}
