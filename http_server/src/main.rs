
use std::net::TcpListener;
use std::net::TcpStream;
<<<<<<< HEAD
=======
use std::io::Read;
>>>>>>> 4001fa0dcb02d17d2704f97ba4eee03156b2e3bc
use std::io::Write;
use std::io::Error;

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
                            let resp_result = handle_client_request(_stream);
                            println!("{:?}", resp_result);
                            match resp_result {
                                Ok(ok_var) => println!("{:?}", ok_var),
                                Err(err_var) => println!("{:?}", err_var)
                            }
                        }

            _      => println!("There was an error...")

        }
    }
}

fn handle_client_request(mut stream:TcpStream) -> Result<usize,Error> {
    println!("Inside request handler: {:?}", stream);
    let response = "HTTP/1.1 200 OK \r\n\r\n";
    let result = stream.write(response.as_bytes());
    return result;
}
