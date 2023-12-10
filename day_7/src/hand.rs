use crate::rank::{Rank, JOKER};
use itertools::Itertools;
use std::fmt::Display;

const HAND_SIZE: usize = 5;

#[derive(PartialEq, Eq)]
pub struct Hand([Rank; HAND_SIZE]);

impl Hand {
    pub fn get_value(&self) -> u32 {
        let mut joker_count: u32 = 0;

        // count of each card rank
        let mut freq_counts: Vec<u32> = self
            .0
            .iter()
            .sorted()
            .group_by(|r| *r)
            .into_iter()
            .filter_map(|(key, group)| {
                // extract and remove the joker count from this list
                if (*key).eq(&JOKER) {
                    joker_count = group.count() as u32;
                    return None;
                }
                return Some(group.count() as u32);
            })
            .sorted()
            .collect_vec();

        // add the joker count to the highest count of non-joker card ranks is
        if self.0.contains(&JOKER) {
            if let Some(count) = freq_counts.last_mut() {
                *count += joker_count;
            } else {
                // case where all 5 are jokers
                freq_counts.push(joker_count);
            }
        }

        freq_counts.resize(5, 0);

        let freq_counts_arr: [u32; 5] = freq_counts.try_into().expect("build array");

        match freq_counts_arr {
            [5, _, _, _, _] => 7, // five of a kind
            [1, 4, _, _, _] => 6, // four of a kind
            [2, 3, _, _, _] => 5, // full house
            [1, 1, 3, _, _] => 4, // three of a kind
            [1, 2, 2, _, _] => 3, // two pair
            [1, 1, 1, 2, _] => 2, // one pair
            [1, 1, 1, 1, 1] => 1, // high card
            _ => 0,
        }
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        assert!(value.len() == HAND_SIZE);
        Self(
            value
                .chars()
                .map(Rank::from)
                .collect_vec()
                .try_into()
                .expect("build array"),
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_value().partial_cmp(&other.get_value()) {
            Some(std::cmp::Ordering::Equal) => {
                for (r1, r2) in self.0.iter().zip(other.0.iter()) {
                    match r1.cmp(r2) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return Some(ordering),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            otherwise => otherwise,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_value().cmp(&other.get_value()) {
            std::cmp::Ordering::Equal => {
                for (r1, r2) in self.0.iter().zip(other.0.iter()) {
                    match r1.cmp(r2) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            }
            otherwise => otherwise,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|r| r.0).join(""))
    }
}
