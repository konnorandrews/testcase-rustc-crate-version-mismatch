fn main() {
    use strum::IntoEnumIterator;
    for x in library::TestEnum::iter() {
        println!("{}", x);
    }

    /*
    for x in <library::TestEnum as strum::IntoEnumIterator>::iter() {
        println!("{}", x);
    }
    */
}
