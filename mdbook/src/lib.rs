use mdbook::MDBook;
use mdbook::errors::*;

pub fn build() -> Result<()> {
    MDBook::load(".").map_err(|e| Error::from(e))?.build()
}
