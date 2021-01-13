//! Macros for defining cards

/// Create a new card with a struct name, card name, and cost
#[macro_export]
macro_rules! card {
    ($struct_name:ident, $name:expr, $cost:expr) => {
        pub struct $struct_name;
        impl Card for $struct_name {
            fn cost(&self) -> i32 {
                return $cost as i32;
            }
            
            fn name(&self) -> &str {
                return $name;
            }

            fn types(&self) -> Vec<&str> {
                return Vec::new();
            }
        }
    };

    ($struct_name:ident, $name:expr, $cost:expr, $types:expr) => {
        pub struct $struct_name;
        impl Card for $struct_name {
            fn cost(&self) -> i32 {
                return $cost as i32;
            }
            
            fn name(&self) -> &str {
                return $name;
            }

            fn types(&self) -> Vec<&str> {
                return $types;
            }
        }
    };
}
