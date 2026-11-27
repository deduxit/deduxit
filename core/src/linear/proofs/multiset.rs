// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use crate::linear::occurrences::{Member, OccId};

/// A multiset of occurrence ids or members, kept as an ascending list with
/// repeats. It holds the linear zone of a sequent, where copies from the
/// unrestricted zone can repeat an occurrence.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Multiset<T = OccId>(Vec<T>);

impl<T> Default for Multiset<T> {
    /// The empty multiset.
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: Copy + Ord> Multiset<T> {
    /// Returns the empty multiset.
    #[cfg(any(test, feature = "interactive"))]
    pub(crate) const fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns the multiset of the given ids, in any order.
    pub(crate) fn of(ids: impl IntoIterator<Item = T>) -> Self {
        let mut ids: Vec<T> = ids.into_iter().collect();
        ids.sort_unstable();
        Self(ids)
    }

    /// Returns the members in ascending order, with repeats.
    pub(crate) fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Returns the members, ascending with repeats, in the form an
    /// inference's sequent holds them.
    pub(crate) fn into_members(self) -> Vec<Member>
    where
        T: Into<Member>,
    {
        self.0.into_iter().map(Into::into).collect()
    }

    /// Returns whether the multiset has no member.
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the position of the first copy of `o`, if it is a member.
    pub(crate) fn position(&self, o: T) -> Option<usize> {
        let i = self.0.partition_point(|&x| x < o);
        (self.0.get(i) == Some(&o)).then_some(i)
    }

    /// Returns whether `o` is a member.
    pub(crate) fn contains(&self, o: T) -> bool {
        self.position(o).is_some()
    }

    /// Adds one copy of `o`.
    pub(crate) fn insert(&mut self, o: T) {
        insert(&mut self.0, o);
    }

    /// Adds `o` if it is not a member.
    pub(crate) fn ensure(&mut self, o: T) {
        if !self.contains(o) {
            self.insert(o);
        }
    }

    /// Removes one copy of `o` and returns whether there was one.
    pub(crate) fn remove(&mut self, o: T) -> bool {
        remove(&mut self.0, o)
    }

    /// Returns the multiset that holds each id `f(a, b)` times, where `a`
    /// and `b` are its counts here and in `other`.
    #[cfg(test)]
    fn merge(&self, other: &Self, f: impl Fn(usize, usize) -> usize) -> Self {
        let (a, b) = (&self.0, &other.0);
        let mut out = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() || j < b.len() {
            let o = match (a.get(i), b.get(j)) {
                (Some(&x), Some(&y)) => x.min(y),
                (Some(&x), None) => x,
                (None, Some(&y)) => y,
                (None, None) => unreachable!(),
            };
            let run = |v: &[T], k: usize| v[k..].iter().take_while(|&&x| x == o).count();
            let (ca, cb) = (run(a, i), run(b, j));
            out.extend(std::iter::repeat_n(o, f(ca, cb)));
            i += ca;
            j += cb;
        }
        Self(out)
    }

    /// Returns the multiset sum: the counts add up.
    pub(crate) fn sum(&self, other: &Self) -> Self {
        Self(merged(&self.0, &other.0))
    }

    /// Returns the multiset union: per id, the larger count.
    #[cfg(test)]
    pub(crate) fn union(&self, other: &Self) -> Self {
        self.merge(other, usize::max)
    }

    /// Returns the multiset difference: per id, this count minus the
    /// other's, or zero if that is negative.
    pub(crate) fn difference(&self, other: &Self) -> Self {
        Self(excess(&self.0, &other.0))
    }

    /// Returns whether no id has more copies here than in `other`.
    pub(crate) fn is_subset(&self, other: &Self) -> bool {
        within(&self.0, &other.0)
    }
}

/// Inserts `o` into a sorted list, keeping repeats.
pub(crate) fn insert<T: Copy + Ord>(list: &mut Vec<T>, o: T) {
    let at = list.partition_point(|&x| x <= o);
    list.insert(at, o);
}

/// Removes one copy of `o` from a sorted list, and returns whether there
/// was one.
pub(crate) fn remove<T: Copy + Ord>(list: &mut Vec<T>, o: T) -> bool {
    let at = list.partition_point(|&x| x < o);
    if list.get(at) == Some(&o) {
        list.remove(at);
        true
    } else {
        false
    }
}

/// Merges `from` into `into`, both sorted, keeping repeats: the sum of two
/// multisets.
pub(crate) fn add<T: Copy + Ord>(into: &mut Vec<T>, from: &[T]) {
    if !from.is_empty() {
        *into = merged(into, from);
    }
}

/// Returns the sum of the sorted multisets `a` and `b`, sorted.
fn merged<T: Copy + Ord>(a: &[T], b: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);
    merged
}

/// Returns the members of `of` beyond those of `within`, with repeats:
/// what `within` lacks to hold `of`. Both are sorted multisets.
pub(crate) fn excess<T: Copy + Ord>(of: &[T], within: &[T]) -> Vec<T> {
    let mut out = Vec::new();
    let mut j = 0;
    for &o in of {
        while j < within.len() && within[j] < o {
            j += 1;
        }
        if j < within.len() && within[j] == o {
            j += 1;
        } else {
            out.push(o);
        }
    }
    out
}

/// Returns whether the sorted multiset `small` lies within the sorted
/// multiset `large`.
pub(crate) fn within<T: Copy + Ord>(small: &[T], large: &[T]) -> bool {
    let mut j = 0;
    for &o in small {
        while j < large.len() && large[j] < o {
            j += 1;
        }
        if j == large.len() || large[j] != o {
            return false;
        }
        j += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Wraps a raw id.
    const fn o(id: u32) -> OccId {
        OccId::new(id)
    }

    /// Members are kept sorted with repeats, and every operation counts
    /// the copies of each id.
    #[test]
    fn counts() {
        let mut m = Multiset::of([o(5), o(2), o(5)]);
        assert_eq!(m.as_slice(), [o(2), o(5), o(5)]);
        assert_eq!(m.position(o(5)), Some(1));
        assert_eq!(m.position(o(3)), None);
        m.insert(o(3));
        m.ensure(o(3));
        m.ensure(o(9));
        assert_eq!(m.as_slice(), [o(2), o(3), o(5), o(5), o(9)]);
        assert!(m.remove(o(5)));
        assert!(m.remove(o(5)));
        assert!(!m.remove(o(5)));
        assert_eq!(m.as_slice(), [o(2), o(3), o(9)]);

        let a = Multiset::of([o(1), o(1), o(2)]);
        let b = Multiset::of([o(1), o(2), o(2), o(4)]);
        assert_eq!(
            a.sum(&b).as_slice(),
            [o(1), o(1), o(1), o(2), o(2), o(2), o(4)]
        );
        assert_eq!(a.union(&b).as_slice(), [o(1), o(1), o(2), o(2), o(4)]);
        assert_eq!(a.difference(&b).as_slice(), [o(1)]);
        assert_eq!(b.difference(&a).as_slice(), [o(2), o(4)]);
        assert!(!a.is_subset(&b));
        assert!(a.difference(&b).is_subset(&a));
        assert!(Multiset::<OccId>::new().is_subset(&a));
        assert!(Multiset::<OccId>::new().is_empty());
    }
}
