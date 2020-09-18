use super::bitsets::all_bitsets;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    pub objects: Vec<Table>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum Table {
    #[serde(alias = "alliances")]
    Alliances { rows: Vec<Alliance> },
    #[serde(alias = "attributes")]
    Attributes { rows: Vec<Attribute> },
    #[serde(alias = "heroes")]
    Heroes { rows: Vec<Hero> },
    #[serde(alias = "tiers")]
    Tiers { rows: Vec<Tier> },
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type")]
pub struct Alliance {
    pub id: String,
    pub name: String,
    pub modulo: i32,
    pub max: i32,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type")]
pub struct Attribute {
    pub hero_id: String,
    pub alliance_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type")]
pub struct Hero {
    pub id: String,
    pub name: String,
}
#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type")]
pub struct Tier {
    pub hero_id: String,
    pub tier: i32,
}

pub struct Game {
    pub heroes: Vec<Hero>,
    pub alliances: Vec<Alliance>,
    pub tiers: Vec<Tier>,
    pub attributes: Vec<Attribute>,
    pub bitsets: Vec<(u32, u64)>,
    pub lock_hero_bitset: u64,
    pub query_hero_bitset: u64,
    pub query_count: usize,
}

impl Game {
    pub fn new() -> Self {
        let data: Data = serde_json::from_str(include_str!("dota.json")).unwrap();
        let mut heroes = vec![];
        let mut alliances = vec![];
        let mut attributes = vec![];
        let mut tiers = vec![];
        let bitsets = all_bitsets();
        let lock_hero_bitset = 0;
        let query_hero_bitset = !0;
        let query_count = 0;
        for table in data.objects {
            match table {
                Table::Alliances { mut rows } => {
                    alliances.append(&mut rows);
                    alliances.sort_unstable();
                }
                Table::Attributes { mut rows } => {
                    attributes.append(&mut rows);
                }
                Table::Heroes { mut rows } => {
                    heroes.append(&mut rows);
                    heroes.sort_unstable();
                }
                Table::Tiers { mut rows } => {
                    tiers.append(&mut rows);
                    tiers.sort_unstable();
                }
            }
        }
        Game {
            heroes,
            alliances,
            attributes,
            tiers,
            bitsets,
            lock_hero_bitset,
            query_hero_bitset,
            query_count,
        }
    }

    pub fn init_hero(&mut self) {
        let (query_hero_bitset, query_count) = self.query();
        self.query_hero_bitset = query_hero_bitset;
        self.query_count = query_count;
    }

    pub fn toggle_hero(&mut self, i: usize) {
        if self.hero_locked(i) {
            self.lock_hero_bitset &= !(1 << i);
        } else {
            self.lock_hero_bitset |= 1 << i;
        }
        let (query_hero_bitset, query_count) = self.query();
        self.query_hero_bitset = query_hero_bitset;
        self.query_count = query_count;
    }

    pub fn hero_locked(&self, i: usize) -> bool {
        self.lock_hero_bitset & (1 << i) != 0
    }

    pub fn hero_filtered(&self, i: usize) -> bool {
        self.query_hero_bitset & (1 << i) != 0
    }

    pub fn heroes_by_tier(&self) -> Vec<(i32, usize, Hero, Vec<Alliance>)> {
        let mut res = vec![];
        let n = self.heroes.len();
        for i in 0..n {
            let hero = &self.heroes[i];
            for tier in &self.tiers {
                if hero.id == tier.hero_id {
                    let mut alliances = vec![];
                    for alliance in &self.alliances {
                        for attr in &self.attributes {
                            if hero.id == attr.hero_id && attr.alliance_id == alliance.id {
                                alliances.push(alliance.clone());
                            }
                        }
                    }
                    res.push((tier.tier, i, hero.clone(), alliances));
                }
            }
        }
        res.sort_unstable();
        res
    }

    pub fn query(&self) -> (u64, usize) {
        let mut hero_bitset = 0;
        let mut count = 0;
        for (_, bitset) in &self.bitsets {
            if bitset & self.lock_hero_bitset == self.lock_hero_bitset {
                hero_bitset |= bitset;
                count += 1;
            }
        }
        (hero_bitset, count)
    }
}
