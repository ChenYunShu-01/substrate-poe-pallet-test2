use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
        Some((1, frame_system::Pallet::<Test>::block_number())))
    });
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );

    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exsit() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}