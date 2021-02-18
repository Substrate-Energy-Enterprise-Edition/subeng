use crate::cargo::{CargoRespond};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
//use sqlx::postgres::PgPool;
//use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[get("/cargos")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    println!("\n find_all route create \n");
    let result = CargoRespond::find_all(db_pool.get_ref()).await;
    match result {
        Ok(cargos) => HttpResponse::Ok().json(cargos),
        _ => HttpResponse::BadRequest().body("Error trying to read all cargos from database\n")
    }
}

#[get("/cargo/{cid}")]
async fn find(cid: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = CargoRespond::find_by_id(cid.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(cargo) => HttpResponse::Ok().json(cargo),
        _ => HttpResponse::BadRequest().body("Cargo not found")
    }
}

#[post("/cargo")]
async fn create(cargo: web::Json<CargoRespond>, db_pool: web::Data<PgPool>) -> impl Responder {
     println!("\n create route create \n");
    let result = CargoRespond::create(cargo.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(cargo) => HttpResponse::Ok().json(cargo),
        _ => HttpResponse::BadRequest().body("Error trying to create new Cargo \n")
    }
    
}

#[put("/cargo")]
async fn update(cargo: web::Json<CargoRespond>, db_pool: web::Data<PgPool>) -> impl Responder {
    println!("\n Update route create \n");
    let result = CargoRespond::update(cargo.into_inner(),db_pool.get_ref()).await;
    println!("\n Update route return \n");
    match result {
        Ok(cargo) => HttpResponse::Ok().json(cargo),
        _ => HttpResponse::BadRequest().body("Cargo not found")
    }
}

#[delete("/cargo/{cid}")]
async fn delete(cid: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    println!("\n delete route delete \n");
    let result = CargoRespond::delete(cid.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Cargo not found")
            }
        },
        _ => HttpResponse::BadRequest().body("Cargo not found")
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}