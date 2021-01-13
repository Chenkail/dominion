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

#[macro_export]
macro_rules! name {
    ($name:expr) => {
        fn name(&self) -> &str { $name }
    };
}

#[macro_export]
macro_rules! cost {
    ($cost:expr) => {
        fn cost(&self) -> i32 { $cost }
    };
}

#[macro_export]
macro_rules! types {
    ($types:expr) => {
        fn types(&self) -> Vec<&str> { $types }
    };
}

#[macro_export]
macro_rules! treasure_value {
    ($value:expr) => {
        fn treasure_value(&self, _: &Player) -> i32 { $value }
    };
}

#[macro_export]
macro_rules! simple_action {
    ($cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        fn action_effects(&self, player: &mut Player, _: &mut Game) {
            player.draw_cards($cards);
            player.add_actions($actions);
            player.add_buys($buys);
            player.add_coins($coins);
        }
    };
}
