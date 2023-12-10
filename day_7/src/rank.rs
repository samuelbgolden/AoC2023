use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Display;

static RANK_STR: &str = "J23456789TQKA";
static RANK_MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    HashMap::<char, u32>::from_iter(RANK_STR.char_indices().map(|(i, c)| (c, i as u32)))
});

pub const JOKER: Lazy<Rank> = Lazy::new(|| Rank::from('J'));

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Rank(pub char);

impl From<char> for Rank {
    fn from(value: char) -> Self {
        assert!(RANK_STR.contains(value));
        Self(value)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        RANK_MAP
            .get(&self.0)
            .expect("rank value")
            .partial_cmp(&RANK_MAP.get(&other.0).expect("rank value"))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        RANK_MAP
            .get(&self.0)
            .expect("rank value")
            .cmp(&RANK_MAP.get(&other.0).expect("rank value"))
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
