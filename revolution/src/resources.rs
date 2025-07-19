//! Resources are held in either [`Resource`](Resource) or [`Bundle`](Bundle) objects.
//! [`Bundle`](Bundle) objects are used to hold a fixed amount of a resource, while [`Resource`](Resource) objects can hold any amount.

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

/// Holds an arbitrary amount of a resource.
/// A [`Resource`](Resource) object can be split into smaller parts, combined or [`Bundle`](Bundle)s can be extracted from them.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource<const RESOURCE_TYPE: ResourceType> {
    pub(crate) amount: u32,
}

impl<const RESOURCE_TYPE: ResourceType> Resource<RESOURCE_TYPE> {
    /// Creates a new empty [`Resource`](Resource).
    pub fn empty() -> Self {
        Self { amount: 0 }
    }

    pub fn resource_type(&self) -> ResourceType {
        RESOURCE_TYPE
    }

    /// The current amount of the resource contained in this [`Resource`](Resource).
    pub fn amount(&self) -> u32 {
        self.amount
    }

    /// Splits the [`Resource`](Resource) into two smaller parts.
    /// If there are insufficient resources in the [`Resource`](Resource), it returns an error with the original resource.
    pub fn split(self, amount: u32) -> Result<(Self, Self), Self> {
        if let Some(remaining) = self.amount.checked_sub(amount) {
            Ok((Resource { amount: remaining }, Resource { amount }))
        } else {
            Err(self)
        }
    }

    /// Consumes a [`Bundle`](Bundle) of the same resource type and adds the contained resources to this [`Resource`](Resource).
    pub fn add_bundle<const AMOUNT: u32>(&mut self, bundle: Bundle<RESOURCE_TYPE, AMOUNT>) {
        self.amount += bundle.amount();
    }

    /// Takes a specified amount of resources from this [`Resource`](Resource) and puts it into a [`Bundle`](Bundle).
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

/// Contains a fixed (compile-time known) amount of a resource.
/// A [`Bundle`](Bundle) can be used to build structures or as input for recipes.
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

    /// Converts this [`Bundle`](Bundle) into a [`Resource`](Resource) with the same resource type and amount.
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
