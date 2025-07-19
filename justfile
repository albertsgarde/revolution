play:
    cargo play

doc:
    cargo doc -p revolution --no-deps
    cp -r target/doc/* docs/