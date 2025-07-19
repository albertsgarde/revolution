use revolution::{
    Assembler, Furnace, Resource, ResourceType,
    recipes::{CopperSmelting, IronSmelting, PointRecipe},
};

pub fn main() {
    let (mut tick, mut iron) = revolution::start();

    let mut iron_furnace = Furnace::<IronSmelting>::build(&tick, iron.bundle().unwrap());

    while iron < 10 {
        let ores = revolution::mine_iron(&mut tick);
        iron_furnace.add_input(&tick, ores);
        while iron_furnace.cur_output(&tick) >= 1 {
            iron += iron_furnace.take_output::<1>(&tick).unwrap();
        }
        println!(
            "Furnace iron ore: {}, Inventory: {}",
            iron_furnace.cur_input(&tick),
            &iron
        );
    }

    let mut copper = Resource::<{ ResourceType::Copper }>::empty();
    let mut copper_furnace = Furnace::<CopperSmelting>::build(&tick, iron.bundle().unwrap());

    while copper < 10 || iron < 15 {
        while iron_furnace.cur_input(&tick) <= 4 {
            let ores = revolution::mine_iron(&mut tick);
            iron_furnace.add_input(&tick, ores);
            println!(
                "Furnace iron ore: {}, Furnace iron: {}, Inventory: {}",
                iron_furnace.cur_input(&tick),
                iron_furnace.cur_output(&tick),
                &iron
            );
        }
        while copper_furnace.cur_input(&tick) <= 4 {
            let ores = revolution::mine_copper(&mut tick);
            copper_furnace.add_input(&tick, ores);
        }
        while copper_furnace.cur_output(&tick) >= 1 {
            copper += copper_furnace.take_output::<1>(&tick).unwrap();
        }
        while iron_furnace.cur_output(&tick) >= 1 {
            iron += iron_furnace.take_output::<1>(&tick).unwrap();
        }
        tick.next();
        println!(
            "Iron ore: {}, Copper ore: {}, Inventory: {}, {}",
            iron_furnace.cur_input(&tick),
            copper_furnace.cur_input(&tick),
            &iron,
            &copper
        );
    }

    let mut points = Resource::<{ ResourceType::Point }>::empty();
    let mut assembler = Assembler::<PointRecipe>::build(&tick, iron.bundle().unwrap(), copper.bundle().unwrap());

    loop {
        while iron_furnace.cur_output(&tick) >= 1 {
            iron += iron_furnace.take_output::<1>(&tick).unwrap();
        }
        while copper_furnace.cur_output(&tick) >= 1 {
            copper += copper_furnace.take_output::<1>(&tick).unwrap();
        }
        while iron > 4 {
            assembler.add_input1(&tick, iron.bundle::<4>().unwrap());
        }
        while copper > 4 {
            assembler.add_input2(&tick, copper.bundle::<4>().unwrap());
        }
        while assembler.cur_output(&tick) >= 1 {
            points += assembler.take_output::<1>(&tick).unwrap();
        }
        while iron_furnace.cur_input(&tick) <= 4 {
            let ores = revolution::mine_iron(&mut tick);
            iron_furnace.add_input(&tick, ores);
        }
        while copper_furnace.cur_input(&tick) <= 4 {
            let ores = revolution::mine_copper(&mut tick);
            copper_furnace.add_input(&tick, ores);
        }
        tick.next();
        println!(
            "Iron ore: {}, Copper ore: {}, Assembler iron: {}, Assembler copper: {}, Inventory: {}, {}, {}",
            iron_furnace.cur_input(&tick),
            copper_furnace.cur_input(&tick),
            assembler.cur_input1(&tick),
            assembler.cur_input2(&tick),
            &iron,
            &copper,
            &points
        );

        if let Some(point_bundle) = points.bundle::<10>() {
            revolution::win(tick, point_bundle);
        }
    }
}
