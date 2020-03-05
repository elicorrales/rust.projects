use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUM:Mutex<usize>  = Mutex::new(10);
}


fn main() {

    let task1 = || {
        let mut rnum = NUM.lock().unwrap();
        *rnum = 11;
        sleep(Duration::from_millis(400));
        if *rnum != 11 {
            print!("E1 ");
        }
    };

    let task2 = || {
        let mut rnum = NUM.lock().unwrap();
        *rnum = 22;
        sleep(Duration::from_millis(400));
        if *rnum != 22 {
            print!("E2 ");
        }
    };


    println!("Main is starting....");


    for i in 0..20 {
        spawn(task1);
        spawn(task2);
    }

    println!("Main is waiting....");

    sleep(Duration::from_millis(1000));


    println!("");
    println!("In Main: {}", NUM.lock().unwrap());

    println!("Main is done");
}
