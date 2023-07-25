use std::collections::HashMap;
use indicatif::ProgressBar;
use crate::analyzer::{Analyzer, Site};
use crate::info::{AbitKey, Info, SiteKey};

pub struct DropAnalyzer {
    percent: u32,
    range: (u32, u32),
    iters: u32,
}

impl DropAnalyzer {
    pub fn new(percent: u32, range: (u32, u32)) -> Self {
        Self {
            percent,
            range,
            iters: 0,
        }
    }

    pub fn run(&self, info: &Info, times: u32) -> HashMap<SiteKey, f64> {
        let mut results = HashMap::new();

        let bar = ProgressBar::new(times as _);

        for i in 0..times {
            let mut info = info.clone();

            info.drop_percent(self.percent, self.range, None);

            let mut analyzer = Analyzer::new();
            analyzer.mute();
            analyzer.run(&info);

            for (site_key, _) in info.site_capacities() {
                let (points, _) = analyzer.get_site_passing_info(&info, site_key);

                if results.contains_key(site_key) {
                    let current = results.get_mut(site_key).unwrap();

                    *current = ((*current * self.iters as f64) + points as f64) / ((self.iters + 1) as f64);
                } else {
                    results.insert(site_key.clone(), points as f64);
                }
            }

            bar.inc(1);
        }

        results
    }

    pub fn run_personal(&self, info: &Info, abit_key: &AbitKey, times: u32) -> (HashMap<SiteKey, u32>, u32) {
        let mut site_stats = HashMap::new();
        let mut not_admitted_times = 0;

        let bar = ProgressBar::new(times as _);

        for i in 0..times {
            let mut info = info.clone();

            info.drop_percent(self.percent, self.range, Some(abit_key));

            let mut analyzer = Analyzer::new();
            analyzer.mute();
            analyzer.run(&info);

            if let Some(site_key) = analyzer.admission().get(abit_key) {
                if !site_stats.contains_key(site_key) {
                    site_stats.insert(site_key.clone(), 0);
                }

                *site_stats.get_mut(site_key).unwrap() += 1;
            } else {
                not_admitted_times += 1;
            }

            bar.inc(1);
        }

        (site_stats, not_admitted_times)
    }
}