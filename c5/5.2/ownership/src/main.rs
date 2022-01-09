fn main() {
    let mut top_grossing_films = vec!["Avatar", "Avengers: Endgame", "Titanic"];
    let top_grossing_films_mutable_reference = &mut top_grossing_films;
    top_grossing_films_mutable_reference.push("Star Wars: The Force Awakens");
    let top_grossing_films_reference = &top_grossing_films;
    println!(
        "Printed using immutable reference: {:#?}",
        top_grossing_films_reference
    );
    let top_grossing_films_moved = top_grossing_films;
    println!("Printed after moving: {:#?}", top_grossing_films_moved);

    // println!("Print using original value: {:#?}", top_grossing_films);
    // println!(
    //     "Print using mutable reference: {:#?}",
    //     top_grossing_films_mutable_reference
    // );
}
