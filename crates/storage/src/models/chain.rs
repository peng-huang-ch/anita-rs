use serde::Deserialize;
use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::{AsRefStr, EnumIs};

#[derive(
    AsRefStr,
    clap::ValueEnum,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    EnumString,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    Deserialize,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    /// Docs on red
    #[strum(ascii_case_insensitive, serialize = "solana", serialize = "sol")]
    SOLANA,
    #[strum(ascii_case_insensitive, serialize = "eth", to_string = "eth")]
    ETH,
    #[strum(disabled)]
    UNKNOWN,
}

#[cfg(test)]
mod tests {
    use super::Chain;

    #[test]
    fn test_from_str() {
        let chain = Chain::try_from("solana").expect("invalid chain");
        assert_eq!(chain, Chain::SOLANA);

        let chain = Chain::try_from("SOLANA").expect("invalid chain");
        assert_eq!(chain, Chain::SOLANA);
    }
}
