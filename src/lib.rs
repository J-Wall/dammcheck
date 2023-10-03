mod alphabets;
mod python_module;
mod quasigroups;

/// Calculates Damm's check digit, given an alphabet
pub fn damm<S: AsRef<str>>(
    s: S,
    alphabet: alphabets::Alphabet,
) -> Result<char, alphabets::EncodingError> {
    alphabet.encode_char(check_value(s, &alphabet)?)
}

/// Calculates the value of Damm's check digit, given an alphabet
pub fn check_value<S: AsRef<str>>(
    s: S,
    alphabet: &alphabets::Alphabet,
) -> Result<u8, alphabets::EncodingError> {
    let s = match alphabet.pad() {
        Some(p) => s.as_ref().trim_end_matches(p),
        None => s.as_ref(),
    };
    s.chars().try_fold(0, |acc, c| {
        Ok(quasigroups::apply(
            acc,
            alphabet.decode_char(c)?,
            alphabet.base(),
        ))
    })
}
