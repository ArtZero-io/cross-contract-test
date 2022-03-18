#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod test1 {
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_lang::ToAccountId;


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),

    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }


    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct TestContract{
        #[OwnableStorageField]
        ownable: OwnableData,

        is_active: bool
    }

    impl Ownable for TestContract {}

    #[brush::trait_definition]
    pub trait Test1 {
        #[ink(message)]
        fn is_active(&self) -> bool;
    }

    impl TestContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.is_active = true;
            })
        }

    }

    impl Test1 for TestContract{
        /* GETTERS */

        #[ink(message)]
        fn is_active(&self) -> bool {
            return self.is_active;
        }
    }
}
