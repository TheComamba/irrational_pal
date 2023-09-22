use data::e;

mod data;

fn main() {
    let digits_e = data::extraction::get_digits(&e::E, 0, 10);
    for i in 0..10 {
        println!("{}: {}", i, digits_e[i]);
    }

    let digits_pi = data::extraction::get_digits(&data::pi::PI, 0, 10);
    for i in 0..10 {
        println!("{}: {}", i, digits_pi[i]);
    }
}
