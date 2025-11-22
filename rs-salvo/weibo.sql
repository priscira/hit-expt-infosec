-- 创建表格weibo_hot_search
CREATE TABLE IF NOT EXISTS weibo_hot_search
(id INTEGER PRIMARY KEY AUTOINCREMENT,
 title TEXT NOT NULL,
 number INTEGER NOT NULL,
 special TEXT NOT NULL DEFAULT '',
 occur_time TEXT NOT NULL CHECK (occur_time GLOB '????-??-??'));