fn main() {
    use strum::IntoEnumIterator;
    for x in library::TestEnum::iter() {
        println!("{}", x);
    }
}
