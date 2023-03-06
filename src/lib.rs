use crate::baz::double;

mod baz;

pub fn roll_call() {
    println!("Test asm");
    let mut local_variable = 0;

    for name in NAMES.iter() {
        println!("{}: HERE!", name);
    }

    let num_present = NAMES.len();
    println!("All {} accounted for!", num_present);

    while local_variable < 10 {
        local_variable+=1;
    }
}

const NAMES: [&'static str; 10] = [
    "Kaladin", "Teft", "Drehy", "Skar", "Rock", "Sigzil", "Moash", "Leyten", "Lopen", "Hobber",
];

pub fn double_n(mut x: usize, mut n: usize) -> usize {
    while n > 0 {
        double(&mut x);
        n -= 1;
    }
    x
}