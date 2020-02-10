
thread_local! {
    static X:u8 = 0;
}

fn main() {
    println!("{:#?}",X);
}
