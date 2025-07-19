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

    let mut iron_furnace = Furnace::<IronSmelting>::build(&tick, iron.bundle().unwrap());

    while iron < 200 {
        let ores = revolution::mine_iron(&mut tick);
        iron_furnace.add_input(&tick, ores);
        while iron_furnace.cur_output(&tick) >= 1 {
            iron += iron_furnace.take_output::<1>(&tick).unwrap();
        }
    }

    let _ = iron_furnace.empty_input(&tick);
    let mut copper = Resource::<{ ResourceType::Copper }>::empty();
    let mut copper_furnace = iron_furnace.change_recipe::<CopperSmelting>().unwrap();

    while copper < 200 {
        let ores = revolution::mine_copper(&mut tick);
        copper_furnace.add_input(&tick, ores);
        while copper_furnace.cur_output(&tick) >= 1 {
            copper += copper_furnace.take_output::<1>(&tick).unwrap();
        }
    }

    let mut points = Resource::<{ ResourceType::Point }>::empty();
    let mut assembler = Assembler::<PointRecipe>::build(&tick, iron.bundle().unwrap(), copper.bundle().unwrap());

    while iron > 4 {
        assembler.add_input1(&tick, iron.bundle::<4>().unwrap());
    }
    while copper > 4 {
        assembler.add_input2(&tick, copper.bundle::<4>().unwrap());
    }

    loop {
        while assembler.cur_output(&tick) >= 1 {
            points += assembler.take_output::<1>(&tick).unwrap();
        }
        tick.next();

        if let Some(point_bundle) = points.bundle::<10>() {
            return (tick, point_bundle);
        }
    }
}
