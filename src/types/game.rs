// SPDX-FileCopyrightText: 2024 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-2.0-only

use anyhow::{anyhow, bail, Result};
use fs_extra::dir::get_size;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Clone)]
pub struct Game {
    pub dir: PathBuf,
    pub size: u64,
    pub id: String,
    pub title: String,
    pub display_title: String,
    pub checked: bool,
}

impl Game {
    pub fn new(path: PathBuf, titles: &HashMap<String, String>) -> Result<Self> {
        let dir = path.to_owned();

        let re = regex!(r"(.+)\[(.+)\]");

        let Some(caps) = re.captures(path.file_name().unwrap().to_str().unwrap()) else {
            bail!("Invalid dir");
        };

        let title = &caps[1];
        let id = &caps[2];

        let display_title = titles
            .get(id)
            .ok_or_else(|| anyhow!("No title found for id {}", id))?
            .clone();

        let size = get_size(&path).unwrap();

        Ok(Self {
            dir,
            id: id.to_owned(),
            title: title.to_owned(),
            size,
            display_title,
            checked: false,
        })
    }

    pub fn delete(&self) -> Result<()> {
        // idk why but i need to call this twice
        let _ = fs::remove_dir_all(&self.dir);
        let _ = fs::remove_dir_all(&self.dir);

        Ok(())
    }
}
