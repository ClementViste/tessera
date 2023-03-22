use std::{
    error::Error,
    fmt::{Formatter, Result},
};

/// Iterates over the whole chain of errors that led to the failure.
pub fn error_chain_fmt(e: &impl Error, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "{e}\n")?;

    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }

    Ok(())
}
