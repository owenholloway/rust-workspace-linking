use crate::utils::internal_add;

#[test]
fn add_function_test_1_plus_1() {
    let result = internal_add(1,1);

    assert_eq!(result, 2);

}
