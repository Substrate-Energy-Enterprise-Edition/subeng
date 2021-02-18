## SubEng.
##### --- 2021-02-18 by maxatbj
### Hackson project-- SEEE.io

#### run a browser on  http://localhost:5000 ,you will see below text, and play on it


### 欢迎来到能源联盟链 SEEE
####     Welcome to   < Hyper Bigdata on Chain SubEng} 
####     Openging API:

####       ------------ (1) Cargo ------------------
*        GET /cargos -> list all cargo 
*        POST /cargo -> creat cargo item, 
+ example: { "cid": "123", "account":"123456", "mktree":[ "1231231", "2323232", "343434343" ] ,  "done": false }
*        GET /cargo/{id} -> list cargo item by cid 
*        PUT /cargo/     -> update cargo item by cid 
+ example: example: { "cid": "123", "account":"123456", "mktree":[ "1231231", "2323232", "343434343" ] ,  "done": false }
        DELETE /cargo/{id} -> delete cargo item by cid 

####       ------------ (2) hash (not complete yet)------------------
 *       GET /hashs ->  list all hashs
 *       GET /hash/{id} -> list a hash by id
 *       DELETE /hash/{id} -> delete a hash item by id 
