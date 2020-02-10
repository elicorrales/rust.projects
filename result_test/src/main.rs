/*

*/
//
fn return_result(msg:&str) -> Result<(), ()> {
    if msg == "0" {
        return Ok(());
    } else {
        return Err(());
    }
}

fn show_result(result:Result<(),()>) {
    match result {
        Ok(()) => println!("It was ok: {:?}", result),
        Err(())=> println!("There was an error: {:?}", result)
    }
}


fn main() {
    let result1 = return_result("0");
    show_result(result1);

    let result2 = return_result("1");
    show_result(result2);
}
