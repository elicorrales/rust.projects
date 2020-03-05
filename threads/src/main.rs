use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

fn main() {

    let task1 = || {
        sleep(Duration::from_millis(200));
        println!("Task 1 is done.");
    };

    let task2 = || {
        sleep(Duration::from_millis(200));
        println!("Task 2 is done.");
    };

    println!("Main is starting....");

    spawn(task1);
    spawn(task2);

    println!("Main is waiting....");

    sleep(Duration::from_millis(500));

    println!("Main is done");
}
