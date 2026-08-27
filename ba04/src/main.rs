use ba04::{add_u8_checked, add_u8_saturating, add_u8_wrapping};

fn main() {
    println!("{} + {} wrapping {}", 255, 1, add_u8_wrapping(255, 1));
    println!("{} + {} checked {:?}", 255, 1, add_u8_checked(255, 1));
    println!("{} + {} saturating {}", 255, 1, add_u8_saturating(255, 1));
}
