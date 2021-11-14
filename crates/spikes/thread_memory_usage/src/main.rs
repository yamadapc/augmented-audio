fn main() {
    let args = std::env::args();
    for arg in args.skip(1) {
        let n: usize = arg.parse().unwrap();
        for i in 0..n {
            std::thread::spawn(|| loop {
                println!("tid {:?}", std::thread::current().id());
            });
        }
    }
    std::thread::park();
}
