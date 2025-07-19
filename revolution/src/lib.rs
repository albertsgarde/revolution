#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)] // silence the “still incomplete” lint

pub mod buildings;
pub mod recipes;
mod resources;
mod tick;

use std::sync::Once;

pub use resources::{Bundle, Resource, ResourceType};
pub use tick::Tick;

static ONCE: Once = Once::new();

/// Runs your play. If it is run multiple times, it will panic. This is to prevent using multiple threads to cheat.
pub fn play(main: fn(Tick, Bundle<{ ResourceType::Iron }, 10>) -> (Tick, Bundle<{ ResourceType::Point }, 10>)) -> ! {
    if ONCE.is_completed() {
        panic!("revolution::play() can only be called once per program run.");
    }
    ONCE.call_once(|| {});
    let tick = Tick::start();
    let iron = Bundle::<{ ResourceType::Iron }, 10>::new();
    let (tick, _points) = main(tick, iron);
    println!("The revolution succeeded or something!");
    println!("You won in {tick} ticks!");
    std::process::exit(0);
}

/// Mines iron ore. Takes 2 ticks to mine 1 ore.
pub fn mine_iron(tick: &mut Tick) -> Bundle<{ ResourceType::IronOre }, 1> {
    for _ in 0..2 {
        tick.next();
    }
    Bundle::new()
}

/// Mines copper ore. Takes 2 ticks to mine 1 ore.
pub fn mine_copper(tick: &mut Tick) -> Bundle<{ ResourceType::CopperOre }, 1> {
    for _ in 0..2 {
        tick.next();
    }
    Bundle::new()
}
