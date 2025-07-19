play:
    cargo play

doc *FLAGS: 
    cargo doc -p revolution --no-deps {{ FLAGS }}
    cp -r target/doc/* docs/

push BRANCH:
    jj bookmark move {{BRANCH}} --to=@-
    jj git push

