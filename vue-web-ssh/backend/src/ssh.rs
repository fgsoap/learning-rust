use actix_web::web;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub mod ssh {
    pub fn exec(info: web::Json<Info>) -> web::Json<Result> {
        info!("Start to connect to {}.", info.ssh_uri.clone());
        let tcp = TcpStream::connect(info.ssh_uri.clone()).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        let mut auth_err = String::new();
        sess.userauth_password(&info.username.clone(), &info.password.clone())
            .unwrap_or_else(|error| {
                auth_err = error.to_string();
            });
        if !sess.authenticated() {
            warn!("{}", auth_err);
            return web::Json(Result {
                result: auth_err.to_string(),
            });
        }
        info!("Authenticated!");
        // assert!(sess.authenticated());
        let mut channel = sess.channel_session().unwrap();
        channel.exec(&info.command.clone()).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.close().unwrap();
        info!("Disconnect from {} successfully.", info.ssh_uri.clone());
        web::Json(Result { result: s })
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Info {
        ssh_uri: String,
        username: String,
        password: String,
        command: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Result {
        result: String,
    }
}
