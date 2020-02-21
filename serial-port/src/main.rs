use serialport::*;

fn main() {

    match available_ports() {

        Ok(ok_var) => println!("Ok {:?}", ok_var.len()),
        Err(err_var) => println!("Err {:?}", err_var)

    }

}
