use text_io::read;

use negabinary_converter::decompose;

fn main() {
    loop {
        print!("Enter a number to decompose: ");
        let target: f64 = read!();

        println!("{}", decompose(target));
    }
}
