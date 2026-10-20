/// Computes the EIP-712 digest a host signer signs for an L1 exchange action.
///
/// This is the `keccak256(0x1901 || domainSeparator || agentStructHash)` value.
/// The host should sign these 32 bytes with secp256k1 and must not hash them a
/// second time.
#[must_use]
pub fn l1_signing_digest(network: Network, connection_id: [u8; 32]) -> [u8; 32] {
    let domain_type = keccak(
        b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
    );
    let agent_type = keccak(b"Agent(string source,bytes32 connectionId)");

    let mut domain = Vec::with_capacity(160);
    domain.extend_from_slice(&domain_type);
    domain.extend_from_slice(&keccak(b"Exchange"));
    domain.extend_from_slice(&keccak(b"1"));
    domain.extend_from_slice(&uint256(L1_SIGNATURE_CHAIN_ID));
    domain.extend_from_slice(&[0_u8; 32]);
    let domain_separator = keccak(&domain);

    let mut agent = Vec::with_capacity(96);
    agent.extend_from_slice(&agent_type);
    agent.extend_from_slice(&keccak(network.l1_source().as_bytes()));
    agent.extend_from_slice(&connection_id);
    let struct_hash = keccak(&agent);

    let mut envelope = Vec::with_capacity(66);
    envelope.extend_from_slice(b"\x19\x01");
    envelope.extend_from_slice(&domain_separator);
    envelope.extend_from_slice(&struct_hash);
    keccak(&envelope)
}
