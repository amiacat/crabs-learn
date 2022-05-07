fn main() {
    let apples = 5; // Não muda lol
    println!("The apples are {apples}");

    let mut oranges = 3;
    println!("The oranges are {oranges}, can I buy more?");

    oranges += 1;

    println!("Take one, the oranges are now {oranges}");

    // Constantes
    const THREE_PLUS_FOUR: u32 = 3 + 4;

    println!("The plus is {THREE_PLUS_FOUR}");

    let x = 5;

    let x = x * x;

    println!("The value is {x}");
}
