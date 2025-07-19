use revolution::{
    Bundle, Resource, ResourceType, Tick,
    buildings::{Assembler, Furnace},
    recipes::{CopperSmelting, IronSmelting, PointRecipe},
};

fn main() {
    revolution::play(user_main);
}

fn user_main(mut tick: Tick, iron: Bundle<{ ResourceType::Iron }, 10>) -> (Tick, Bundle<{ ResourceType::Point }, 10>) {
    let mut iron = iron.to_resource();

    let mut points = Resource::<{ ResourceType::Point }>::empty();
    let win_bundle = points
        .bundle::<10>()
        .expect("This will always fail because you have no points yet.");
    (tick, win_bundle)
}
