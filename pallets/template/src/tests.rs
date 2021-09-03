use crate::{mock::*, Error, Something};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(TemplateModule::something(), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            TemplateModule::cause_error(Origin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}

#[test]
fn runtime_api_success() {
    new_test_ext().execute_with(|| {
        Something::<Test>::put(12);
        assert_eq!(TemplateModule::get_something(), 12);
    });
}

#[test]
fn runtime_api_fail() {
    new_test_ext().execute_with(|| {
        assert_eq!(TemplateModule::get_something(), 0);
    });
}
