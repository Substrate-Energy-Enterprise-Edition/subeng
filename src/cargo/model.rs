use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{FromRow, Row};
//use sqlx::postgres::PgPool;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
//use sqlx::postgres::PgDatabaseError;
use anyhow::{Result, anyhow};
use log::info;

//use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};


// this struct will use to represent cargo database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CargoRespond {
    #[serde(default = "default_cid")]
    pub cid: String,
    pub account: String,
    #[serde(default = "default_tstz")]
    pub tstz: i32,
    pub mkarr: Vec<String>,
    #[serde(default = "default_mkroot")]
    pub mkroot: String,
    #[serde(default = "default_blocknum")]
    pub blocknum: String,
    #[serde(default = "default_done")]
    pub done: bool
}

//BEGIN-----Deserialize Default Value for CargoRespond------

fn default_cid() -> String {
    "0".to_string()
}

fn default_tstz() -> i32 {
    0
}

fn default_mkroot() -> String {
    "0".to_string()
}

fn default_blocknum() -> String {
    "0".to_string()
}

fn default_done() -> bool {
   false
}


//END---------------------------------------------------



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


// Implementation for CargoRespond struct, functions for read/write/update and delete cargo from database
impl CargoRespond {

    pub async fn create(cargo: CargoRespond, pool: &PgPool) ->  Result<u64>   {
    
        let mkarr = serde_json::to_string(&cargo.mkarr)?;
        let  result = sqlx::query!( 
            r#"
                INSERT INTO cargo (cid, account, mkarr) VALUES ( digest($1, 'sha256'), $2, $3 ) 
            "#,
            &mkarr, 
            &cargo.account,
            &cargo.mkarr,
            )
            .execute(pool)
            .await;

        match result {
            Ok(cargo) => {
                println!("Insert {} rows ok!", cargo.rows_affected());
                Ok(cargo.rows_affected())
            },
            Err(error) => {
               println!("Insert error: {}", error);
               Err( anyhow!("Insert error: {}", error) )
               /*
                Sqlx error test:
                https://github.com/launchbadge/sqlx/blob/master/tests/postgres/postgres.rs
               */
            }
        }

    }


     pub async fn update(cargo: CargoRespond,  pool: &PgPool) -> Result<u64> {
      //  let mut tx = pool.begin().await.unwrap();


        println!(" \n Update  enter model : {:#?} \n", cargo);

        let result = sqlx::query!(
                 r#"
                 UPDATE cargo SET mkarr = $1, done = $2, account = $3, mkroot = $4, blocknum = $5 WHERE cid = $6 
                 "#,
                 &cargo.mkarr, 
                 cargo.done,
                 &cargo.account, 
                 &cargo.mkroot,
                 &cargo.blocknum,
                 &cargo.cid
            )             
             .execute(pool)
             .await?;

        let rows = result.rows_affected();

        Ok(rows)

        /*     match rows {
                Ok(gout) => {
                    println!("Update {} rows ok!", gout.rows_affected());
                    Ok(gout.rows_affected())
                },
                Err(error) => {
                   println!("Update error: {}", error);
                   Err( anyhow!("Update error: {}", error) )
                }
            }
        */

 
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
                mkarr:  rec.mkarr,
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
            mkarr:  rec.mkarr,
            mkroot: rec.mkroot,
            blocknum: rec.blocknum,
            done: rec.done
        })
    }

}