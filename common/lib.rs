#![cfg_attr(not(feature = "std"), no_std)]

pub use openbrush::contracts::traits::psp34::Id;

#[derive(Debug, Clone, scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Bar {
    pub inner: Id,
}

