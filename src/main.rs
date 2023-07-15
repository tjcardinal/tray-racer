const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    println!("P3\n{WIDTH} {HEIGHT}\n255");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = x;
            let g = y;
            let b = 15;

            println!("{r} {g} {b}");
        }
    }
}
