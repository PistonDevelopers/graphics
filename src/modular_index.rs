
//! Helper functions for computing modular index safely.

/// Computes modular offset safely for indices.
#[inline(always)]
pub fn offset(n: uint, i: uint, off: int) -> uint {
    (i + (off % n as int + n as int) as uint) % n
}

/// Computes previous modular index safely.
#[inline(always)]
pub fn previous(n: uint, i: uint) -> uint {
    offset(n, i, -1)
}

/// Computes next modular index safely.
#[inline(always)]
pub fn next(n: uint, i: uint) -> uint {
    offset(n, i, 1)
}

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

