use std::io;
use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

fn main()
{
    let mut ssh_uri = String::new();

    println!("Please input the Server IP and SSH Port: IP:Port");
    io::stdin()
        .read_line(&mut ssh_uri)
        .expect("Failed to read SSH URI.");
    let ssh_uri: String = match ssh_uri.trim().parse() {
        Ok(str) => str,
        Err(err) => err.to_string(),
    };

    let tcp = TcpStream::connect(&ssh_uri).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let mut username = String::new();

    println!("Please input your username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username.");

    let username: String = match username.trim().parse() {
        Ok(str) => str,
        Err(err) => err.to_string(),
    };

    // Prompt for a password on STDOUT
    let password = rpassword::prompt_password_stdout("Please input your password: ").unwrap();
    sess.userauth_password(&username, &password).unwrap();

    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("cat /var/log/*.log").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    println!("{}", channel.exit_status().unwrap());
}