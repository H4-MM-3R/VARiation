use anyhow::Result;
use clap::Parser;
use variation::{
    config::{Config, Operation},
    opts::Opts,
    variance::Variance,
};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    // println!("{:?}", config);

    let mut proj = Variance::from_config(config.config, config.pwd);

    match config.operation {
        // If no arguments print all the key, value pairs
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string_pretty(&value)?;
            println!("{}", value);
        }

        // print value based on the key
        Operation::Print(Some(key)) => {
            proj.get_value(&key).map(|x| {
                println!("{}", x);
            });
        }

        // add a key, value pair
        Operation::Add(key, value) => {
            proj.set_value(key, value);
            proj.save()?;
        }

        // remove a key
        Operation::Remove(key) => {
            proj.remove_value(&key);
            proj.save()?;
        }
    }

    return Ok(());
}
