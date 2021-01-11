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
            
            fn name(&self) -> &'static str {
                return $name;
            }
        }
    };
}

/// Implement Treasure for a Card with a fixed gold value
#[macro_export]
macro_rules! treasure {
    ($struct_name:ident, $value:expr) => {
        impl Treasure for $struct_name {
            fn value(&self, _: &Player) -> i32 {
                return $value;
            }
        }
    };
}

/// Implement Victory for a Card with a fixed victory point value
#[macro_export]
macro_rules! victory {
    ($struct_name:ident, $points:expr) => {
        impl Victory for $struct_name {
            fn points(&self, _: &Player) -> i32 {
                return $points;
            }
        }
    };
}

/// Implement Curse for a Card with a fixed curse value
#[macro_export]
macro_rules! curse {
    ($struct_name:ident, $points:expr) => {
        impl CurseTrait for $struct_name {
            fn points(&self, _: &Player) -> i32 {
                return $points;
            }
        }
    };
}
