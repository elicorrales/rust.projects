
use serialport::*;

fn main() {

    match available_ports() {

        Ok(ok_var) => println!("{:?}", ok_var),
        Err(err_var) => println!("{:?}", err_var)

    }

}
