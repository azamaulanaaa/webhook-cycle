use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    println!("Hello, world!");

    Ok(())
}
