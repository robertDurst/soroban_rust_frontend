pub fn simple_panic() {
    if true {
        if true {
            panic!("This is a panic message");
        }

        println!("This is not a panic message");
    }
}
