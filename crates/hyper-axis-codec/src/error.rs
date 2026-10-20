/// Codec failures that must be surfaced before a signing request.
#[derive(Debug, Error)]
pub enum CodecError {
    /// `MessagePack` encoding failed.
    #[error("unable to encode action as MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
}
