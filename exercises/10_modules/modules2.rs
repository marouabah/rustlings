// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    pub use self::fruits::PEAR as fruit ;
    pub use self::veggies::CUCUMBER as veggie ;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

// ANSWER
// ajout de pub use self::fruits::PEAR as fruit ; pour rendre la constante PEAR publique et la renommer fruit
// ajout de pub use self::veggies::CUCUMBER as veggie ; pour rendre la constante CUCUMBER publique et la renommer veggie
