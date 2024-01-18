// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

// ANSWER
// ajout de derive(Debug) pour pouvoir afficher les valeurs de l'enum
