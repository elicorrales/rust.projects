use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;


static mut SOME_EXTERN_VAR:u32 = 0;


fn main() {

    let my_thread_1 = |val|{
        //print!("1 ");
        unsafe {
            SOME_EXTERN_VAR = 11;
        }
        sleep(Duration::from_millis(500));
        unsafe {
            if SOME_EXTERN_VAR != 11 {
                //print!("E1 ");
            } 
        }
        std::io::stdout().flush().unwrap();
    };

    let my_thread_2 = |val|{
        //print!("2 ");
        unsafe {
            SOME_EXTERN_VAR = 22;
        }
        sleep(Duration::from_millis(500));
        unsafe {
            if SOME_EXTERN_VAR != 22 {
                //print!("E2 ");
            }
        }
        std::io::stdout().flush().unwrap();
    };


    for i in 0..40 {
        sleep(Duration::from_millis(80));
        spawn(my_thread_1(11));
        spawn(my_thread_2(22));
    }


    println!("");
    println!("Main thread waiting...");
    println!("");
    sleep(Duration::from_millis(1200));
    println!("");
    println!("Main thread done.");
}

