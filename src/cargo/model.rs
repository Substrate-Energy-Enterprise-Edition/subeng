use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{FromRow, Row};
//use sqlx::postgres::PgPool;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use anyhow::Result;
//use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

// this struct will use to represent cargo database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CargoRequest {
    pub cid: String,
    pub account: String,
    pub tstz: i32,
    pub mkarr: String,
    pub mkroot: String,
    pub blocknum: String,
    pub done: bool
}




// this struct will use to represent cargo database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CargoRespond {
    pub cid: String,
    pub account: String,
    pub tstz: i32,
    pub mkarr: Vec<String>,
    pub mkroot: String,
    pub blocknum: String,
    pub done: bool
}

// this struct will be used to represent hashs database record
#[derive(Serialize, FromRow, Debug)]
pub struct Hash {
    pub id: i32,
    pub cid: String,
    pub hashcode: String,
    pub proof: String
}


// implementation of Actix Responder for CargoRespond struct so we can return CargoRespond from action handler
impl Responder for CargoRespond {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}


impl Responder for CargoRequest {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}



// Implementation for CargoRespond struct, functions for read/write/update and delete cargo from database
impl CargoRespond {

    pub async fn create(cargo: CargoRespond, pool: &PgPool) -> Result<CargoRespond> {

        // let mut conn = pool.begin().await.unwrap();
         println!(" \n cargo sqlx after tx await \n  {} \n", &cargo.account);
         let mkarr = serde_json::to_string(&cargo.mkarr)?;
 
        let cargo = sqlx::query!( 
             r#"
                 INSERT INTO cargo (cid, account, mkarr) VALUES ( digest($1, 'sha256'), $2, $3 ) 
             "#,
             &mkarr, 
             &cargo.account,
             serde_json::to_string(&cargo.mkarr)?,
         )
         .map(|row: PgRow| {
            CargoRequest{
                 cid: row.get(0),
                 account: row.get(1),
                 tstz: row.get(2),
                 mkarr: row.get(3),
                 mkroot: row.get(4),
                 blocknum: row.get(5),
                 done: row.get(6)
             }
         }) 
        .fetch_one(pool)
        .await?;

        println!(" \n cargo before OK(cargo) \n  \n");

        let cargo1 = CargoRespond {
            cid: cargo.cid,
            account: cargo.account,
            tstz: cargo.tstz,
            mkarr: serde_json::from_str(&cargo.mkarr)?,
            mkroot: cargo.mkroot,
            blocknum: cargo.blocknum,
            done: cargo.done
        };


       // let cargo1 = serde_json::to_string_pretty(&cargo)?;

       // println!(" \n cargo before OK(cargo) \n {} \n", cargo1);

         Ok(cargo1)
     }
 
     pub async fn update(cargo: CargoRespond,  pool: &PgPool) -> Result<CargoRespond> {
      //  let mut tx = pool.begin().await.unwrap();
        println!(" \n create cargo update  enter model and cid: \n");
       
        
         let cargo = sqlx::query!(
                 r#"
                 UPDATE cargo SET mkarr = $1, done = $2, account = $3, mkroot = $4, blocknum = $5 WHERE cid = $6 
                 "#,
                 serde_json::to_string(&cargo.mkarr)?, 
                 cargo.done,
                 &cargo.account, 
                 &cargo.mkroot,
                 &cargo.blocknum,
                 &cargo.cid
            )
             .map(|row: PgRow| {
                CargoRequest {
                     cid: row.get(0),
                     account: row.get(1),
                     tstz: row.get(2),
                     mkarr: row.get(3),
                     mkroot: row.get(4),
                     blocknum: row.get(5),
                     done: row.get(6)
                 }
             }) 
             .fetch_one(pool)
             .await?;

             println!(" \n cargo before OK(cargo) \n  \n");

            let cargo1 = CargoRespond {
                cid: cargo.cid,
                account: cargo.account,
                tstz: cargo.tstz,
                mkarr: serde_json::from_str(&cargo.mkarr)?,
                mkroot: cargo.mkroot,
                blocknum: cargo.blocknum,
                done: cargo.done
            };



            // let cargo1 = serde_json::to_string_pretty(&cargo)?;

            // println!(" \n update cargo before OK(cargo) \n {} \n", cargo1);
              
         Ok(cargo1)
     }
 
     pub async fn delete(cid: String, pool: &PgPool) -> Result<u64> {
         println!(" \n delete model  \n");
         let mut tx = pool.begin().await?;
         let deleted = sqlx::query!( 
             r#"
             DELETE FROM cargo WHERE cid = $1 
             "#,
             cid
         )
             .execute(&mut tx)
             .await?;
 
         tx.commit().await?;
         let rows = deleted.rows_affected();
         Ok(rows)
     }


    pub async fn find_all(pool: &PgPool) -> Result<Vec<CargoRespond>> {
        println!(" \n find_all model  \n");
        let mut cargos = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT cid, account, tstz, mkarr, mkroot, blocknum, done
                    FROM cargo
                ORDER BY cid
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            cargos.push(CargoRespond {
                cid: rec.cid,
                account: rec.account,
                tstz:   rec.tstz,
                mkarr:  serde_json::from_str(&rec.mkarr)?,
                mkroot: rec.mkroot, 
                blocknum: rec.blocknum,
                done: rec.done
            });
        }

        Ok(cargos)
    }

    pub async fn find_by_id(cid: String, pool: &PgPool) -> Result<CargoRespond> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM cargo WHERE cid = $1
                "#,
                cid
            )
            .fetch_one(&*pool)
            .await?;

        Ok(CargoRespond {
            cid: rec.cid,
            account: rec.account,
            tstz:   rec.tstz,
            mkarr:  serde_json::from_str(&rec.mkarr)?,
            mkroot: rec.mkroot,
            blocknum: rec.blocknum,
            done: rec.done
        })
    }

}