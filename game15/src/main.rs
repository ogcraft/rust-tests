use std::io;

fn print_line(row: &[i32]) {
    println!(".----.----.----.");
    println!("| {:02} | {:02} | {:02} |", row[0], row[1], row[2]);
    println!(".----.----.----.");
}

fn main() {
    println!("Starting 15 game");

    println!();

    let row: [i32; 3] = [1,2,3];

    print_line(&row);

    println!();
    println!("End game");
}
