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
(id INTEGER PRIMARY KEY AUTOINCREMENT,
 mid TEXT NOT NULL,
 mblogid TEXT NOT NULL,
 text TEXT NOT NULL,
 mem_id TEXT NOT NULL,
 mem_name TEXT NOT NULL,
 occur_era TEXT NOT NULL CHECK (occur_era GLOB '????-??-??'),
 UNIQUE (mid));

-- 创建表格weibo_hot_timeline_pic
CREATE TABLE IF NOT EXISTS weibo_hot_timeline_pic
(id INTEGER PRIMARY KEY AUTOINCREMENT,
 mid TEXT NOT NULL,
 pic_id TEXT NOT NULL,
 pic_url TEXT NOT NULL);

-- 创建表格weibo_hot_timeline_pic
CREATE TABLE IF NOT EXISTS weibo_hot_timeline_comm
(id INTEGER PRIMARY KEY AUTOINCREMENT,
 mid TEXT NOT NULL,
 comm_mid TEXT NOT NULL,
 text TEXT NOT NULL,
 mem_id TEXT NOT NULL,
 mem_name TEXT NOT NULL,
 comm_era TEXT NOT NULL CHECK (comm_era GLOB '????-??-??'),
 reply BOOLEAN NOT NULL,
 senior_id TEXT NOT NULL);
