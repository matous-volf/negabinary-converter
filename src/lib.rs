#[cfg(test)]
mod tests;

use std::cmp::max;
use std::fmt::Display;

pub struct Decomposition {
    bits: Vec<bool>,
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

        (self.starting_exponent..0)
            .map(|_| &false)
            .chain(&self.bits)
            .chain((0..(self.starting_exponent - self.bits.len() as i32 + 1)).map(|_| &false))
            .enumerate()
            .map(|(index, &bit)| (max(0, self.starting_exponent) - index as i32, bit))
            .for_each(|(exponent, bit)| {
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
        let opposite_sign_sum = (-2f64).powi(exponent - 1) * 4f64 / 3f64;

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

fn find_starting_exponent(target: f64) -> i32 {
    let mut starting_exponent = target.abs().log2().ceil() as i32;

    if (target < 0_f64 && starting_exponent % 2 == 0)
        || (target > 0_f64 && starting_exponent % 2 != 0) {
        starting_exponent += 1;
    }

    starting_exponent
}
