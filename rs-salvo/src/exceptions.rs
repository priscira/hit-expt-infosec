use std::error::Error;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct NyquestError {
  pub msg: String,
}

impl NyquestError {
  pub fn new(msg: impl Into<String>) -> Self {
    Self { msg: msg.into() }
  }
}

impl fmt::Display for NyquestError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "nyquest failed: {}", self.msg)
  }
}

impl Error for NyquestError {}

impl From<nyquest::Error> for NyquestError {
  fn from(err: nyquest::Error) -> Self {
    NyquestError::new(err.to_string())
  }
}

#[derive(Debug)]
pub struct JzonError {
  pub msg: String,
}

impl JzonError {
  pub fn new(msg: impl Into<String>) -> Self {
    Self { msg: msg.into() }
  }
}

impl fmt::Display for JzonError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "jzon failed: {}", self.msg)
  }
}

impl Error for JzonError {}

impl From<jzon::Error> for JzonError {
  fn from(err: jzon::Error) -> Self {
    JzonError::new(err.to_string())
  }
}

#[derive(Debug)]
pub struct NjordError {
  pub msg: String,
}

impl NjordError {
  pub fn new(msg: impl Into<String>) -> Self {
    Self { msg: msg.into() }
  }
}

impl fmt::Display for NjordError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "njord failed: {}", self.msg)
  }
}

impl Error for NjordError {}

impl From<njord::sqlite::SqliteError> for NjordError {
  fn from(err: njord::sqlite::SqliteError) -> Self {
    match err {
      njord::sqlite::SqliteError::InsertError(err) => NjordError::new(err.to_string()),
      njord::sqlite::SqliteError::UpdateError(err) => NjordError::new(err.to_string()),
      njord::sqlite::SqliteError::DeleteError(err) => NjordError::new(err.to_string()),
      njord::sqlite::SqliteError::SelectError(err) => NjordError::new(err.to_string()),
    }
  }
}

#[derive(Debug)]
pub enum WeiboError {
  WeiboNyquestError(NyquestError),
  WeiboJzonError(JzonError),
  WeiboNjordError(NjordError),
}

impl fmt::Display for WeiboError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      WeiboError::WeiboNyquestError(err) => write!(f, "NyquestError: {}", err),
      WeiboError::WeiboJzonError(err) => write!(f, "JzonError: {}", err),
      WeiboError::WeiboNjordError(err) => write!(f, "NjordError: {}", err),
    }
  }
}

impl Error for WeiboError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      WeiboError::WeiboNyquestError(err) => Some(err),
      WeiboError::WeiboJzonError(err) => Some(err),
      WeiboError::WeiboNjordError(err) => Some(err),
    }
  }
}

impl From<jzon::Error> for WeiboError {
  fn from(err: jzon::Error) -> Self {
    WeiboError::WeiboJzonError(err.into())
  }
}

impl From<NyquestError> for WeiboError {
  fn from(err: NyquestError) -> Self {
    WeiboError::WeiboNyquestError(err)
  }
}

impl From<JzonError> for WeiboError {
  fn from(err: JzonError) -> Self {
    WeiboError::WeiboJzonError(err)
  }
}

impl From<NjordError> for WeiboError {
  fn from(err: NjordError) -> Self {
    WeiboError::WeiboNjordError(err)
  }
}
