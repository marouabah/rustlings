// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".to_string());
}

// ANSWER
// Ajouter <String> à la déclaration de shopping_list
// ajout de .to_string() à la ligne 13
