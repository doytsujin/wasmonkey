extern crate clap;
extern crate failure;
extern crate wasmonkey;

mod config;

use config::*;
use wasmonkey::*;

fn main() -> Result<(), WError> {
    let config = Config::parse_cmdline()?;
    let patcher = Patcher::from_file(config.patcher_config, config.input_path)?;
    patcher.store_to_file(config.output_path)?;
    Ok(())
}
