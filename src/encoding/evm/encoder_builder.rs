use tycho_core::models::Chain;

use crate::encoding::{
    errors::EncodingError,
    evm::{
        strategy_encoder::strategy_encoders::{ExecutorStrategyEncoder, SplitSwapStrategyEncoder},
        swap_encoder::swap_encoder_registry::SwapEncoderRegistry,
        tycho_encoder::EVMTychoEncoder,
    },
    strategy_encoder::StrategyEncoder,
};

/// Builder pattern for constructing an `EVMTychoEncoder` with customizable options.
///
/// This struct allows setting a chain and strategy encoder before building the final encoder.
pub struct EVMEncoderBuilder {
    strategy: Option<Box<dyn StrategyEncoder>>,
    chain: Option<Chain>,
}

impl Default for EVMEncoderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EVMEncoderBuilder {
    pub fn new() -> Self {
        EVMEncoderBuilder { chain: None, strategy: None }
    }
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = Some(chain);
        self
    }

    /// Sets the `strategy_encoder` manually.
    ///
    /// **Note**: This method should not be used in combination with `tycho_router` or
    /// `direct_execution`.
    pub fn strategy_encoder(mut self, strategy: Box<dyn StrategyEncoder>) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Shortcut method to initialize a `SplitSwapStrategyEncoder` without any approval nor token in
    /// transfer. **Note**: Should not be used at the same time as `strategy_encoder`.
    pub fn tycho_router(self, executors_file_path: Option<String>) -> Result<Self, EncodingError> {
        if let Some(chain) = self.chain {
            let swap_encoder_registry = SwapEncoderRegistry::new(executors_file_path, chain)?;
            let strategy =
                Box::new(SplitSwapStrategyEncoder::new(chain, swap_encoder_registry, None)?);
            Ok(EVMEncoderBuilder { chain: Some(chain), strategy: Some(strategy) })
        } else {
            Err(EncodingError::FatalError(
                "Please set the chain before setting the tycho router".to_string(),
            ))
        }
    }

    /// Shortcut method to initialize a `SplitSwapStrategyEncoder` with Permit2 approval and token
    /// in transfer. **Note**: Should not be used at the same time as `strategy_encoder`.
    pub fn tycho_router_with_permit2(
        self,
        executors_file_path: Option<String>,
        swapper_pk: String,
    ) -> Result<Self, EncodingError> {
        if let Some(chain) = self.chain {
            let swap_encoder_registry = SwapEncoderRegistry::new(executors_file_path, chain)?;
            let strategy = Box::new(SplitSwapStrategyEncoder::new(
                chain,
                swap_encoder_registry,
                Some(swapper_pk),
            )?);
            Ok(EVMEncoderBuilder { chain: Some(chain), strategy: Some(strategy) })
        } else {
            Err(EncodingError::FatalError(
                "Please set the chain before setting the tycho router".to_string(),
            ))
        }
    }

    /// Shortcut method to initialize an `ExecutorStrategyEncoder`.
    /// **Note**: Should not be used at the same time as `strategy_encoder`.
    pub fn direct_execution(
        self,
        executors_file_path: Option<String>,
    ) -> Result<Self, EncodingError> {
        if let Some(chain) = self.chain {
            let swap_encoder_registry = SwapEncoderRegistry::new(executors_file_path, chain)?;
            let strategy = Box::new(ExecutorStrategyEncoder::new(swap_encoder_registry));
            Ok(EVMEncoderBuilder { chain: Some(chain), strategy: Some(strategy) })
        } else {
            Err(EncodingError::FatalError(
                "Please set the chain before setting the strategy".to_string(),
            ))
        }
    }

    /// Builds the `EVMTychoEncoder` instance using the configured chain and strategy.
    /// Returns an error if either the chain or strategy has not been set.
    pub fn build(self) -> Result<EVMTychoEncoder, EncodingError> {
        if let (Some(chain), Some(strategy)) = (self.chain, self.strategy) {
            EVMTychoEncoder::new(chain, strategy)
        } else {
            Err(EncodingError::FatalError(
                "Please set the chain and strategy before building the encoder".to_string(),
            ))
        }
    }
}
