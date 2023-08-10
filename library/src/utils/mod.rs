#[link(name="test_utils")]
extern "C" {
    pub fn ext_add(a: i32, b: i32) -> i32;
}


pub fn internal_add(a: i32, b: i32) -> i32 {
    return unsafe {
        ext_add(a, b)
    }
}
