use actix::{Actor,StreamHandler};
use actix_web_actors::ws;

pub struct WsCon{
    pub nick : String
}

impl Actor for WsCon {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("{} join!", self.nick);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("{} exit!", self.nick);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsCon {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg))=>ctx.pong(&msg),
            Ok(ws::Message::Text(text))=>ctx.text(text),
            Ok(ws::Message::Binary(bin))=>ctx.binary(bin),
            _ => (),
        }
    }
}