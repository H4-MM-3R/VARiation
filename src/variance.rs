use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub variance: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Variance {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        variance: HashMap::new(),
    };
}

impl Variance {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.pwd.as_path());
        let mut paths = vec![];
        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent()
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.variance.get(path) {
                out.extend(map.iter());
            }
        }
        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(map) = self.data.variance.get(p) {
                if let Some(val) = map.get(key) {
                    out = Some(val);
                    break;
                }
            }
            curr = p.parent();
        }
        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .variance
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.variance.get_mut(&self.pwd).map(|x| {
            x.remove(key);
        });
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }
        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;

        return Ok(());
    }

    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"projector\":{}}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data {
                variance: HashMap::new(),
            });

            return Variance { config, pwd, data };
        }
        return Variance {
            config,
            pwd,
            data: default_data(),
        };
    }
}
