use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::definition::Loader;
use crate::config::Config;

pub struct FileLoader {
    pub config: Config,
}

impl FileLoader {
  pub fn new(language: String, directory: String) -> Self {
    let config = Config {
      language,
      directory,
      max_sentences_per_text: usize::MAX,
      file_prefix: String::from(""),
    };

    Self { config }
  }
}

impl Loader for FileLoader {
  fn get_config(&self) -> &Config {
      &self.config
  }

  fn load(&self, file_name: &Path, _filtered_titles: &HashSet<String>) -> Result<Vec<String>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("{}", e))?;
    let mut all_sentences = String::new();
    file.read_to_string(&mut all_sentences)
        .map_err(|e| format!("{}", e))?;
    Ok(all_sentences
        .lines()
        .map(String::from)
        .collect())
  }
}
