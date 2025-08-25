use cutie::color::Color;
use cutie::string::Cute;

/// Displays a 16 x 16 square of all the colors for extended color
/// with their index as the color it represents
///
/// `0-7` are the standard colors, defined by Colors
///
/// `8-15` are the bright standard colors, also defined by Colors
///
/// `16-231` are calculated from RGB via `16 + 36 * r + 6 * g + b` where `(0 <= r,g,b <= 5)`
///
/// `232-255` are grayscale from dark to light in 24 steps gradually
fn main() {
    // Use for loops to design a 16x16 square
    for row in 0..16 {
        for column in 0..16 {
            let index = row * 16 + column; // Get the index 0-255
            let cell: String = format!("{:<4}", index); // Generate the cell
            // TODO: Shouldn't have the do this, need to implement better fmt method for CuteString to allow for raw padding
            print!("{}", cell.foreground(Color::ExtendedColor(index)))
        }
        print!("\n"); // Separate the rows
    }
    println!() // Finally output the full square
}