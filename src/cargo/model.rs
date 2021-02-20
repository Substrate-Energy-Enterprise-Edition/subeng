use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{FromRow, Row};
//use sqlx::postgres::PgPool;
use sqlx::PgPool;
//use sqlx::postgres::PgRow;
//use sqlx::postgres::PgDatabaseError;
use anyhow::{Result, anyhow};
//use log::info;
use chrono::{DateTime, Utc};
//use chrono::{NaiveDate, NaiveDateTime};


// this struct will use to represent cargo database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CargoRespond {
    #[serde(default = "default_id")]
    pub id: i64,
    #[serde(default = "default_cid")]
    pub cid: String,
    pub account: String,
    #[serde(default = "default_timestamp")]
    pub timestamp: i32,
    pub mkarr: Vec<String>,
    #[serde(default = "default_mkroot")]
    pub mkroot: String,
    #[serde(default = "default_blocknum")]
    pub blocknum: String,
    #[serde(default = "default_done")]
    pub done: bool
}

//BEGIN-----Deserialize Default Value for CargoRespond------

fn default_id() -> i64 {
    -1
}

fn default_cid() -> String {
    "0".to_string()
}

fn default_timestamp() -> i32 {
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



// this struct will be used to represent hashs database record
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Hash {
    pub id: i64,
    #[serde(default = "default_hash_cid")]
    pub cid: String,
    #[serde(default = "default_hash_account")]
    pub account: String,
    #[serde(default = "default_hash_hashcode")]
    pub hashcode: String,
    #[serde(default = "default_hash_proof")]
    pub proof: String
}

//BEGIN-----Deserialize Default Value for CargoRespond------

fn default_hash_cid() -> String {
    "0".to_string()
}

fn default_hash_account() -> String {
    "0".to_string()
}

fn default_hash_hashcode() -> String {
    "0".to_string()
}

fn default_hash_proof() -> String {
    "0".to_string()
}
//END---------------------------------------------------


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
        let now: DateTime<Utc> = Utc::now();
        let timestr = now.timestamp().to_string();
        let cidstr:String = format!("{}{}",&mkarr, &timestr ); 

        let  result = sqlx::query!( 
            r#"
                INSERT INTO cargo (cid, account, mkarr) VALUES ( digest($1, 'sha256'), $2, $3 ) 
            "#,
            &cidstr, 
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
                 UPDATE cargo SET mkarr = $1, done = $2, account = $3, mkroot = $4, blocknum = $5 WHERE id = $6 
                 "#,
                 &cargo.mkarr, 
                 cargo.done,
                 &cargo.account, 
                 &cargo.mkroot,
                 &cargo.blocknum,
                 &cargo.id
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
 
     pub async fn delete(id: i64, pool: &PgPool) -> Result<u64> {
         println!(" \n delete model  \n");
         let mut tx = pool.begin().await?;
         let deleted = sqlx::query!( 
             r#"
             DELETE FROM cargo WHERE id = $1 
             "#,
             id
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
                SELECT id, cid, account, timestamp, mkarr, mkroot, blocknum, done
                    FROM cargo
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            cargos.push(CargoRespond {
                id: rec.id,
                cid: rec.cid,
                account: rec.account,
                timestamp:   rec.timestamp,
                mkarr:  rec.mkarr,
                mkroot: rec.mkroot, 
                blocknum: rec.blocknum,
                done: rec.done
            });
        }

        Ok(cargos)
    }

    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<CargoRespond> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM cargo WHERE id = $1
                "#,
                id
            )
            .fetch_one(&*pool)
            .await?;

        Ok(CargoRespond {
            id: rec.id,
            cid: rec.cid,
            account: rec.account,
            timestamp:   rec.timestamp,
            mkarr:  rec.mkarr,
            mkroot: rec.mkroot,
            blocknum: rec.blocknum,
            done: rec.done
        })
    }

}