use std::fmt;

use super::*;

/// A transparent color with an alpha channel
///
/// Alpha of 1 means the color is fully opaque, and alpha of 0 means it's fully transparent.
///
/// This uses a straight alpha, not a premultiplied alpha.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Alpha<C, A> {
    pub color: C,
    pub alpha: A
}

impl<C, A: Channel> Alpha<C, A> {
    /// Creates a new alpha channel.
    ///
    /// This makes sure that the alpha-channel is in the proper range
    /// by calling `Channel::to_range`
    pub fn new(color: C, alpha: A) -> Self {
        Alpha { color, alpha: alpha.to_range() }
    }
}

impl<C: Color, A: Channel> Color for Alpha<C, A> {
    fn normalize(self) -> Self {
        let color = self.color.normalize();
        let alpha = self.alpha.to_range();
        Alpha { color, alpha }
    }

    fn is_normal(&self) -> bool {
        self.color.is_normal() && self.alpha.in_range()
    }
}

impl<T, A, S> Alpha<RGBColor<T, S>, A> {
    /// Deconstructs this color into a tuple of it's channels
    pub fn tuple(self) -> (T, T, T, A) {
        (self.color.r, self.color.g, self.color.b, self.alpha)
    }
}

impl<T, S> Alpha<RGBColor<T, S>, T> {
    /// Deconstructs this color into an array of it's channels
    pub fn array(self) -> [T; 4] {
        [self.color.r, self.color.g, self.color.b, self.alpha]
    }
}

impl<A> Alpha<SRGBColor, A> {
    pub fn std_decode(self) -> Alpha<LinRGBColor, A> {
        Alpha { color: self.color.std_decode(), alpha: self.alpha }
    }
}

impl<A> Alpha<LinRGBColor, A> {
    pub fn std_encode(self) -> Alpha<SRGBColor, A> {
        Alpha { color: self.color.std_encode(), alpha: self.alpha }
    }
}

impl<C: Default, A: Channel> Default for Alpha<C, A> {
    fn default() -> Self {
        Alpha::new(C::default(), A::ch_max())
    }
}

impl<C, A> AsRef<C> for Alpha<C, A> {
    fn as_ref(&self) -> &C {
        &self.color
    }
}

impl<C, A> AsMut<C> for Alpha<C, A> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<T: Color, A: Channel> From<T> for Alpha<T, A> {
    fn from(color: T) -> Self {
        Alpha::new(color, A::ch_max())
    }
}

impl<T: Channel, A: Channel, S> From<(T, T, T, A)> for Alpha<RGBColor<T, S>, A> {
    fn from(tuple: (T, T, T, A)) -> Self {
        Alpha::new(RGBColor::new(tuple.0, tuple.1, tuple.2), tuple.3)
    }
}

impl<T, A, S> From<&(T, T, T, A)> for Alpha<RGBColor<T, S>, A>
    where T: Clone + Channel, A: Clone + Channel
{
    fn from(tuple: &(T, T, T, A)) -> Self {
        let (r, g, b, a) = tuple;
        Alpha::new(RGBColor::new(r.clone(), g.clone(), b.clone()), a.clone())
    }
}

impl<T: Channel + Clone, S> From<[T; 4]> for Alpha<RGBColor<T, S>, T> {
    fn from(array: [T; 4]) -> Self {
        let f = |n: usize| array[n].clone();
        Alpha::new(RGBColor::new(f(0), f(1), f(2)), f(3))
    }
}

impl<T: Channel + Clone, S> From<&[T; 4]> for Alpha<RGBColor<T, S>, T> {
    fn from(slice: &[T; 4]) -> Self {
        let f = |n: usize| slice[n].clone();
        Alpha::new(RGBColor::new(f(0), f(1), f(2)), f(3))
    }
}

impl<C: fmt::UpperHex> fmt::UpperHex for Alpha<C, u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}{:02X}", self.color, self.alpha)
    }
}

impl<C: fmt::LowerHex> fmt::LowerHex for Alpha<C, u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}{:02x}", self.color, self.alpha)
    }
}
