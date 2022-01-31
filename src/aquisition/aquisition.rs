
/// Aquires a vector of numbers, now that I now about closures and generics
/// I wonder if this could be better...

pub fn aquire_vec() -> Vec<isize> {
    let mut vec: Vec<isize> = Vec::new();
    println!("give me ints one by one, q quits");
    loop {
        match aquire_num() {
            Some(num) => vec.push(num),
            None => {
                if vec.len() >= 1 {
                    break vec;
                } else {
                    println!("More than one number must be provided");
                }
            }
        }
    }
}

/// Aquires a number that must be an integer, if not it either prints a hint or
/// if its 'q' it quits
/// # Panic
/// if for some reason it can't get an input and read it as a string it panics
fn aquire_num() -> Option<isize> {
    let err = "int or q";
    let element = loop {
        let mut attempt = String::new();
        std::io::stdin().read_line(&mut attempt).expect(err);
        let trimmed = attempt.trim();
        match trimmed.parse::<isize>() {
            Ok(num) => break Some(num),
            Err(_) => match trimmed {
                "q" => break None,
                _ => {
                    println!("{}", err);
                }
            },
        }
    };
    element
}
/// Aquires a string
/// # Panic
/// If it fails to aquire a string it panics and says si
pub fn aquire_str() -> String {
    let err = "Not a valid string for some reason";
    println!("Human says :");
    let mut attempt = String::new();
    std::io::stdin().read_line(&mut attempt).expect(err);
    attempt.trim().to_string()
}
