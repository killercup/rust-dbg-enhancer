use snafu::{ResultExt, Snafu};
use std::{fmt, io::Read};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .context(CouldNotRead)?;
    let res = rust_dbg_enhancer::enhance(&input).context(CouldNotEnhance)?;
    println!("{}", res);
    Ok(())
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("Could not read stdin: {}", source))]
    CouldNotRead { source: std::io::Error },
    #[snafu(display("Could not enhance input: {}", source))]
    CouldNotEnhance { source: rust_dbg_enhancer::Error },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(backtrace) = snafu::ErrorCompat::backtrace(&self) {
            writeln!(f, "{}", backtrace)?;
        }
        Ok(())
    }
}
