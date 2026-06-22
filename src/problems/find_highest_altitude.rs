use crate::problems::Solution;

impl Solution {
    /// There is a biker going on a road trip. The road trip consists of
    /// `n + 1` points at various altitudes. The biker starts his trip on point
    /// `0` with altitude equal `0`.
    ///
    /// You are given an integer array `gain` of length `n` where `gain[i]` is
    /// the net gain in altitude between poitns `i` and `i + 1` for all
    /// `(0 <= i < n)`.
    ///
    /// Return the highest altitude of a point.
    #[allow(unused)]
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut altitudes = vec![0];
        for delta_altitude in gain {
            let new_alt = altitudes[altitudes.len() - 1] + delta_altitude;
            altitudes.push(new_alt);
        }
        return *altitudes
            .iter()
            .reduce(|x: &i32, acc: &i32| {
                if x > acc {
                    return x;
                } else {
                    return acc;
                }
            })
            .unwrap();
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let gain = vec![-5, 1, 5, 0, -7];
    assert_eq!(Solution::largest_altitude(gain), 1);
}

#[test]
fn test_02() {
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    assert_eq!(Solution::largest_altitude(gain), 0);
}
