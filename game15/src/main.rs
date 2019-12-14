use std::io;

fn print_line(row: &[i32]) {
    println!("| {:02} | {:02} | {:02} | {:02} |", row[0], row[1], row[2], row[3]);
}

fn main() {
    println!("Starting 15 game");

    println!();

    let row1: [i32; 4] = [1,2,3,4];
    let row2: [i32; 4] = [5,6,7,8];
    let row3: [i32; 4] = [9,10,11,12];
    let row4: [i32; 4] = [13,14,15,0];

    println!("o----o----o----o----o");
    print_line(&row1);
    println!("o----o----o----o----o");
    print_line(&row2);
    println!("o----o----o----o----o");
    print_line(&row3);
    println!("o----o----o----o----o");
    print_line(&row4);
    println!("o----o----o----o----o");

    println!();
    println!("End game");
}
