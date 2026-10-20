/// Encodes a typed action as a named `MessagePack` map.
///
/// # Errors
///
/// Returns [`CodecError::MessagePack`] when the action cannot be serialized.
pub fn encode_action<T: Serialize>(action: &T) -> Result<Vec<u8>, CodecError> {
    let encoded = rmp_serde::to_vec_named(action).map_err(CodecError::from)?;
    emit(
        LogLevel::Trace,
        "hyper_axis.codec.encode_action",
        format!("action encoded bytes={}", encoded.len()),
    );
    Ok(encoded)
}

/// Computes the exact L1 action hash consumed by the phantom-agent signature.
///
/// Layout, in order:
///
/// 1. named-map `MessagePack` action;
/// 2. eight-byte big-endian nonce;
/// 3. `0x00` for no vault, otherwise `0x01` plus its 20 address bytes;
/// 4. when present, `0x00` plus eight-byte big-endian expiry.
///
/// # Errors
///
/// Returns an error when action encoding fails.
pub fn action_hash<T: Serialize>(
    action: &T,
    vault_address: Option<&EvmAddress>,
    nonce: u64,
    expires_after: Option<u64>,
) -> Result<[u8; 32], CodecError> {
    let mut payload = encode_action(action)?;
    payload.extend_from_slice(&nonce.to_be_bytes());
    match vault_address {
        None => payload.push(0),
        Some(address) => {
            payload.push(1);
            payload.extend_from_slice(address.as_bytes());
        }
    }
    if let Some(expires_after) = expires_after {
        payload.push(0);
        payload.extend_from_slice(&expires_after.to_be_bytes());
    }

    let digest = Keccak256::digest(payload);
    let mut hash = [0_u8; 32];
    hash.copy_from_slice(&digest);
    emit(
        LogLevel::Trace,
        "hyper_axis.codec.action_hash",
        format!(
            "action hash computed vault={} expiry={}",
            vault_address.is_some(),
            expires_after.is_some()
        ),
    );
    Ok(hash)
}

/// Hex-encodes a hash with an EVM-style prefix.
#[must_use]
pub fn prefixed_hex(hash: &[u8]) -> String {
    format!("0x{}", hex::encode(hash))
}

#[cfg(test)]
mod tests;
