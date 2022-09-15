#![no_std]

elrond_wasm::imports!();

/// This contract is meant to store 10% of the $ECITY token's supply, and unlock it over 5 years for the Team to use in the project's developement.
#[elrond_wasm::contract]
pub trait VestingLocker {
    #[init]
    fn init(&self) {}
}
