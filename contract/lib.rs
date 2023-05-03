#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod foo {

    use common::{Bar, Id};

    #[ink(storage)]
    pub struct Foo {
        value: Id,
    }

    impl Foo {
        #[ink(constructor)]
        pub fn new(init_value: Bar) -> Self {
            Self {
                value: init_value.inner,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> Id {
            self.value.clone()
        }
    }
}
