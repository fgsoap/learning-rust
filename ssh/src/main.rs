use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;

use actix_web::{App, HttpResponse, HttpServer, Result, web};
use askama::Template;
use ssh2::Session;

fn ssh(uri: &String, u: &String, p: &String, c: &String) -> String {
    let tcp = TcpStream::connect(uri).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(u, p).unwrap_or_default();
    if !sess.authenticated() {
        return "Not authenticated!".to_string();
    }
    // assert!(sess.authenticated());
    let mut channel = sess.channel_session().unwrap();
    channel.exec(c).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    return s;
}

#[derive(Template)]
#[template(path = "command.html")]
struct CommandTemplate<'a> {
    text: &'a str
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let uri = query.get("ssh_uri");
    let u = query.get("username");
    let p = query.get("password");
    let s = if let Some(c) = query.get("command")
    {
        let r = ssh(uri.unwrap(), u.unwrap(), p.unwrap(), c);
        CommandTemplate {
            text: &*r
        }
            .render()
            .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start http server
    HttpServer::new(move || {
        App::new().service(web::resource("/").route(web::get().to(index)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}