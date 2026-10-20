fn require_positive_amount(amount: &DecimalValue) -> Result<(), ProtocolError> {
    if !amount.is_positive() {
        return Err(ProtocolError::InvalidAction(
            "transfer amount must be positive",
        ));
    }
    Ok(())
}

fn require_positive_wei(wei: u64) -> Result<(), ProtocolError> {
    require_positive_protocol_integer(wei, "staking amount in wei must be positive")
}

fn require_positive_protocol_integer(
    value: u64,
    message: &'static str,
) -> Result<(), ProtocolError> {
    if value == 0 {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

fn validate_spot_token_identifier(token: &str) -> Result<(), ProtocolError> {
    let Some((name, identifier)) = token.split_once(':') else {
        return Err(ProtocolError::InvalidAction(
            "spot token must use name:identifier format",
        ));
    };
    if name.is_empty()
        || identifier.is_empty()
        || name != name.trim()
        || identifier != identifier.trim()
    {
        return Err(ProtocolError::InvalidAction(
            "spot token name and identifier must not be blank",
        ));
    }
    Ok(())
}

fn validate_asset_token(token: &str) -> Result<(), ProtocolError> {
    if token.is_empty() || token != token.trim() || token.chars().any(char::is_control) {
        return Err(ProtocolError::InvalidAction(
            "spot token must be non-empty and contain no surrounding whitespace or control characters",
        ));
    }
    if let Some((name, identifier)) = token.split_once(':')
        && (name.is_empty()
            || identifier.is_empty()
            || name != name.trim()
            || identifier != identifier.trim())
    {
        return Err(ProtocolError::InvalidAction(
            "spot token name and identifier must not be blank",
        ));
    }
    Ok(())
}

fn validate_dex_name(name: &str) -> Result<(), ProtocolError> {
    if name.trim() != name || name.chars().any(char::is_control) {
        return Err(ProtocolError::InvalidAction(
            "DEX name must not contain surrounding whitespace or control characters",
        ));
    }
    Ok(())
}

fn require_non_empty<T>(values: &[T], message: &'static str) -> Result<(), ProtocolError> {
    if values.is_empty() {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
