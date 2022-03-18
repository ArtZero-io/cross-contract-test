#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod test2 {
    use ink_env::CallFlags;
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;

    use ink_storage::{
        traits::{
            SpreadAllocate
        }
    };

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
    pub struct Test2{
        #[OwnableStorageField]
        ownable: OwnableData,
        test1_address: AccountId,
        is_active: bool
    }

    impl Ownable for Test2 {}

    #[brush::wrapper]
    pub type Test1Ref = dyn Test1;

    #[brush::trait_definition]
    pub trait Test1 {
        #[ink(message)]
        fn is_active(&self) -> bool;
    }

    impl Test2 {
        #[ink(constructor)]
        pub fn new(owner:AccountId, test1_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.is_active = true;
                instance.test1_address = test1_address;
                instance._init_with_owner(owner);
            })
        }

        /* GETTERS */
        /// Standard cross call
        #[ink(message)]
        pub fn is_active(&self) -> bool {
            Test1Ref::is_active(&self.test1_address)
        }

        /// Cross call with set_forward_input enable
        #[ink(message)]
        pub fn is_active_1(&self) -> bool {
            Test1Ref::is_active_builder(&self.test1_address).call_flags(CallFlags::default().set_forward_input(true)).fire().unwrap()
        }
        /// Cross call with set_allow_reentry enable
        #[ink(message)]
        pub fn is_active_2(&self) -> bool {
            Test1Ref::is_active_builder(&self.test1_address).call_flags(CallFlags::default().set_allow_reentry(true)).fire().unwrap()
        }
        /// Cross call with set_tail_call enable
        #[ink(message)]
        pub fn is_active_3(&self) -> bool {
            Test1Ref::is_active_builder(&self.test1_address).call_flags(CallFlags::default().set_tail_call(true)).fire().unwrap()
        }

        #[ink(message)]
        pub fn get_test_1(&self) -> AccountId {
            self.test1_address
        }

    }
}
