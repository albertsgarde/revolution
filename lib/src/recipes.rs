use std::fmt::Debug;

use crate::ResourceType;

pub trait AssemblerRecipe: Debug {
    const INPUT1: ResourceType;
    const INPUT1_AMOUNT: u32;
    const INPUT2: ResourceType;
    const INPUT2_AMOUNT: u32;
    const OUTPUT: ResourceType;
    const OUTPUT_AMOUNT: u32;
    const TIME: u64;
}

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

pub trait FurnaceRecipe: Debug {
    const INPUT: ResourceType;
    const INPUT_AMOUNT: u32;
    const OUTPUT: ResourceType;
    const OUTPUT_AMOUNT: u32;
    const TIME: u64;
}

#[derive(Debug)]
pub struct IronSmelting;

impl FurnaceRecipe for IronSmelting {
    const INPUT: ResourceType = ResourceType::IronOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Iron;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}

#[derive(Debug)]
pub struct CopperSmelting;

impl FurnaceRecipe for CopperSmelting {
    const INPUT: ResourceType = ResourceType::CopperOre;
    const INPUT_AMOUNT: u32 = 2;
    const OUTPUT: ResourceType = ResourceType::Copper;
    const OUTPUT_AMOUNT: u32 = 1;
    const TIME: u64 = 10;
}
