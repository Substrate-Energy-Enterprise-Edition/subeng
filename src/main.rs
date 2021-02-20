//#[macro_use]
//extern crate log;
use log::*;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
//use sqlx::postgres::PgPool;
//use sqlx::postgres::PgPoolOptions;
use anyhow::Result;



// import cargo module (routes and model)
mod cargo;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(r#"
        欢迎来到能源联盟链 SEEE
        Welcome to   < Hyper Bigdata on Chain SubEng} 
        Openging API:
        ------------ (1) Cargo ------------------
        GET /cargos -> list all cargo  
        POST /cargo -> creat cargo item, 
                       example: { "cid": "123", "account":"123456", "mktree":[ "1231231", "2323232", "343434343" ] ,  "done": false }
        GET /cargo/{id} -> list cargo item by cid  (find 提交的cid 以"/"单斜杠开头，url参数无需/转义字符)
        PUT /cargo/     -> update cargo item by cid (update 提交的cid必须是"//"双斜杠开头，实现/转义字符)
                           example: example: { "cid": "123", "account":"123456", "mktree":[ "1231231", "2323232", "343434343" ] ,  "done": false }
        DELETE /cargo/{id} -> delete cargo item by cid  (delete 提交的cid 以"/"单斜杠开头，url参数无需/转义字符)

        ------------ (2) hash (not complete yet)------------------
        GET /hashs ->  list all hashs
        GET /hash/{id} -> list a hash by id
        DELETE /hash/{id} -> delete a hash item by id 

        ------------（3）注意 ---------------
        POST/PUT 必须提供全部 [必须] 字段:(例如)
                     
        curl -X POST   'http://localhost:5000/cargo'  \
        -H 'Content-Type: application/json; charset=utf-8' \
        -d '{
            "id": -1                // id      [PUT必须], [POST缺省值]: 自增数字
            "cid": "",              // cid     [PUT必须], [POST缺省值]: "0"
            "account": "1234567",   // account [POST/PUT必须]
            "mkarr": [              // mkarr   [POST/PUT必须]  为要上链的数据 hash数组，
                "1231281",          // 数组每一个成员都是一个文件或者数据的 Hash 摘要值
                "2323232",
                "xzzzzzzzy"
            ],
            "tstz": 12312312,      // tstz   [缺省值]: 0
            "mkroo
            t":"0",          // mkroot [缺省值]: "0"
            "blocknum":"0",        // mkroot [缺省值]: "0"
            "done": false          // mkroot [缺省值]: false
        }'
    "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;
   /*
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;
    */
    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .configure(cargo::init) // init cargo routes
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}