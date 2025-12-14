use log::info;
use salvo_core::http::{Request, ResBody, Response, StatusCode};
use salvo_core::{async_trait, Depot, FlowCtrl, Handler};

pub struct LogLogger;

impl LogLogger {
  pub fn new() -> Self {
    Self
  }
}

#[async_trait]
impl Handler for LogLogger {
  async fn handle(
    &self, req: &mut Request, depot: &mut Depot, resp: &mut Response, ctrl: &mut FlowCtrl) {
    let req_met = req.method().to_string();
    let req_uri = req.uri().path().to_string();

    ctrl.call_next(req, depot, resp).await;

    let resp_sta = resp.status_code.unwrap_or(match &resp.body {
      ResBody::None => StatusCode::NOT_FOUND,
      ResBody::Error(flaw) => flaw.code,
      _ => StatusCode::OK,
    }).to_string();
    if let ResBody::Error(flaw) = &resp.body {
      info!("{} {} {} {}", req_met, req_uri, resp_sta, flaw.to_string());
    } else {
      info!("{} {} {}", req_met, req_uri, resp_sta);
    }
  }
}
