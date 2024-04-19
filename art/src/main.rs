//use art::kinds::PrimaryColor;
//use art::utils::mix;
// after refactor:
use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
