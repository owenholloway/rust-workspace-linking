#[no_mangle]
pub extern "C" fn ext_add(a: i32, b: i32) -> i32 {
    a + b + 10
}
