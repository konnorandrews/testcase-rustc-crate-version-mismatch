This is another test case for https://github.com/rust-lang/rust/issues/89143.
This particular setup was reported by @johnm#2610 (@JtotheThree) on the Rust Discord server.
This variant is hard to recognize because the help message contradicts with what the user has already done.

The problem was only recognized when attempting to using the fully qualified name failed.
```
error[E0277]: the trait bound `TestEnum: IntoEnumIterator` is not satisfied
 --> src/main.rs:7:14
  |
7 |     for x in <library::TestEnum as strum::IntoEnumIterator>::iter() {
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntoEnumIterator` is not implemented for `TestEnum`
```
This error is also not directly helpful, but at least it does not give an unhelpful help message.

## How to Run

Run the following
```
cd binary/
cargo +stable check
cargo +nightly check
```
The incorrect output follows.
The version mismatch is not detected, and a seemingly incorrect help is given.
```
error[E0599]: no variant or associated item named `iter` found for enum `TestEnum` in the current scope
 --> src/main.rs:3:33
  |
3 |     for x in library::TestEnum::iter() {
  |                                 ^^^^ variant or associated item not found in `TestEnum`
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
  |
1 | use strum::IntoEnumIterator;
  |

warning: unused import: `strum::IntoEnumIterator`
 --> src/main.rs:2:9
  |
2 |     use strum::IntoEnumIterator;
  |         ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0599`.
warning: `binary` (bin "binary") generated 1 warning
error: could not compile `binary` due to previous error; 1 warning emitted
```
