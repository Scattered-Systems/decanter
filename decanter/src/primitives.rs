/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::types::*;

mod types {
    use generic_array::GenericArray;
    use typenum::{
        bit::{B0, B1},
        uint::{UInt, UTerm},
    };

    ///
    pub type GenericHashOutput = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
    ///
    pub type GenericHash<T = u8, Output = GenericHashOutput> = GenericArray<T, Output>;
}
