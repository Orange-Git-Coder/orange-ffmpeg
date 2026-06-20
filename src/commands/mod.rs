pub mod aria2;
pub mod audio;
pub mod compress;
pub mod convert;
pub mod download;
pub mod gif;
pub mod info;
pub mod merge;
pub mod play;
pub mod screenshot;
pub mod subtitle;

use crate::state::AppState;
use anyhow::Result;
use std::collections::HashMap;

// ── Command Trait ────────────────────────────────────────────────────

pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn aliases(&self) -> &[&str] {
        &[]
    }
    fn description(&self) -> &str;
    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }
    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()>;
}

// ── Command Registry ─────────────────────────────────────────────────

pub struct CommandRegistry {
    commands: Vec<Box<dyn Command>>,
    index: HashMap<String, usize>, // name → index
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            commands: Vec::new(),
            index: HashMap::new(),
        };
        reg.register_defaults();
        reg
    }

    fn register_defaults(&mut self) {
        self.register(Box::new(download::DownloadCommand));
        self.register(Box::new(info::InfoCommand));
        self.register(Box::new(play::PlayCommand));
        self.register(Box::new(convert::ConvertCommand));
        self.register(Box::new(compress::CompressCommand));
        self.register(Box::new(gif::GifCommand));
        self.register(Box::new(audio::AudioCommand));
        self.register(Box::new(merge::MergeCommand));
        self.register(Box::new(subtitle::SubtitleCommand));
        self.register(Box::new(screenshot::ScreenshotCommand));
        self.register(Box::new(aria2::Aria2Command));
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        let idx = self.commands.len();
        self.index.insert(cmd.name().to_string(), idx);
        for alias in cmd.aliases() {
            self.index.insert(alias.to_string(), idx);
        }
        self.commands.push(cmd);
    }

    pub fn find(&self, name: &str) -> Option<&dyn Command> {
        self.index.get(name).map(|&i| self.commands[i].as_ref())
    }

    pub fn list_all(&self) -> Vec<&dyn Command> {
        self.commands.iter().map(|c| c.as_ref()).collect()
    }

    /// Fuzzy search commands by input
    pub fn fuzzy_search(&self, input: &str) -> Vec<&dyn Command> {
        let lower = input.to_lowercase();
        let mut results: Vec<&dyn Command> = self
            .commands
            .iter()
            .filter(|c| {
                c.name().to_lowercase().contains(&lower)
                    || c.description().to_lowercase().contains(&lower)
                    || c.aliases()
                        .iter()
                        .any(|a| a.to_lowercase().contains(&lower))
            })
            .map(|c| c.as_ref())
            .collect();

        // Sort: exact match first, then prefix match, then substring
        results.sort_by(|a, b| {
            let a_exact = a.name().to_lowercase() == lower;
            let b_exact = b.name().to_lowercase() == lower;
            if a_exact != b_exact {
                return b_exact.cmp(&a_exact);
            }

            let a_pref = a.name().to_lowercase().starts_with(&lower);
            let b_pref = b.name().to_lowercase().starts_with(&lower);
            b_pref.cmp(&a_pref)
        });

        results
    }

    /// Autocomplete: return command names that start with input
    pub fn autocomplete(&self, input: &str) -> Vec<String> {
        let lower = input.to_lowercase();
        let mut results: Vec<String> = self
            .commands
            .iter()
            .filter(|c| c.name().to_lowercase().starts_with(&lower))
            .map(|c| c.name().to_string())
            .collect();
        results.sort();
        if results.len() == 1 {
            // Also include subcommands
            if let Some(cmd) = self.find(&results[0]) {
                for sub in cmd.subcommands() {
                    results.push(sub.name().to_string());
                }
            }
        }
        results
    }
}
