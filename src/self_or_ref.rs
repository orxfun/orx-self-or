use crate::sealed::Sealed;

/// A representation of an instance of a type that is either the instance itself,
/// or a shared reference to the instance.
///
/// Notice that this common representation:
/// * is capable of creating shared references through `get_ref` method;
/// * however, it cannot create mutable references.
pub trait SoR<T>: Sealed {
    /// Returns a reference to self.
    fn get_ref(&self) -> &T;
}

impl<T> SoR<T> for T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }
}

impl<T> SoR<T> for &T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }
}

fn sth(a: usize) -> usize {
    13
}
