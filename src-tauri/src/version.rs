//! Debian-style version comparison, which opkg and webOS packages follow.

use std::cmp::Ordering;

pub fn compare(a: &str, b: &str) -> Ordering {
    let (ea, ua, ra) = split(a);
    let (eb, ub, rb) = split(b);
    ea.cmp(&eb)
        .then_with(|| verrevcmp(ua, ub))
        .then_with(|| verrevcmp(ra, rb))
}

/// Split `[epoch:]upstream[-revision]`.
fn split(v: &str) -> (u64, &str, &str) {
    let v = v.trim();
    let (epoch, rest) = match v.split_once(':') {
        Some((e, r)) if e.chars().all(|c| c.is_ascii_digit()) => (e.parse().unwrap_or(0), r),
        _ => (0, v),
    };
    match rest.rsplit_once('-') {
        Some((u, r)) => (epoch, u, r),
        None => (epoch, rest, ""),
    }
}

fn order(c: Option<u8>) -> i32 {
    match c {
        None => 0,
        Some(b'~') => -1,
        Some(c) if c.is_ascii_digit() => 0,
        Some(c) if c.is_ascii_alphabetic() => i32::from(c),
        Some(c) => i32::from(c) + 256,
    }
}

/// A port of dpkg's `verrevcmp`.
fn verrevcmp(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let at = |s: &[u8], i: usize| s.get(i).copied();
    let is_digit = |c: Option<u8>| c.is_some_and(|c| c.is_ascii_digit());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        while (i < a.len() && !is_digit(at(a, i))) || (j < b.len() && !is_digit(at(b, j))) {
            let (ac, bc) = (order(at(a, i)), order(at(b, j)));
            if ac != bc {
                return ac.cmp(&bc);
            }
            i += 1;
            j += 1;
        }
        while at(a, i) == Some(b'0') {
            i += 1;
        }
        while at(b, j) == Some(b'0') {
            j += 1;
        }
        let mut first_diff = Ordering::Equal;
        while is_digit(at(a, i)) && is_digit(at(b, j)) {
            if first_diff == Ordering::Equal {
                first_diff = a[i].cmp(&b[j]);
            }
            i += 1;
            j += 1;
        }
        if is_digit(at(a, i)) {
            return Ordering::Greater;
        }
        if is_digit(at(b, j)) {
            return Ordering::Less;
        }
        if first_diff != Ordering::Equal {
            return first_diff;
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::compare;
    use std::cmp::Ordering::*;

    #[test]
    fn numeric_parts_compare_by_value() {
        assert_eq!(compare("1.10.0", "1.9.0"), Greater);
        assert_eq!(compare("0.7.1", "0.7.1"), Equal);
        assert_eq!(compare("1.0", "1.0.1"), Less);
        assert_eq!(compare("01.2", "1.2"), Equal);
    }

    #[test]
    fn tilde_sorts_before_release() {
        assert_eq!(compare("1.0~beta1", "1.0"), Less);
        assert_eq!(compare("1.0~beta2", "1.0~beta1"), Greater);
    }

    #[test]
    fn epoch_and_revision() {
        assert_eq!(compare("1:0.1", "9.9"), Greater);
        assert_eq!(compare("1.0-2", "1.0-10"), Less);
    }

    #[test]
    fn letters_sort_after_nothing() {
        assert_eq!(compare("1.0a", "1.0"), Greater);
    }
}
