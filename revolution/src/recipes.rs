//! A recipe is a way of turning resources into other resources.
//! A specific recipe specifies the input and output resources, as well as the time it takes to complete the recipe.

use std::fmt::Debug;

use crate::ResourceType;

/// Any recipe that implements this trait can be used in an [`Assembler`](crate::buildings::Assembler).
pub trait AssemblerRecipe: Debug {
    const INPUT1: ResourceType;
    const INPUT1_AMOUNT: u32;
    const INPUT2: ResourceType;
    const INPUT2_AMOUNT: u32;
    const OUTPUT: ResourceType;
    const OUTPUT_AMOUNT: u32;
    const TIME: u64;
}

/// The recipe you need to win! An [`Assembler`](crate::buildings::Assembler) recipe that creates points. Converts 4 iron and 4 copper into 1 point resource. Takes 20 ticks.
#[derive(Debug)]
pub struct PointRecipe;

impl AssemblerRecipe for PointRecipe {
    const INPUT1: ResourceType = ResourceType::Iron;
    const INPUT1_AMOUNT: u32 = 4;
    const INPUT2: ResourceType = ResourceType::Copper;
    const INPUT2_AMOUNT: u32 = 4;
    const OUTPUT: ResourceType = ResourceType::Point;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 20;
}

/// Any recipe that implements this trait can be used in a [`Furnace`](crate::buildings::Furnace).
pub trait FurnaceRecipe: Debug {
    const INPUT: ResourceType;
    const INPUT_AMOUNT: u32;
    const OUTPUT: ResourceType;
    const OUTPUT_AMOUNT: u32;
    const TIME: u64;
}

/// A [`Furnace`](crate::buildings::Furnace) recipe that smelts iron ore into iron. Converts 2 iron ore into 1 iron. Takes 10 ticks.
#[derive(Debug)]
pub struct IronSmelting;

impl FurnaceRecipe for IronSmelting {
    const INPUT: ResourceType = ResourceType::IronOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Iron;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}

/// A [`Furnace`](crate::buildings::Furnace) recipe that smelts copper ore into copper. Converts 2 copper ore into 1 copper. Takes 10 ticks.
#[derive(Debug)]
pub struct CopperSmelting;

impl FurnaceRecipe for CopperSmelting {
    const INPUT: ResourceType = ResourceType::CopperOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Copper;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}
