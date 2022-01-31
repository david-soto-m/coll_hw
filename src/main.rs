mod statistics;
use statistics::statistics as stt;
mod aquisition;
use aquisition::aquisition as aq;
mod piggify;
use piggify::piggify as pig;
mod company;
use company::company as comp;

/// # Solution to the collections homework
/// The proposed solution to the three problems suggested by the rust book
/// Also a test for publishing


fn main() {
    println!("{:?}", stt::stats(&aq::aquire_vec()));
    println!("{}", pig::pig_latin(&aq::aquire_str()));
    comp::interact();
}
