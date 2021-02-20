CREATE TABLE IF NOT EXISTS hashs (         -- 上传的打包hashs数据包cargo 对应每条hash的验证信息表
    id          serial PRIMARY KEY,        -- ID 
    cid         text NOT NULL UNIQUE REFERENCES cargo(cid) ON DELETE CASCADE,          -- 对应的 cargo ID （ cargo表里的 cid ）
    account     text NOT NULL,
    hashcode    text NOT NULL,             -- hash值
    proof       text NOT NULL              -- hash值的 proof-data，包括节点和路径index
);

