use crate::error::ParseError;
use crate::DIDRange;

pub(crate) fn parse_key_value(
    did_url: &str,
    start: usize,
    end: usize,
) -> Result<(usize, usize, usize), ParseError> {
    // Skip separator
    let key_start = start + 1;

    // Value starts after equal sign
    // No equal sign is an error
    let value_start = did_url[key_start..end]
        .find('=')
        .map(|i| key_start + i + 1)
        .ok_or(ParseError::InvalidDIDURL)?;

    // Empty key or value is an error
    if value_start == key_start || value_start == end {
        return Err(ParseError::InvalidDIDURL);
    }

    // Value ends at end of string or next separator
    let next_pos = did_url[value_start..end]
        .find(|c: char| c == ';' || c == '?' || c == '#' || c == '/' || c == '&')
        .map_or(end, |i| value_start + i);

    Ok((key_start, value_start, next_pos))
}

// TODO: Support relative DID URLs
// TODO: Support tunnel methods
pub(crate) fn parse_did_method_id(
    did_url: &str,
) -> Result<(DIDRange, DIDRange, DIDRange), ParseError> {
    // DID = "did:" method ":" method-specific-id
    let method_start = did_url.find(':').ok_or(ParseError::InvalidDIDURL)?;
    let method_end = did_url[method_start + 1..]
        .find(':')
        .map(|i| i + method_start + 1)
        .ok_or(ParseError::InvalidDIDURL)?;

    // TODO
    // assumed: method-specific-id = 1*idchar
    // actual : method-specific-id = *( *idchar ":" ) 1*idchar
    let id_start = method_end + 1;
    let id_end = did_url[id_start..]
        .find(|c: char| c == ';' || c == '/' || c == '?' || c == '#' || c == '&')
        .map_or(did_url.len(), |i| i + id_start);

    let did = 0..id_end;
    let method = method_start + 1..method_end;
    let id = id_start..id_end;

    // No method-specific-id is an error
    if id.is_empty() {
        return Err(ParseError::InvalidDIDURL);
    }

    Ok((did, method, id))
}

pub(crate) fn parse_path(did_url: &str, current_pos: usize) -> Option<DIDRange> {
    // Path ends with query, fragment, param or end of string
    let path_end = did_url[current_pos..]
        .find(|c: char| c == '?' || c == '#' || c == ';')
        .map_or(did_url.len(), |i| i + current_pos);

    Some(current_pos..path_end)
}