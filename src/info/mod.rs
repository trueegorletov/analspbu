use std::collections::HashMap;

mod loader;

pub use loader::Loader;

pub type AbitKey = String;
pub type SiteKey = String;

pub struct Info {
    abits: HashMap<AbitKey, Abit>,
    site_capacities: HashMap<SiteKey, usize>,
}

impl Info {
    fn new() -> Self {
        Self {
            abits: HashMap::new(),
            site_capacities: HashMap::new(),
        }
    }

    fn set_site_capacity(&mut self, key: &SiteKey, value: usize) {
        self.site_capacities.insert(key.clone(), value);
    }

    fn get_abit_mut(&mut self, key: &AbitKey) -> &mut Abit {
        if !self.abits.contains_key(key) {
            self.abits.insert(key.clone(), Abit::new());
        }

        self.abits.get_mut(key).unwrap()
    }

    pub fn get_site_capacity(&self, key: &SiteKey) -> usize {
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

    pub fn site_capacities(&self) -> &HashMap<SiteKey, usize> {
        &self.site_capacities
    }
}

pub struct Abit {
    priorities: Vec<Option<Priority>>,
    site_scores: HashMap<SiteKey, usize>,
}

impl Abit {
    fn new() -> Self {
        Self {
            priorities: vec![],
            site_scores: HashMap::new(),
        }
    }

    fn add_priority(&mut self, site: &SiteKey, place: usize, priority: usize) {
        if priority == 0 {
            self.priorities.insert(0, Some(Priority::new(site.clone(), place)));
            return;
        }

        let index = priority - 1;

        while self.priorities.len() <= index {
            self.priorities.push(None);
        }

        self.priorities[index] = Some(Priority::new(site.clone(), place));
    }

    pub fn priorities(&self) -> &Vec<Option<Priority>> {
        &self.priorities
    }

    pub fn get_score(&self, site: &SiteKey) -> usize {
        *self.site_scores.get(site).unwrap()
    }
}

pub struct Priority {
    pub site: SiteKey,
    pub place: usize,
}

impl Priority {
    fn new(site: SiteKey, place: usize) -> Self {
        Self {
            site,
            place,
        }
    }
}