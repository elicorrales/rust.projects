
use std::net::TcpListener;

const IP:&str = "localhost:8594";

fn main() {

    println!("Starting server....");

    let _listener = TcpListener::bind(IP).expect("Unable to create listener.");

    println!("{:?}", _listener);

    println!("Server listening for http://{}", IP);
    for result in _listener.incoming() {

        println!("Got incoming connection request...");

        println!("{:?}", result);

        match result {

            Ok(_stream) => println!("Handling request....{:?}", _stream),

            _      => println!("There was an error...")

        }

    }
}
