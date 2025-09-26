#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hello_world {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct HelloWorld {}

    impl HelloWorld {
        /// Constructor (required in ink!)
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Message function to return "Hello World"
        #[ink(message)]
        pub fn say_hello(&self) -> String {
            String::from("Hello World")
        }
    }

    /// Implement Default for convenience
    impl Default for HelloWorld {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_says_hello() {
            let contract = HelloWorld::default();
            assert_eq!(contract.say_hello(), "Hello World");
        }
    }
}
