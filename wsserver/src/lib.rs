
use actix_web::{HttpRequest,HttpResponse,Error,web};
mod ws_con;

pub async fn ws_handle(
    req: HttpRequest,
	stream: web::Payload,) -> Result<HttpResponse,Error>{
    let resp = actix_web_actors::ws::start(ws_con::WsConn{nick:"".to_string()}, &req, stream);
    resp
}
