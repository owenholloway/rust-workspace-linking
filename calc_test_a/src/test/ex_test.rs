use lata_core_lib::utils::{internal_add, set_value, get_value};

#[test]
fn add_function_test() {
    let result = internal_add(1,1);

    assert_eq!(result, 2);

}

#[test]
fn can_set_and_get() {
    unsafe { 
        let result = get_value(1000);

        assert_eq!(result, 0.0);

        set_value(1000, 10.2);

        let result = get_value(1000);

        assert_eq!(result, 10.2);

    };
}
