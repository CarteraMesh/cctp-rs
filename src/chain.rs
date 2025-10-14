use alloy_chains::NamedChain;
use alloy_primitives::Address;

use crate::error::{CctpError, Result};

use crate::{
    ARBITRUM_DOMAIN_ID, ARBITRUM_MESSAGE_TRANSMITTER_ADDRESS,
    ARBITRUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS, ARBITRUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS,
    ARBITRUM_TOKEN_MESSENGER_ADDRESS, AVALANCHE_DOMAIN_ID, AVALANCHE_MESSAGE_TRANSMITTER_ADDRESS,
    AVALANCHE_TOKEN_MESSENGER_ADDRESS, BASE_DOMAIN_ID, BASE_MESSAGE_TRANSMITTER_ADDRESS,
    BASE_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS, BASE_SEPOLIA_TOKEN_MESSENGER_ADDRESS,
    BASE_TOKEN_MESSENGER_ADDRESS, ETHEREUM_DOMAIN_ID, ETHEREUM_MESSAGE_TRANSMITTER_ADDRESS,
    ETHEREUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS, ETHEREUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS,
    ETHEREUM_TOKEN_MESSENGER_ADDRESS, OPTIMISM_DOMAIN_ID, OPTIMISM_MESSAGE_TRANSMITTER_ADDRESS,
    OPTIMISM_TOKEN_MESSENGER_ADDRESS, POLYGON_CCTP_V1_MESSAGE_TRANSMITTER,
    POLYGON_CCTP_V1_TOKEN_MESSENGER, POLYGON_DOMAIN_ID, UNICHAIN_CCTP_V1_MESSAGE_TRANSMITTER,
    UNICHAIN_CCTP_V1_TOKEN_MESSENGER, UNICHAIN_DOMAIN_ID,
};

/// Trait for chains that support CCTP bridging
pub trait CctpV1 {
    /// The average time to confirmation of the chain, according to the CCTP docs: <https://developers.circle.com/stablecoins/required-block-confirmations>
    fn confirmation_average_time_seconds(&self) -> Result<u64>;
    /// The domain ID of the chain - used to identify the chain when bridging: <https://developers.circle.com/stablecoins/evm-smart-contracts>
    fn cctp_domain_id(&self) -> Result<u32>;
    /// The address of the `TokenMessenger` contract on the chain
    fn token_messenger_address(&self) -> Result<Address>;
    /// The address of the `MessageTransmitter` contract on the chain
    fn message_transmitter_address(&self) -> Result<Address>;

    /// Check if the chain is supported for CCTP
    fn is_supported(&self) -> bool;
}

impl CctpV1 for NamedChain {
    fn confirmation_average_time_seconds(&self) -> Result<u64> {
        use NamedChain::*;

        match self {
            Mainnet | Arbitrum | Base | Optimism | Unichain => Ok(19 * 60),
            Avalanche => Ok(20),
            Polygon => Ok(8 * 60),
            // Testnets
            Sepolia => Ok(60),
            ArbitrumSepolia | AvalancheFuji | BaseSepolia | OptimismSepolia | PolygonAmoy => Ok(20),
            _ => Err(CctpError::ChainNotSupported {
                chain: self.to_string(),
            }),
        }
    }

    fn cctp_domain_id(&self) -> Result<u32> {
        use NamedChain::*;

        match self {
            Arbitrum | ArbitrumSepolia => Ok(ARBITRUM_DOMAIN_ID),
            Avalanche => Ok(AVALANCHE_DOMAIN_ID),
            Base | BaseSepolia => Ok(BASE_DOMAIN_ID),
            Mainnet | Sepolia => Ok(ETHEREUM_DOMAIN_ID),
            Optimism => Ok(OPTIMISM_DOMAIN_ID),
            Polygon => Ok(POLYGON_DOMAIN_ID),
            Unichain => Ok(UNICHAIN_DOMAIN_ID),
            _ => Err(CctpError::ChainNotSupported {
                chain: self.to_string(),
            }),
        }
    }

    fn token_messenger_address(&self) -> Result<Address> {
        use NamedChain::*;

        match self {
            Arbitrum => Ok(ARBITRUM_TOKEN_MESSENGER_ADDRESS),
            ArbitrumSepolia => Ok(ARBITRUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS),
            Avalanche => Ok(AVALANCHE_TOKEN_MESSENGER_ADDRESS),
            Base => Ok(BASE_TOKEN_MESSENGER_ADDRESS),
            BaseSepolia => Ok(BASE_SEPOLIA_TOKEN_MESSENGER_ADDRESS),
            Sepolia => Ok(ETHEREUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS),
            Mainnet => Ok(ETHEREUM_TOKEN_MESSENGER_ADDRESS),
            Optimism => Ok(OPTIMISM_TOKEN_MESSENGER_ADDRESS),
            Polygon => Ok(POLYGON_CCTP_V1_TOKEN_MESSENGER),
            Unichain => Ok(UNICHAIN_CCTP_V1_TOKEN_MESSENGER),
            _ => Err(CctpError::ChainNotSupported {
                chain: self.to_string(),
            }),
        }
    }

    fn message_transmitter_address(&self) -> Result<Address> {
        use NamedChain::*;

        match self {
            Arbitrum => Ok(ARBITRUM_MESSAGE_TRANSMITTER_ADDRESS),
            Avalanche => Ok(AVALANCHE_MESSAGE_TRANSMITTER_ADDRESS),
            Base => Ok(BASE_MESSAGE_TRANSMITTER_ADDRESS),
            Mainnet => Ok(ETHEREUM_MESSAGE_TRANSMITTER_ADDRESS),
            Optimism => Ok(OPTIMISM_MESSAGE_TRANSMITTER_ADDRESS),
            Polygon => Ok(POLYGON_CCTP_V1_MESSAGE_TRANSMITTER),
            // Testnets
            ArbitrumSepolia => Ok(ARBITRUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS),
            BaseSepolia => Ok(BASE_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS),
            Sepolia => Ok(ETHEREUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS),
            Unichain => Ok(UNICHAIN_CCTP_V1_MESSAGE_TRANSMITTER),
            _ => Err(CctpError::ChainNotSupported {
                chain: self.to_string(),
            }),
        }
    }

    fn is_supported(&self) -> bool {
        use NamedChain::*;

        matches!(
            self,
            Mainnet
                | Arbitrum
                | Base
                | Optimism
                | Unichain
                | Avalanche
                | Polygon
                | Sepolia
                | ArbitrumSepolia
                | AvalancheFuji
                | BaseSepolia
                | OptimismSepolia
                | PolygonAmoy
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_chains::NamedChain;
    use rstest::rstest;

    #[rstest]
    #[case(NamedChain::Mainnet, true)]
    #[case(NamedChain::Arbitrum, true)]
    #[case(NamedChain::Base, true)]
    #[case(NamedChain::Optimism, true)]
    #[case(NamedChain::Unichain, true)]
    #[case(NamedChain::Avalanche, true)]
    #[case(NamedChain::Polygon, true)]
    #[case(NamedChain::Sepolia, true)]
    #[case(NamedChain::ArbitrumSepolia, true)]
    #[case(NamedChain::AvalancheFuji, true)]
    #[case(NamedChain::BaseSepolia, true)]
    #[case(NamedChain::OptimismSepolia, true)]
    #[case(NamedChain::PolygonAmoy, true)]
    #[case(NamedChain::BinanceSmartChain, false)]
    #[case(NamedChain::Fantom, false)]
    fn test_is_supported(#[case] chain: NamedChain, #[case] expected: bool) {
        assert_eq!(chain.is_supported(), expected);
    }

    #[rstest]
    #[case(NamedChain::Mainnet, 19 * 60)]
    #[case(NamedChain::Arbitrum, 19 * 60)]
    #[case(NamedChain::Base, 19 * 60)]
    #[case(NamedChain::Optimism, 19 * 60)]
    #[case(NamedChain::Unichain, 19 * 60)]
    #[case(NamedChain::Avalanche, 20)]
    #[case(NamedChain::Polygon, 8 * 60)]
    #[case(NamedChain::Sepolia, 60)]
    #[case(NamedChain::ArbitrumSepolia, 20)]
    #[case(NamedChain::AvalancheFuji, 20)]
    #[case(NamedChain::BaseSepolia, 20)]
    #[case(NamedChain::OptimismSepolia, 20)]
    #[case(NamedChain::PolygonAmoy, 20)]
    fn test_confirmation_average_time_seconds_supported_chains(
        #[case] chain: NamedChain,
        #[case] expected: u64,
    ) {
        assert_eq!(chain.confirmation_average_time_seconds().unwrap(), expected);
    }

    #[test]
    fn test_confirmation_average_time_seconds_unsupported_chain() {
        let result = NamedChain::BinanceSmartChain.confirmation_average_time_seconds();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CctpError::ChainNotSupported { .. }
        ));
    }

    #[rstest]
    #[case(NamedChain::Arbitrum, ARBITRUM_DOMAIN_ID)]
    #[case(NamedChain::ArbitrumSepolia, ARBITRUM_DOMAIN_ID)]
    #[case(NamedChain::Avalanche, AVALANCHE_DOMAIN_ID)]
    #[case(NamedChain::Base, BASE_DOMAIN_ID)]
    #[case(NamedChain::BaseSepolia, BASE_DOMAIN_ID)]
    #[case(NamedChain::Mainnet, ETHEREUM_DOMAIN_ID)]
    #[case(NamedChain::Sepolia, ETHEREUM_DOMAIN_ID)]
    #[case(NamedChain::Optimism, OPTIMISM_DOMAIN_ID)]
    #[case(NamedChain::Polygon, POLYGON_DOMAIN_ID)]
    #[case(NamedChain::Unichain, UNICHAIN_DOMAIN_ID)]
    fn test_cctp_domain_id_supported_chains(#[case] chain: NamedChain, #[case] expected: u32) {
        assert_eq!(chain.cctp_domain_id().unwrap(), expected);
    }

    #[test]
    fn test_cctp_domain_id_unsupported_chain() {
        let result = NamedChain::BinanceSmartChain.cctp_domain_id();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CctpError::ChainNotSupported { .. }
        ));
    }

    #[rstest]
    #[case(NamedChain::Arbitrum, ARBITRUM_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::ArbitrumSepolia, ARBITRUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Avalanche, AVALANCHE_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Base, BASE_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::BaseSepolia, BASE_SEPOLIA_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Sepolia, ETHEREUM_SEPOLIA_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Mainnet, ETHEREUM_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Optimism, OPTIMISM_TOKEN_MESSENGER_ADDRESS)]
    #[case(NamedChain::Polygon, POLYGON_CCTP_V1_TOKEN_MESSENGER)]
    #[case(NamedChain::Unichain, UNICHAIN_CCTP_V1_TOKEN_MESSENGER)]
    fn test_token_messenger_address_supported_chains(
        #[case] chain: NamedChain,
        #[case] expected: Address,
    ) {
        let result = chain.token_messenger_address().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_token_messenger_address_unsupported_chain() {
        let result = NamedChain::BinanceSmartChain.token_messenger_address();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CctpError::ChainNotSupported { .. }
        ));
    }

    #[rstest]
    #[case(NamedChain::Arbitrum, ARBITRUM_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Avalanche, AVALANCHE_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Base, BASE_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Mainnet, ETHEREUM_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Optimism, OPTIMISM_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Polygon, POLYGON_CCTP_V1_MESSAGE_TRANSMITTER)]
    #[case(
        NamedChain::ArbitrumSepolia,
        ARBITRUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS
    )]
    #[case(NamedChain::BaseSepolia, BASE_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Sepolia, ETHEREUM_SEPOLIA_MESSAGE_TRANSMITTER_ADDRESS)]
    #[case(NamedChain::Unichain, UNICHAIN_CCTP_V1_MESSAGE_TRANSMITTER)]
    fn test_message_transmitter_address_supported_chains(
        #[case] chain: NamedChain,
        #[case] expected: Address,
    ) {
        let result = chain.message_transmitter_address().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_message_transmitter_address_unsupported_chain() {
        let result = NamedChain::BinanceSmartChain.message_transmitter_address();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CctpError::ChainNotSupported { .. }
        ));
    }

    #[test]
    fn test_address_parsing_validation() {
        // All addresses should be valid Ethereum addresses
        for chain in [
            NamedChain::Mainnet,
            NamedChain::Arbitrum,
            NamedChain::Base,
            NamedChain::Optimism,
            NamedChain::Unichain,
            NamedChain::Avalanche,
            NamedChain::Polygon,
            NamedChain::Sepolia,
            NamedChain::ArbitrumSepolia,
            NamedChain::BaseSepolia,
        ] {
            assert!(
                chain.token_messenger_address().is_ok(),
                "Token messenger address should be valid for {chain:?}"
            );
            assert!(
                chain.message_transmitter_address().is_ok(),
                "Message transmitter address should be valid for {chain:?}"
            );
        }
    }
}
