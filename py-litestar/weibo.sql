-- 创建表格weibo_hot_search
CREATE TABLE IF NOT EXISTS weibo_hot_search
(id INTEGER PRIMARY KEY AUTOINCREMENT,
 title TEXT NOT NULL,
 number INTEGER NOT NULL,
 special TEXT NOT NULL DEFAULT '',
 occur_era TEXT NOT NULL CHECK (occur_era GLOB '????-??-??'),
 UNIQUE (title, occur_era));

-- 创建表格weibo_hot_timeline
CREATE TABLE IF NOT EXISTS weibo_hot_timeline
(mid TEXT NOT NULL PRIMARY KEY,
 mblogid TEXT NOT NULL,
 text TEXT NOT NULL,
 mem_id TEXT NOT NULL,
 mem_name TEXT NOT NULL,
 occur_era TEXT NOT NULL CHECK (occur_era GLOB '????-??-??')
);

-- 创建表格weibo_hot_timeline_pic
CREATE TABLE IF NOT EXISTS weibo_hot_timeline_pic
(pic_id TEXT NOT NULL,
 pic_url TEXT NOT NULL,
 timeline_id TEXT,
 FOREIGN KEY(timeline_id) REFERENCES weibo_hot_timeline(mid) ON DELETE SET NULL
);

-- 创建表格weibo_hot_timeline_pic
CREATE TABLE IF NOT EXISTS weibo_hot_timeline_comm
(comm_mid TEXT NOT NULL PRIMARY KEY,
 text TEXT NOT NULL,
 mem_id TEXT NOT NULL,
 mem_name TEXT NOT NULL,
 comm_era TEXT NOT NULL CHECK (comm_era GLOB '????-??-??'),
 reply BOOLEAN NOT NULL,
 senior_id TEXT NOT NULL,
 timeline_id TEXT NOT NULL,
 FOREIGN KEY(senior_id) REFERENCES weibo_hot_timeline_comm(comm_mid) ON DELETE SET NULL,
 FOREIGN KEY(timeline_id) REFERENCES weibo_hot_timeline(mid) ON DELETE SET NULL
);
