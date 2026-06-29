deploy:
    cross build --target aarch64-unknown-linux-gnu --release
    scp target/aarch64-unknown-linux-gnu/release/doorbell pi@<pi-ip>:~/doorbell
