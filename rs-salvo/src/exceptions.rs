use std::error::Error;
use std::fmt;
use salvo::http::ParseError;

#[derive(fmt::Debug)]
pub enum WeiboError {
  NyquestError(String),
  JzonError(String),
  SalvoError(String),
  RbatisError(String),
}

impl fmt::Display for WeiboError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      WeiboError::NyquestError(err) => write!(f, "NyquestError: {}", err),
      WeiboError::JzonError(err) => write!(f, "JzonError: {}", err),
      WeiboError::SalvoError(err) => write!(f, "SalvoError: {}", err),
      WeiboError::RbatisError(err) => write!(f, "RbatisError: {}", err),
    }
  }
}

impl Error for WeiboError {}

impl From<nyquest::Error> for WeiboError {
  fn from(err: nyquest::Error) -> Self {
    WeiboError::NyquestError(err.to_string())
  }
}

impl From<jzon::Error> for WeiboError {
  fn from(err: jzon::Error) -> Self {
    WeiboError::JzonError(err.to_string())
  }
}

impl From<rbs::Error> for WeiboError {
  fn from(err: rbs::Error) -> Self {
    WeiboError::RbatisError(err.to_string())
  }
}

impl From<ParseError> for WeiboError {
  fn from(err: ParseError) -> Self {
    WeiboError::SalvoError(err.to_string())
  }
}

impl From<hifitime::HifitimeError> for WeiboError {
  fn from(err: hifitime::HifitimeError) -> Self {
    WeiboError::JzonError(err.to_string())
  }
}

#[macro_export]
macro_rules! weibo_nyquest_err {
  ($msg:expr) => {
    WeiboError::NyquestError($msg.to_string())
  };
}

#[macro_export]
macro_rules! weibo_jzon_err {
  ($msg:expr) => {
    WeiboError::JzonError($msg.to_string())
  };
}
