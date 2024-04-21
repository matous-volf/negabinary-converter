#[cfg(test)]
mod tests;

use std::cmp::max;
use std::fmt::Display;

pub struct Decomposition {
    bits: Vec<bool>,
    // the exponent of the first stored bit
    starting_exponent: i32,
}

impl Decomposition {
    fn new(starting_exponent: i32) -> Self {
        Decomposition { bits: Vec::new(), starting_exponent }
    }
}

impl Display for Decomposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        // in case the starting exponent is less than 0, prepend zeros
        (self.starting_exponent..0).map(|_| &false)
            // the stored bits
            .chain(&self.bits)
            // in case only bits of exponents greater than 0 are stored, append zeros
            .chain((0..self.starting_exponent - self.bits.len() as i32 + 1).map(|_| &false))
            .enumerate()
            // map the index to the exponent
            .map(|(index, &bit)| (max(0, self.starting_exponent) - index as i32, bit))
            .for_each(|(exponent, bit)| {
                // ignore leading zeros
                if !bit && exponent > 0 && string.is_empty() {
                    return;
                }

                if exponent == -1 {
                    string.push('.');
                }

                string.push(if bit { '1' } else { '0' });
            });

        write!(f, "{}", string)
    }
}

pub fn decompose(target: f64) -> Decomposition {
    let mut exponent = find_starting_exponent(target);
    let mut result = 0f64;
    let mut decomposition = Decomposition::new(exponent);

    loop {
        let result_new = result + (-2f64).powi(exponent);
        /* the sum of all powers of -2 that have
        - lower exponents and
        - opposite sign to
        the current power, i.e. (-2) ^ `exponent`,
        which is the sum of all terms in the geometric progression:
        a1 = (-2) ^ (`exponent` - 1); r = 1/4 */
        let opposite_sign_sum = (-2f64).powi(exponent - 1) * 4f64 / 3f64;

        // whether it is possible to reach the target when including the current power
        let target_possible = if exponent % 2 == 0 {
            result_new + opposite_sign_sum <= target
        } else {
            result_new + opposite_sign_sum >= target
        };

        if target_possible {
            decomposition.bits.push(true);
            result = result_new;
        } else {
            decomposition.bits.push(false);
        }

        if result == target {
            break;
        }

        exponent -= 1;
    }

    decomposition
}

/* the lowest integer to which -2 raised is
   - greater, if the target is positive, or
   - lower, if the target is negative,
   than or equal to the target */
fn find_starting_exponent(target: f64) -> i32 {
    if target == 0f64 { 
        return 0;
    }
    
    // the absolute value is needed in order to calculate the logarithm
    let mut starting_exponent = target.abs().log2().ceil() as i32;

    // tweak the exponent in case the power's sign doesn't match the target's
    if (target < 0_f64 && starting_exponent % 2 == 0)
        || (target > 0_f64 && starting_exponent % 2 != 0) {
        starting_exponent += 1;
    }

    starting_exponent
}
