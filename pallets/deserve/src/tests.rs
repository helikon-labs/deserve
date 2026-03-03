use crate::{mock_runtime as runtime, Error, Event, Value};
use frame::testing_prelude::*;

const ACCOUNT_ID: u64 = 33;

fn run_to_block(n: u64) {
    while runtime::System::block_number() < n {
        if runtime::System::block_number() > 0 {
            runtime::DeServe::on_finalize(runtime::System::block_number());
            runtime::System::on_finalize(runtime::System::block_number());
        }
        runtime::System::reset_events();
        runtime::System::set_block_number(runtime::System::block_number() + 1);
        runtime::System::on_initialize(runtime::System::block_number());
        runtime::DeServe::on_initialize(runtime::System::block_number());
    }
}

#[test]
fn store_value_works() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let value: u32 = 42;
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::store_value(origin, value,));
        assert_eq!(Value::<runtime::Test>::get().map(|v| v.value), Some(value));
        runtime::System::assert_has_event(
            Event::ValueStored {
                value,
                who: ACCOUNT_ID,
            }
            .into(),
        );
    });
}

#[test]
fn increment_value_works() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::store_value(origin.clone(), u32::MAX - 1,));
        assert_ok!(runtime::DeServe::increment_value(origin,));
        assert_eq!(
            Value::<runtime::Test>::get().map(|v| v.value),
            Some(u32::MAX)
        );
        runtime::System::assert_has_event(
            Event::ValueIncremented {
                new_value: u32::MAX,
                who: ACCOUNT_ID,
            }
            .into(),
        );
    });
}

#[test]
fn correct_error_for_increment_none_value() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_noop!(
            runtime::DeServe::increment_value(origin),
            Error::<runtime::Test>::NoneValue,
        );
    });
}

#[test]
fn correct_error_for_increment_overflow() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::store_value(origin.clone(), u32::MAX - 1,));
        assert_ok!(runtime::DeServe::increment_value(origin.clone(),));
        assert_noop!(
            runtime::DeServe::increment_value(origin),
            Error::<runtime::Test>::StorageOverflow,
        );
    });
}

#[test]
fn decrement_value_works() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::store_value(origin.clone(), 5,));
        assert_ok!(runtime::DeServe::decrement_value(origin,));
        assert_eq!(Value::<runtime::Test>::get().map(|v| v.value), Some(4));
        runtime::System::assert_has_event(
            Event::ValueDecremented {
                new_value: 4,
                who: ACCOUNT_ID,
            }
            .into(),
        );
    });
}

#[test]
fn correct_error_for_decrement_none_value() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_noop!(
            runtime::DeServe::decrement_value(origin),
            Error::<runtime::Test>::NoneValue,
        );
    });
}

#[test]
fn correct_error_for_decrement_overflow() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::store_value(origin.clone(), 1,));
        assert_ok!(runtime::DeServe::decrement_value(origin.clone(),));
        assert_noop!(
            runtime::DeServe::decrement_value(origin),
            Error::<runtime::Test>::StorageOverflow,
        );
    });
}

#[test]
fn reset_value_works() {
    runtime::new_test_ext().execute_with(|| {
        run_to_block(1);
        let origin = runtime::RuntimeOrigin::signed(ACCOUNT_ID);
        assert_ok!(runtime::DeServe::reset_value(origin.clone()));
        assert_eq!(Value::<runtime::Test>::get().map(|v| v.value), Some(0));
        runtime::System::assert_has_event(Event::ValueReset { who: ACCOUNT_ID }.into());
    });
}
