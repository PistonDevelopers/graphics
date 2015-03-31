//! Helper functions for computing modular index safely.

/// Computes modular offset safely for indices.
#[inline(always)]
pub fn offset(n: usize, i: usize, off: isize) -> usize {
    (i + (off % n as isize + n as isize) as usize) % n
}

/// Computes previous modular index safely.
#[inline(always)]
pub fn previous(n: usize, i: usize) -> usize {
    offset(n, i, -1)
}

/// Computes next modular index safely.
#[inline(always)]
pub fn next(n: usize, i: usize) -> usize {
    offset(n, i, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset() {
        assert_eq!(offset(3, 0, -1), 2);
        assert_eq!(offset(3, 1, -1), 0);
        assert_eq!(offset(3, 2, -1), 1);
        assert_eq!(offset(3, 3, -1), 2);

        assert_eq!(offset(3, 0, 1), 1);
        assert_eq!(offset(3, 1, 1), 2);
        assert_eq!(offset(3, 2, 1), 0);
        assert_eq!(offset(3, 3, 1), 1);
    }
}
