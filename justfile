play:
    cargo play

doc:
    cargo doc -p revolution --no-deps
    cp -r target/doc/* docs/

push BRANCH:
    jj bookmark move {{BRANCH}} --to=@-
    jj git push
    