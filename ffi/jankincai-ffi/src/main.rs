use jankincai_ffi::jankincai::jankincai_sum;

fn main() {
    unsafe {
        println!("{:?}", jankincai_sum(1, 2));
    }
}
