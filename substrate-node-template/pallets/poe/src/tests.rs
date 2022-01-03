use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_failed_when_too_big() {
    new_test_ext().execute_with(||{
        //maximum claim length is set to 8 for test
        let claim = vec![1; 1024];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimTooBig
        );
    });
}

#[test]
fn create_claim_failed_when_too_small() {
    new_test_ext().execute_with(||{
        //minimum claim length is set to 4 for test
        let claim = vec![1; 8];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimTooSmall
        );
    });
}

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
        Some((1, frame_system::Pallet::<Test>::block_number())))
    });
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
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
        let claim = vec![1; 64];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
                   None)
    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
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
        let claim = vec![1; 64];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 3, claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((3, frame_system::Pallet::<Test>::block_number())))
    });
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![1; 64];
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
        let claim = vec![1; 64];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::DestinationIsClaimOwner
        );
    })
}







