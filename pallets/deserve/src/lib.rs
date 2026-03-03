//! DeServe initial pallet.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock_runtime;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use frame::prelude::*;

    /// Pallet configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Sample storage struct.
    #[derive(
        Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct CompositeStruct {
        /// The value.
        pub(crate) value: u32,
    }

    /// A storage item.
    #[pallet::storage]
    pub type Value<T: Config> = StorageValue<_, CompositeStruct>;

    /// Pallet events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ValueStored { value: u32, who: T::AccountId },
        ValueIncremented { new_value: u32, who: T::AccountId },
        ValueDecremented { new_value: u32, who: T::AccountId },
        ValueReset { who: T::AccountId },
    }

    /// Pallet errors.
    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Pallet calls.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Store a simple u32 value.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::store_value())]
        pub fn store_value(origin: OriginFor<T>, value: u32) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <Value<T>>::put(CompositeStruct { value });
            Self::deposit_event(Event::ValueStored { value, who });
            Ok(().into())
        }

        /// Increment the stored value.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::increment_value())]
        pub fn increment_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Value<T>>::get() {
                None => Err(Error::<T>::NoneValue)?,
                Some(mut value) => {
                    value.value = value
                        .value
                        .checked_add(1)
                        .ok_or(Error::<T>::StorageOverflow)?;
                    <Value<T>>::put(value.clone());
                    Self::deposit_event(Event::ValueIncremented {
                        new_value: value.value,
                        who,
                    });
                    Ok(().into())
                }
            }
        }

        /// Decrement the stored value.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::decrement_value())]
        pub fn decrement_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Value<T>>::get() {
                None => Err(Error::<T>::NoneValue)?,
                Some(mut value) => {
                    value.value = value
                        .value
                        .checked_sub(1)
                        .ok_or(Error::<T>::StorageOverflow)?;
                    <Value<T>>::put(value.clone());
                    Self::deposit_event(Event::ValueDecremented {
                        new_value: value.value,
                        who,
                    });
                    Ok(().into())
                }
            }
        }

        /// Decrement the stored value.
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::reset_value())]
        pub fn reset_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <Value<T>>::put(CompositeStruct { value: 0 });
            Self::deposit_event(Event::ValueReset { who });
            Ok(().into())
        }
    }
}
