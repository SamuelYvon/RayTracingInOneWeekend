/// A similar structure to a range, but provides utilities for
/// inclusive or exclusive checks at-will.
#[derive(Clone, Copy)]
pub struct Interval<T: PartialOrd + Copy + Clone>(T, T);

impl<T: PartialOrd + Copy + Clone> Interval<T> {
    pub fn new(low: T, high: T) -> Self {
        let l = if low < high { low } else { high };
        let h = if high > low { high } else { low };

        Self(l, h)
    }

    pub fn low(&self) -> T {
        self.0
    }

    pub fn high(&self) -> T {
        self.1
    }

    /// Check if the value [v] is within the range. The [inclusive] parameter
    /// controls if the check should include the bounds or not.
    pub fn contains(&self, v: T, inclusive: bool) -> bool {
        let (l, h) = (self.0, self.1);

        if inclusive {
            l <= v && v <= h
        } else {
            l < v && v < h
        }
    }
}
