use std::collections::HashMap;
use indicatif::ProgressBar;
use crate::info::{AbitKey, Info, SiteKey};

pub struct Analyzer {
    sites: HashMap<SiteKey, Site>,
    admission: HashMap<AbitKey, SiteKey>,
    muted: bool,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            sites: HashMap::new(),
            admission: HashMap::new(),
            muted: false,
        }
    }

    pub fn run(&mut self, info: &Info) {
        self.sites.clear();
        self.admission.clear();

        if self.muted {
            for (abit_key, _) in info.abits() {
                self.admit(info, abit_key);
            }
        } else {
            println!("Handling abiturients...");
            let bar = ProgressBar::new(info.abits().len() as u64);
            for (abit_key, _) in info.abits() {
                self.admit(info, abit_key);

                bar.inc(1);
            }
            bar.finish();
        }
    }

    fn admit(&mut self, info: &Info, abit_key: &AbitKey) {
        let abit = info.get_abit(abit_key);

        for priority in abit.priorities() {
            if priority.is_none() {
                continue;
            }

            let priority = priority.as_ref().unwrap();

            let site = self.get_site(info, &priority.site);
            let (admitted, dropout) = site.admit(abit_key, priority.place);

            if admitted {
                self.admission.insert(abit_key.clone(), priority.site.clone());

                if let Some(key) = dropout {
                    self.admission.remove(&key);
                    self.admit(info, &key);
                }

                return;
            }
        }
    }

    fn get_site(&mut self, info: &Info, key: &SiteKey) -> &mut Site {
        if !self.sites.contains_key(key) {
            self.sites.insert(key.clone(), Site::new(info.get_site_capacity(key)));
        }

        self.sites.get_mut(key).unwrap()
    }

    pub fn admission(&self) -> &HashMap<AbitKey, SiteKey> {
        &self.admission
    }

    pub fn get_site_passing_info(&self, info: &Info, site_key: &SiteKey) -> (u32, u32) {
        let site = self.sites.get(site_key).unwrap();

        if site.abits.is_empty() {
            return (0, 0);
        }

        let (abit_key, place) = &site.abits[site.abits.len() - 1];

        (info.get_abit(abit_key).get_score(site_key), *place)
    }

    pub fn mute(&mut self) {
        self.muted = true;
    }
}

pub struct Site {
    capacity: u32,
    abits: Vec<(AbitKey, u32)>,
}

impl Site {
    fn new(capacity: u32) -> Self {
        Self {
            capacity,
            abits: vec![],
        }
    }

    fn admit(&mut self, abit_key: &AbitKey, abit_place: u32) -> (bool, Option<AbitKey>) {
        for i in 0..self.abits.len() {
            let (_key, place) = &self.abits[i];

            if abit_place < *place {
                self.abits.insert(i, (abit_key.clone(), abit_place));

                return (true, if self.abits.len() > self.capacity as usize {
                    Some(self.abits.pop().unwrap().0)
                } else {
                    None
                });
            }
        }

        if self.abits.len() < self.capacity as usize {
            self.abits.push((abit_key.clone(), abit_place));
            return (true, None);
        }

        (false, None)
    }
}