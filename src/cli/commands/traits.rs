use crate::config::Config;
use crate::Result;

use clap::ArgMatches;

use pastel::Color;

pub trait GenericCommand {
    fn run(&self, matches: &ArgMatches, config: &Config) -> Result<()>;
}

pub trait ColorCommand {
    fn run(&self, matches: &ArgMatches, config: &Config, color: &Color) -> Result<()>;
}