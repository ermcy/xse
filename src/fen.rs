use crate::error::XSError;

#[derive(Debug)]
struct FEN;

impl FEN {
    pub fn new() -> Result<Self, XSError> {
        // TODO: implement FEN parsing
        Ok(FEN)
    }
}