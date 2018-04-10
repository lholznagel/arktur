//! All available errors

/// Contains error message that could be thrown during parsing;
#[derive(Copy, Clone, Debug)]
pub enum ParseErrors {
    /// thown when the checksum does not match
    ChecksumDoNotMatch,
    /// Thrown when decrypting the message is not successfull
    ErrorDecrypting,
    /// Thrown when a message is not encrypted
    NotEncrypted,
    /// for example thrown when a u64 value gets parsed
    /// but not or too many values are given
    NotEnoughBytes
}
