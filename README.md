# emilhul-simulation

## How to install

* Clone repo
* Build using `cargo build --release` from directory
* Launch emilhul-sorting.exe
* (Optional) Run unit tests with `cargo test`

## Disclaimer

Not quite finished yet. For no apperent reason I decided against using a premade crate for vectors and related math and instead created my own Vector2 struct found in `./src/vector2.rs`. To make sure I could rely on this struct quite some time was spent on writing unit tests. To run only these tests use `cargo test vector2`. (Although currently no other test exist).

The program is supposed to be a simulation of a small part of the forest where an bird hunts for mice. Currently the bird will fly to a certain spot in the bottom corner and the mice will randomly choose a direction each frame. That is until they detect one another. They then switch to seek and flee behaviours respectively. The turning isn't working properly when the angle is more than ±π/2. It will then instead turn 180 degree in the oposite direction.

Press the escape key (`esc`) to exit application
