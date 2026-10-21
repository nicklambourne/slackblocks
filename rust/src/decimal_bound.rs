//! Exact ordering for already validated finite decimal strings, without exponent expansion.
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Integer {
    negative: bool,
    digits: String,
}
impl Integer {
    fn new(value: &str) -> Self {
        let digits = value.trim_start_matches(['-', '+']).trim_start_matches('0');
        Self {
            negative: value.starts_with('-') && !digits.is_empty(),
            digits: if digits.is_empty() {
                "0".into()
            } else {
                digits.into()
            },
        }
    }
    fn magnitude(&self, other: &Self) -> Ordering {
        self.digits
            .len()
            .cmp(&other.digits.len())
            .then_with(|| self.digits.cmp(&other.digits))
    }
    fn compare(&self, other: &Self) -> Ordering {
        if self.negative != other.negative {
            return if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        if self.negative {
            self.magnitude(other).reverse()
        } else {
            self.magnitude(other)
        }
    }
    fn add_small(self, offset: i64) -> Self {
        let other = Self::new(&offset.to_string());
        let (large, small) = if self.magnitude(&other).is_lt() {
            (&other, &self)
        } else {
            (&self, &other)
        };
        let add = self.negative == other.negative;
        let mut result = Vec::new();
        let mut carry = 0_i16;
        let mut right = small.digits.bytes().rev();
        for digit in large.digits.bytes().rev() {
            let a = i16::from(digit - b'0');
            let b = i16::from(right.next().unwrap_or(b'0') - b'0');
            let sum = a + if add { b } else { -b } + carry;
            result.push(char::from(b'0' + u8::try_from(sum.rem_euclid(10)).unwrap()));
            carry = sum.div_euclid(10);
        }
        if carry > 0 {
            result.push('1');
        }
        let digits: String = result.into_iter().rev().collect();
        let mut value = Self::new(&digits);
        value.negative = large.negative && value.digits != "0";
        value
    }
}

fn parts(value: &str) -> (i8, String, Integer) {
    let (mantissa, exponent) = value.split_once(['e', 'E']).unwrap_or((value, "0"));
    let unsigned = mantissa.trim_start_matches(['-', '+']);
    let fraction = unsigned
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let all = unsigned.replace('.', "");
    let digits = all.trim_start_matches('0').to_string();
    let sign = if digits.is_empty() {
        0
    } else if mantissa.starts_with('-') {
        -1
    } else {
        1
    };
    let rank = Integer::new(exponent)
        .add_small(i64::try_from(digits.len()).unwrap() - i64::try_from(fraction).unwrap());
    (sign, digits, rank)
}

pub(crate) fn compare(left: &str, right: &str) -> Ordering {
    let (a, x, rank_x) = parts(left);
    let (b, y, rank_y) = parts(right);
    if a != b {
        return a.cmp(&b);
    }
    if a == 0 {
        return Ordering::Equal;
    }
    let rank = rank_x.compare(&rank_y);
    let order = if rank.is_eq() {
        let width = x.len().max(y.len());
        x.bytes()
            .chain(std::iter::repeat(b'0'))
            .take(width)
            .cmp(y.bytes().chain(std::iter::repeat(b'0')).take(width))
    } else {
        rank
    };
    if a < 0 { order.reverse() } else { order }
}

/// Accepts only ASCII signed decimal/scientific strings; no whitespace or nonfinite spellings.
pub(crate) fn valid(value: &str) -> bool {
    let mut parts = value.split(['e', 'E']);
    let mantissa = parts.next().unwrap_or("");
    let mantissa = mantissa.strip_prefix(['+', '-']).unwrap_or(mantissa);
    if let Some(exponent) = parts.next() {
        let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    if parts.next().is_some() {
        return false;
    }
    let mut digit = false;
    let mut dot = false;
    for c in mantissa.bytes() {
        if c.is_ascii_digit() {
            digit = true;
        } else if c == b'.' && !dot {
            dot = true;
        } else {
            return false;
        }
    }
    digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_arithmetic_matches_native_small_integers() {
        for value in [-1001, -100, -10, -1, 0, 1, 9, 99, 1000] {
            for offset in [-1000, -99, -1, 0, 1, 99, 1000] {
                let actual = Integer::new(&value.to_string()).add_small(offset);
                let expected = Integer::new(&(value + offset).to_string());
                assert!(actual == expected, "{value} + {offset}");
            }
        }
    }

    #[test]
    fn long_exponents_are_ordered_without_expansion() {
        let exponent = "9".repeat(1000);
        assert!(compare(&format!("1e{exponent}"), &format!("2e{exponent}")).is_lt());
        assert!(compare(&format!("1e-{exponent}"), &format!("2e-{exponent}")).is_lt());
        assert!(compare(&format!("10e-{exponent}"), &format!("2e-{exponent}")).is_gt());
    }
}
