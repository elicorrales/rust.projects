use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Instant, Duration};
use std::thread;

lazy_static! {

    static ref SOME_RESOURCE:Mutex<u32> = Mutex::new(0);

}

fn main() {

    let since_main_start = Instant::now();

    for _i in 0..5 {
        let since_loop_start = Instant::now();
        //-------------------------------------------------------------------

        thread::spawn(||{
            let since_thread_start = Instant::now();

            let _a = *SOME_RESOURCE.lock().unwrap();

            thread::sleep(Duration::from_millis(600));

            println!("Thread 1 is done in {:.2?}", since_thread_start.elapsed());
        });

        thread::spawn(||{
            let since_thread_start = Instant::now();

            let _a = *SOME_RESOURCE.lock().unwrap();

            thread::sleep(Duration::from_millis(600));

            println!("Thread 2 is done in {:.2?}", since_thread_start.elapsed());
        });



        //-------------------------------------------------------------------
        println!("Loop is done in {:.2?}", since_loop_start.elapsed());
    }

    println!("Main is waiting in {:.2?}", since_main_start.elapsed());

    thread::sleep(Duration::from_millis(2000));

    println!("Main is done in {:.2?}", since_main_start.elapsed());
}
