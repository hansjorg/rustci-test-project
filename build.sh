./clean.sh
rustc --lib hellolib.rs
rustc --test hellolib.rs
rustc -L . hello.rs
