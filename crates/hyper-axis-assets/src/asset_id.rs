/// Constructs an asset ID for a regular perpetual market.
#[must_use]
pub const fn regular_perp(index_in_meta: u32) -> AssetId {
    AssetId::new(index_in_meta)
}

/// Constructs an asset ID for a spot market.
///
/// # Errors
///
/// Returns [`ProtocolError::AssetIdOverflow`] when adding the spot offset
/// exceeds the wire integer range.
pub fn spot(spot_index: u32) -> Result<AssetId, ProtocolError> {
    SPOT_OFFSET
        .checked_add(spot_index)
        .map(AssetId::new)
        .ok_or(ProtocolError::AssetIdOverflow {
            kind: "spot",
            index: spot_index,
        })
}

/// Constructs an asset ID for a HIP-3 perpetual market.
///
/// # Errors
///
/// Returns [`ProtocolError::AssetIdOverflow`] when the DEX and market indexes
/// cannot be represented by the protocol formula.
pub fn hip3(perp_dex_index: u32, index_in_meta: u32) -> Result<AssetId, ProtocolError> {
    perp_dex_index
        .checked_mul(HIP3_DEX_STRIDE)
        .and_then(|offset| HIP3_OFFSET.checked_add(offset))
        .and_then(|offset| offset.checked_add(index_in_meta))
        .map(AssetId::new)
        .ok_or(ProtocolError::AssetIdOverflow {
            kind: "HIP-3",
            index: index_in_meta,
        })
}
