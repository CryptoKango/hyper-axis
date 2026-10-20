#[derive(Clone, Copy)]
enum UserField<'a> {
    Text(&'a str),
    Address(&'a EvmAddress),
    Bytes32(&'a [u8; 32]),
    Bool(bool),
    Uint64(u64),
}

fn multi_sig_user_action_digest(
    primary_type: &str,
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    action_fields: &[UserField<'_>],
) -> [u8; 32] {
    let enriched_type = primary_type.replacen(
        "string hyperliquidChain,",
        "string hyperliquidChain,address payloadMultiSigUser,address outerSigner,",
        1,
    );
    let mut fields = Vec::with_capacity(action_fields.len() + 3);
    fields.push(UserField::Text(network.hyperliquid_chain()));
    fields.push(UserField::Address(payload_multi_sig_user));
    fields.push(UserField::Address(outer_signer));
    fields.extend_from_slice(action_fields);
    user_action_digest(&enriched_type, &fields)
}

fn user_action_digest(primary_type: &str, fields: &[UserField<'_>]) -> [u8; 32] {
    let mut encoded = Vec::with_capacity(32 * (fields.len() + 1));
    encoded.extend_from_slice(&keccak(primary_type.as_bytes()));
    for field in fields {
        match field {
            UserField::Text(value) => encoded.extend_from_slice(&keccak(value.as_bytes())),
            UserField::Address(value) => {
                encoded.extend_from_slice(&[0_u8; 12]);
                encoded.extend_from_slice(value.as_bytes());
            }
            UserField::Bytes32(value) => encoded.extend_from_slice(*value),
            UserField::Bool(value) => encoded.extend_from_slice(&uint256(u64::from(*value))),
            UserField::Uint64(value) => encoded.extend_from_slice(&uint256(*value)),
        }
    }
    eip712_digest(user_domain_separator(), keccak(&encoded))
}

fn user_domain_separator() -> [u8; 32] {
    let domain_type = keccak(
        b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
    );
    let mut domain = Vec::with_capacity(160);
    domain.extend_from_slice(&domain_type);
    domain.extend_from_slice(&keccak(b"HyperliquidSignTransaction"));
    domain.extend_from_slice(&keccak(b"1"));
    domain.extend_from_slice(&uint256(USER_SIGNATURE_CHAIN_ID));
    domain.extend_from_slice(&[0_u8; 32]);
    keccak(&domain)
}

fn eip712_digest(domain_separator: [u8; 32], struct_hash: [u8; 32]) -> [u8; 32] {
    let mut envelope = Vec::with_capacity(66);
    envelope.extend_from_slice(b"\x19\x01");
    envelope.extend_from_slice(&domain_separator);
    envelope.extend_from_slice(&struct_hash);
    keccak(&envelope)
}

/// Returns the domain used by user-signed actions such as withdrawals.
#[must_use]
pub const fn user_signed_domain() -> SigningDomain {
    SigningDomain {
        name: "HyperliquidSignTransaction",
        version: "1",
        chain_id: USER_SIGNATURE_CHAIN_ID,
        verifying_contract: ZERO_ADDRESS,
    }
}

fn decode_scalar(field: &'static str, value: &str) -> Result<[u8; 32], SigningError> {
    let unprefixed = value.strip_prefix("0x").unwrap_or(value);
    if unprefixed.len() != 64 {
        return Err(SigningError::InvalidScalar {
            field,
            reason: "expected exactly 32 hexadecimal bytes",
        });
    }
    let decoded = hex::decode(unprefixed).map_err(|_| SigningError::InvalidScalar {
        field,
        reason: "contains non-hexadecimal characters",
    })?;
    decoded.try_into().map_err(|_| SigningError::InvalidScalar {
        field,
        reason: "expected exactly 32 hexadecimal bytes",
    })
}

fn signature_from_components(r: [u8; 32], s: [u8; 32]) -> Result<Signature, SigningError> {
    let mut bytes = [0_u8; 64];
    bytes[..32].copy_from_slice(&r);
    bytes[32..].copy_from_slice(&s);
    Signature::from_slice(&bytes).map_err(|_| SigningError::InvalidSignature)
}

const fn normalize_v(v: u8) -> Result<u8, SigningError> {
    match v {
        0 | 1 => Ok(v + 27),
        27 | 28 => Ok(v),
        _ => Err(SigningError::InvalidV(v)),
    }
}

fn address_from_verifying_key(verifying_key: &VerifyingKey) -> EvmAddress {
    let encoded = verifying_key.to_sec1_point(false);
    let digest = Keccak256::digest(&encoded.as_bytes()[1..]);
    let mut address = [0_u8; 20];
    address.copy_from_slice(&digest[12..]);
    EvmAddress::from_bytes(address)
}

fn keccak(value: &[u8]) -> [u8; 32] {
    let digest = Keccak256::digest(value);
    let mut output = [0_u8; 32];
    output.copy_from_slice(&digest);
    output
}

fn uint256(value: u64) -> [u8; 32] {
    let mut output = [0_u8; 32];
    output[24..].copy_from_slice(&value.to_be_bytes());
    output
}

#[cfg(test)]
mod tests;
