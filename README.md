# Revolution
Aid the revolution by taking ownership of resources and using them to build industry for the people!

## How to play
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repo.
3. A play of this game is a Rust executable where the main function does nothing but call `revolution::play()`. This function takes a closure. Writing this closure is the gameplay. I would recommend starting by editing `game/src/main.rs`.
    Some examples of play are in `revolution/src/bin/`.
4. Run with `cargo play`. This will compile and run your solution and tell you if you win or panic.

## Rules
The only rule that is not enforced by the compiler is that you should use the library as it exists in the repo. By editing the library it is trivial to do arbitrarily well. I have deliberately not done anything to prevent this, as that could be seen as a challenge.

## Help
Library documentation is available [here](https://albertsgarde.github.io/revolution).
A good place to start is to build a [furnace](https://albertsgarde.github.io/revolution/revolution/buildings/struct.Furnace.html) and start [mining](https://albertsgarde.github.io/revolution/revolution/fn.mine_iron.html) and smelting iron.