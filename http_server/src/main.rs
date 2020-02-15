
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;

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

            Ok(_stream) => {
                            println!("Handling request....{:?}", _stream);
                            handle_client_request(_stream);
                        }

            _      => println!("There was an error...")

        }
    }
}

fn handle_client_request(mut stream:TcpStream) {
    println!("Inside request handler: {:?}", stream);

    let mut incoming_request_data = [0;32];
    println!("{:?}", incoming_request_data);

    let read_result = stream.read(&mut incoming_request_data);
    match read_result {
        Ok(ok_var) => println!("got Ok: {:?}", ok_var),
        Err(err_var) => println!("got Err: {:?}", err_var)
    }

    let bytes_to_string = std::str::from_utf8(&incoming_request_data);

    //println!("{:?}", incoming_request_data.unwrap());
    println!("{:?}", incoming_request_data);
    println!("{:?}", bytes_to_string.unwrap());
}




