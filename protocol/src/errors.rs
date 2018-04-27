//! All available errors

/// Contains error message that could be thrown during parsing;
#[derive(Copy, Clone, Debug, Fail)]
pub enum ParseErrors {
    /// Thrown when decrypting the message is not successfull
    #[fail(display = "Error decrypting the message")]
    ErrorDecrypting,
    /// for example thrown when a u64 value gets parsed
    /// but not or too many values are given
    #[fail(display = "Not enough bytes to read")]
    NotEnoughBytes
}
