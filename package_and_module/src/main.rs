mod garden;

use garden::say_hello;
use garden::vegetables::Asparagus;

fn main() {
    say_hello();

    let asparagus = Asparagus;
    println!("asparagus: {:?}", asparagus);
}
