# loc-counter
loc-counter is a little Rust project I made in order to learn some Rust. The app itself it pretty useful (especially for Windows) and simple to use. You can count the total number of lines of code of every file in every subdirectory in a given directory with one command. You can even filter out some files by extension, e.g. only count .rs file or .cpp and .hpp.

# How to use it?

1. Clone the git repo and cd into it:
```
git clone https://github.com/GameWin221/loc-counter
cd loc-counter
```
2. Run the following command:
```
cargo run --release <directory> [extensions]
```
**(the extensions are optional)**