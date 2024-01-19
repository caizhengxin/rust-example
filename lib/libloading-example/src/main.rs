type Jankincai = unsafe fn(u8, u8) -> u8;


fn call_dynamic() -> Result<u8, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./crates/jankincai/target/release/libjankincai.so")?;
        let func: libloading::Symbol<Jankincai> = lib.get(b"jankincai")?;

        println!(">>> {:?}", func(1, 2));

        Ok(0)
    }
}


fn main() {
    println!("{:?}", call_dynamic());
}
