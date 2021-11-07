use actix::{Actor,StreamHandler};
use actix_web_actors::ws::{self, Message, WebsocketContext};

pub struct WsConn {
    pub nick: String,
}

impl Actor for WsConn {
    type Context = WebsocketContext<Self>;

    /// 连接上
    fn started(&mut self, _: &mut Self::Context) {
        println!("{} join!", self.nick);
    }

    /// 断开连接
    fn stopped(&mut self, _: &mut Self::Context) {
        println!("{} exit!", self.nick);
    }
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(text)) => {
                println!("received: {}", text);
                // let req = WsRequest::from_str(&text);
                // req.run_cmd(ctx.address());
                ctx.text(text)
            },
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}