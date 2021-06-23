#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod counter {
    use ink_prelude;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
        /// Stores a single `i8` value on the storage.
        value: i8,
    }

    impl Counter {
        /// Constructor that initializes the `i8` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i8) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `i8` value to `0`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `i8` from `0`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn increment(&mut self) {
            // note: adding one like this is an easy way to accidentally overflow
            // real smart contracts will want to have safety checks
            self.value += 1;
            let log_message = ink_prelude::format!("Increased number to {}", self.value);
            ink_env::debug_println(&log_message);
        }
        /// Simply returns the current value of our `i8`.
        #[ink(message)]
        pub fn get(&self) -> i8 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let counter = Counter::default();
            assert_eq!(counter.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut counter = Counter::new(1);
            assert_eq!(counter.get(), 1);
            counter.increment();
            assert_eq!(counter.get(), 2);
        }
    }
}
