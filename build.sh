./clean.sh
rustc --lib hellolib.rs
rust test hellolib.rs
rustc -L . hello.rs
