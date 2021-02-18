CREATE TABLE IF NOT EXISTS cargo (       -- 上传的打包hashs数据包， 简称cargo
    cid         text NOT NULL PRIMARY KEY,        -- cargo ID ， 由 mktree -$1 字段做sha256运算而来 ->  digest($1, 'sha256')
    account     text NOT NULL,           -- user account
    tstz        timestamp NOT NULL DEFAULT current_timestamp,    --  Default: timestamp with timezone
    mkarr       text[] NOT NULL,           -- merkle tree with string array string 
    mkroot      text NOT NULL DEFAULT "0",    -- merkle root
    blocknum    text NOT NULL DEFAULT "0",    -- blockchain return block-hash 
    done        boolean NOT NULL DEFAULT FALSE    -- lable if block-hash returned and complete writing hash table 
);



CREATE TABLE IF NOT EXISTS cargo (       
    cid         text NOT NULL PRIMARY KEY,        
    account     text NOT NULL,         
    tstz        INTEGER NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),   
    mkarr       text[] NOT NULL,          
    mkroot      text NOT NULL DEFAULT '0',   
    blocknum    text NOT NULL DEFAULT '0',    
    done        boolean NOT NULL DEFAULT FALSE   
);

/*

-- First  enable PostgreSQL pgcrypto Extention:
create extension pgcrypto;

-- Insert a cargo
INSERT into cargo (cid, account, mkarr ) 
VALUES (digest('["123432423423","12321312312","123123123123"]', 'sha256'), '123456', '["123432423423","12321312312","123123123123"]');


digest($1, 'sha256')
*/



