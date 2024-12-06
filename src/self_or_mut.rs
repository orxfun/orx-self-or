use crate::sealed::Sealed;

/// A representation of an instance of a type that is either the instance itself,
/// or a mutable reference to the instance.
///
/// Notice that, either variant of the instance is capable of:
/// * producing a shared reference through `get_ref` method, and
/// * a mutable reference through `get_mut` method.
pub trait SoM<T>: Sealed {
    /// Returns a reference to self.
    fn get_ref(&self) -> &T;

    /// Returns a mutable reference to self.
    fn get_mut(&mut self) -> &mut T;
}

impl<T> SoM<T> for T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> SoM<T> for &mut T {
    #[inline(always)]
    fn get_ref(&self) -> &T {
        self
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        self
    }
}
