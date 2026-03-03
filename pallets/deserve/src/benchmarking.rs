//! Benchmarking setup for the DeServe pallet.
use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet;
    use frame_system::RawOrigin;

    #[benchmark]
    fn store_value() {
        let caller: T::AccountId = whitelisted_caller();
        let value: u32 = 100;
        #[extrinsic_call]
        store_value(RawOrigin::Signed(caller), value);
        assert_eq!(Value::<T>::get().map(|v| v.value), Some(value.into()));
    }

    #[benchmark]
    fn increment_value() {
        let value: u32 = 100;
        let incremented_value: u32 = 101;
        Value::<T>::put(CompositeStruct { value });
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        increment_value(RawOrigin::Signed(caller));
        assert_eq!(Value::<T>::get().map(|v| v.value), Some(incremented_value));
    }

    #[benchmark]
    fn decrement_value() {
        let value: u32 = 100;
        let decremented_value: u32 = 99;
        Value::<T>::put(CompositeStruct { value });
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        decrement_value(RawOrigin::Signed(caller));
        assert_eq!(Value::<T>::get().map(|v| v.value), Some(decremented_value));
    }

    #[benchmark]
    fn reset_value() {
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        reset_value(RawOrigin::Signed(caller));
        assert_eq!(Value::<T>::get().map(|v| v.value), Some(0));
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock_runtime::new_test_ext(),
        crate::mock_runtime::Test,
    );
}
