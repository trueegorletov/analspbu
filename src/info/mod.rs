use std::cmp::max;
use std::collections::HashMap;
use rand::Rng;

mod loader;

pub use loader::Loader;

pub type AbitKey = String;
pub type SiteKey = String;

#[derive(Clone)]
pub struct Info {
    abits: HashMap<AbitKey, Abit>,
    site_capacities: HashMap<SiteKey, u32>,
}

impl Info {
    fn new() -> Self {
        Self {
            abits: HashMap::new(),
            site_capacities: HashMap::new(),
        }
    }

    fn set_site_capacity(&mut self, key: &SiteKey, value: u32) {
        self.site_capacities.insert(key.clone(), value);
    }

    fn get_abit_mut(&mut self, key: &AbitKey) -> &mut Abit {
        if !self.abits.contains_key(key) {
            self.abits.insert(key.clone(), Abit::new());
        }

        self.abits.get_mut(key).unwrap()
    }

    pub fn get_site_capacity(&self, key: &SiteKey) -> u32 {
        *self.site_capacities.get(key).unwrap()
    }

    pub fn get_abit(&self, key: &AbitKey) -> &Abit {
        self.abits.get(key).unwrap()
    }

    pub fn abits(&self) -> &HashMap<AbitKey, Abit> {
        &self.abits
    }

    pub fn is_abit_registered(&self, key: &AbitKey) -> bool {
        self.abits.contains_key(key)
    }

    pub fn site_capacities(&self) -> &HashMap<SiteKey, u32> {
        &self.site_capacities
    }

    pub fn rescale(&mut self, percent: u32) {
        let k = percent as f64 / 100.;

        for (_, capacity) in &mut self.site_capacities {
            let t = ((*capacity as f64) * k).floor() as u32;
            *capacity = t;
        }
    }

    pub fn drop_percent(&mut self, percent: u32, range: (u32, u32), ignore: Option<&AbitKey>) {
        let mut remove = vec![];

        for (abit_key, abit) in &self.abits {
            if let Some(key) = ignore {
                if *key == *abit_key {
                    continue;
                }
            }

            let score = abit.get_biggest_score();

            if score < range.0 || score > range.1 {
                continue;
            }

            let mut rng = rand::thread_rng();

            let k = 100_000;

            let result = (rng.gen_range(1..=k) as f64 / (k as f64)) * 100.;

            if result <= percent as f64 {
                remove.push(abit_key.clone());
            }
        }

        for key in remove {
            self.abits.remove(&key);
        }
    }
}

#[derive(Clone)]
pub struct Abit {
    priorities: Vec<Option<Priority>>,
    site_scores: HashMap<SiteKey, u32>,
}

impl Abit {
    fn new() -> Self {
        Self {
            priorities: vec![],
            site_scores: HashMap::new(),
        }
    }

    fn add_priority(&mut self, site: &SiteKey, place: u32, priority: u32) {
        if priority == 0 {
            self.priorities.insert(0, Some(Priority::new(site.clone(), place)));
            return;
        }

        let index = priority - 1;

        while self.priorities.len() <= index as usize {
            self.priorities.push(None);
        }

        self.priorities[index as usize] = Some(Priority::new(site.clone(), place));
    }

    pub fn priorities(&self) -> &Vec<Option<Priority>> {
        &self.priorities
    }

    pub fn get_score(&self, site: &SiteKey) -> u32 {
        *self.site_scores.get(site).unwrap()
    }

    pub fn get_biggest_score(&self) -> u32 {
        let mut biggest = 0;

        for (_, score) in &self.site_scores {
            biggest = max(biggest, *score);
        }

        biggest
    }
}

#[derive(Clone)]
pub struct Priority {
    pub site: SiteKey,
    pub place: u32,
}

impl Priority {
    fn new(site: SiteKey, place: u32) -> Self {
        Self {
            site,
            place,
        }
    }
}