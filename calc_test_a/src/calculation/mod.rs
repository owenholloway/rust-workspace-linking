#[no_mangle]
pub extern "C" fn example() -> i32 {

    return unsafe {
        lata_core_lib::utils::ext_add(1, 5)
    };
}