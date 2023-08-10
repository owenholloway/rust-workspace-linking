const MAX_POINTS: usize = 1000000;

static mut VALUES: [f32; MAX_POINTS] = [0.0; MAX_POINTS];

#[no_mangle]
pub extern "C" fn set_value(key: usize, value: f32) {

    unsafe {
        VALUES[key] = value;
    }

}

#[no_mangle]
pub extern "C" fn get_value(key: usize) -> f32 {

    unsafe {
        VALUES[key]
    }

}
