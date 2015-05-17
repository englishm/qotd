use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;


fn handle_tcp_client(mut stream: TcpStream) {
    let quote = "
Of course, there isn’t any \"God of the Internet.\"
The Internet works because a lot of people cooperate to do things together.
  ~ Jon Postel
".as_bytes();
     let _ = stream.write(quote);
}

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:17").unwrap();
    println!("Listening on TCP port 17...");
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_tcp_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    drop(tcp_listener);
}
