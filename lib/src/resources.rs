use std::{
    fmt::Display,
    marker::{ConstParamTy, PhantomData},
    ops::AddAssign,
};

#[derive(ConstParamTy, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Point,
    IronOre,
    Iron,
    CopperOre,
    Copper,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource<const RESOURCE_TYPE: ResourceType> {
    pub(crate) amount: u32,
}

impl<const RESOURCE_TYPE: ResourceType> Resource<RESOURCE_TYPE> {
    pub fn empty() -> Self {
        Self { amount: 0 }
    }

    pub fn resource_type(&self) -> ResourceType {
        RESOURCE_TYPE
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn split(self, amount: u32) -> Result<(Self, Self), Self> {
        if let Some(remaining) = self.amount.checked_sub(amount) {
            Ok((Resource { amount: remaining }, Resource { amount }))
        } else {
            Err(self)
        }
    }

    pub fn add_bundle<const AMOUNT: u32>(&mut self, bundle: Bundle<RESOURCE_TYPE, AMOUNT>) {
        self.amount += bundle.amount();
    }

    pub fn bundle<const AMOUNT: u32>(&mut self) -> Option<Bundle<RESOURCE_TYPE, AMOUNT>> {
        if let Some(remaining) = self.amount.checked_sub(AMOUNT) {
            self.amount = remaining;
            Some(Bundle { dummy: PhantomData })
        } else {
            None
        }
    }
}

impl<const RESOURCE_TYPE: ResourceType> Display for Resource<RESOURCE_TYPE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} x {}", RESOURCE_TYPE, self.amount)
    }
}

impl<const RESOURCE_TYPE: ResourceType> PartialOrd<u32> for Resource<RESOURCE_TYPE> {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.amount.cmp(other))
    }
}

impl<const RESOURCE_TYPE: ResourceType> PartialEq<u32> for Resource<RESOURCE_TYPE> {
    fn eq(&self, other: &u32) -> bool {
        self.amount == *other
    }
}

impl<const RESOURCE_TYPE: ResourceType> PartialOrd<Resource<RESOURCE_TYPE>> for u32 {
    fn partial_cmp(&self, other: &Resource<RESOURCE_TYPE>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other.amount))
    }
}

impl<const RESOURCE_TYPE: ResourceType> PartialEq<Resource<RESOURCE_TYPE>> for u32 {
    fn eq(&self, other: &Resource<RESOURCE_TYPE>) -> bool {
        *self == other.amount
    }
}

pub struct Bundle<const RESOURCE_TYPE: ResourceType, const AMOUNT: u32> {
    dummy: PhantomData<ResourceType>,
}

impl<const RESOURCE_TYPE: ResourceType, const AMOUNT: u32> Bundle<RESOURCE_TYPE, AMOUNT> {
    pub(crate) fn new() -> Self {
        Self { dummy: PhantomData }
    }

    pub fn resource_type(&self) -> ResourceType {
        RESOURCE_TYPE
    }

    pub fn amount(&self) -> u32 {
        AMOUNT
    }

    pub fn to_resource(self) -> Resource<RESOURCE_TYPE> {
        Resource { amount: AMOUNT }
    }
}

impl<const RESOURCE_TYPE: ResourceType, const AMOUNT: u32> AddAssign<Bundle<RESOURCE_TYPE, AMOUNT>>
    for Resource<RESOURCE_TYPE>
{
    fn add_assign(&mut self, _bundle: Bundle<RESOURCE_TYPE, AMOUNT>) {
        self.amount += AMOUNT;
    }
}
