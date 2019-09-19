use core::fmt::Display;
use core::hash::Hash;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait DataType:
    Eq + Ord + Clone + Debug + Send + Serialize + DeserializeOwned + Sync + Hash + Display
{
}
impl<N> DataType for N where
    N: Eq + Ord + Clone + Debug + Send + Serialize + DeserializeOwned + Sync + Hash + Display
{
}
