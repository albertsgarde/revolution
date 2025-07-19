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

pub fn play(main: fn(Tick, Bundle<{ ResourceType::Iron }, 10>) -> (Tick, Bundle<{ ResourceType::Point }, 10>)) -> ! {
    if ONCE.is_completed() {
        panic!("revolution::play() can only be called once per program run.");
    }
    ONCE.call_once(|| {});
    let tick = Tick::start();
    let iron = Bundle::<{ ResourceType::Iron }, 10>::new();
    let (tick, _points) = main(tick, iron);
    panic!("You won in {tick} ticks!")
}

pub fn mine_iron(tick: &mut Tick) -> Bundle<{ ResourceType::IronOre }, 1> {
    for _ in 0..2 {
        tick.next();
    }
    Bundle::new()
}

pub fn mine_copper(tick: &mut Tick) -> Bundle<{ ResourceType::CopperOre }, 1> {
    for _ in 0..2 {
        tick.next();
    }
    Bundle::new()
}
