# substrate-poe-pallet-test2
为poe-pallet 的 create_claim(), revoke_claim(), transfer_claim() 增加测试，以及设置claim的长度限制并测试。

---
pallets/poe/src/lib.rs:
```javascript
#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        #[pallet::constant]
        type MaximumClaimLength: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
    >;

    #[pallet::event]
    //#[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransferred(T::AccountId, T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        DestinationIsClaimOwner,
        ClaimTooBig,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            ensure!(
				claim.len() <= T::MaximumClaimLength::get() as usize,
				Error::<T>::ClaimTooBig
			);
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
            Proofs::<T>::insert(
                &claim,
                (sender.clone(), <frame_system::Pallet::<T>>::block_number()),
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            ensure!(
				claim.len() <= T::MaximumClaimLength::get() as usize,
				Error::<T>::ClaimTooBig
			);
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            destination: T::AccountId,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            ensure!(
				claim.len() <= T::MaximumClaimLength::get() as usize,
				Error::<T>::ClaimTooBig
			);
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            ensure!(owner != destination, Error::<T>::DestinationIsClaimOwner);
            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(
                &claim,
                (destination.clone(), <frame_system::Pallet::<T>>::block_number()),
            );
            Self::deposit_event(Event::ClaimTransferred(sender, destination, claim));
            Ok(().into())
        }
    }

}
```
---
pallets/poe/src/tests.rs:
```javascript
use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_failed_when_too_big() {
    new_test_ext().execute_with(||{
        //maximum claim length is set to 8 for test
        let claim = vec![0, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimTooBig
        );
    });
}

fn create_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
        Some((1, frame_system::Pallet::<Test>::block_number())))
    });
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );

    });
}

// test revoke claim ************************************************
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
                   None)
    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// test transfer claim ************************************************
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 3, claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((3, frame_system::Pallet::<Test>::block_number())))
    });
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), 2, claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_destination() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::DestinationIsClaimOwner
        );
    })
}
```
