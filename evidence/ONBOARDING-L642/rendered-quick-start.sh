git clone https://github.com/aien-dev/interplane-mv.git interplane
cd interplane
git checkout --detach b669d06e21c9debb15073289d7c0b78b1b2d362c
cd rust
cargo fetch --locked
cargo run --locked --offline -q -p interplane-crossveil --example offline
