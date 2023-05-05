#[ctor::ctor]
fn dylib_init() {
    println!("INIT");
}
