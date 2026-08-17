//! The `encoding` module provides genotype implementations for common
//! standard-library and third-party collection types.
//!
//! Supported encoding schemes include:
//! * binary encoding (e.g. `Vec<bool>`, `FixedBitSet`)
//! * value encoding (e.g. `Vec<T>`, `SmallVec<A>`)
//! * permutation encoding (e.g. `Vec<usize>`, `SmallVec` of `usize`)
//! * tree encoding

use std::fmt::Debug;

use crate::genetic::Genotype;

/// Implementation of a genotype using `Vec`.
impl<V> Genotype for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;
}

#[cfg(feature = "fixedbitset")]
mod fixedbitset_genotype {
    use fixedbitset::FixedBitSet;

    use super::Genotype;

    /// Implementation of genotype using `fixedbistset::FixedBitSet`.
    impl Genotype for FixedBitSet {
        type Dna = bool;
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_genotype {
    use std::fmt::Debug;

    use smallvec::{Array, SmallVec};

    use super::Genotype;

    /// Implementation of genotype using `smallvec::SmallVec`.
    impl<A, V> Genotype for SmallVec<A>
    where
        A: Array<Item = V> + Sync,
        V: Clone + Debug + PartialEq + Send + Sync,
    {
        type Dna = V;
    }
}
