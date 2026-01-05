from piccolo.engine.sqlite import SQLiteEngine
from src.prefs import WEIBO_DB_PTH

DB = SQLiteEngine(path=WEIBO_DB_PTH)
