pub use fixed_point::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod fixed_point {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"Q112\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESOLUTION\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static FIXEDPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        97,
        1,
        18,
        97,
        0,
        38,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        97,
        0,
        25,
        87,
        254,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        96,
        61,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        59,
        247,
        168,
        62,
        20,
        96,
        155,
        87,
        128,
        99,
        85,
        47,
        136,
        138,
        20,
        96,
        179,
        87,
        91,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        96,
        161,
        96,
        207,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        96,
        185,
        96,
        215,
        86,
        91,
        96,
        64,
        128,
        81,
        96,
        255,
        144,
        146,
        22,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        96,
        1,
        96,
        112,
        27,
        129,
        86,
        91,
        96,
        112,
        129,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        243,
        70,
        45,
        26,
        243,
        111,
        92,
        149,
        139,
        10,
        89,
        76,
        222,
        75,
        134,
        81,
        175,
        11,
        67,
        148,
        196,
        44,
        165,
        107,
        230,
        131,
        77,
        20,
        200,
        58,
        188,
        136,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        6,
        6,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static FIXEDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        96,
        61,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        59,
        247,
        168,
        62,
        20,
        96,
        155,
        87,
        128,
        99,
        85,
        47,
        136,
        138,
        20,
        96,
        179,
        87,
        91,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        96,
        161,
        96,
        207,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        96,
        185,
        96,
        215,
        86,
        91,
        96,
        64,
        128,
        81,
        96,
        255,
        144,
        146,
        22,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        96,
        1,
        96,
        112,
        27,
        129,
        86,
        91,
        96,
        112,
        129,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        243,
        70,
        45,
        26,
        243,
        111,
        92,
        149,
        139,
        10,
        89,
        76,
        222,
        75,
        134,
        81,
        175,
        11,
        67,
        148,
        196,
        44,
        165,
        107,
        230,
        131,
        77,
        20,
        200,
        58,
        188,
        136,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        6,
        6,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static FIXEDPOINT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FixedPoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FixedPoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FixedPoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FixedPoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FixedPoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(FixedPoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FixedPoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FIXEDPOINT_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                FIXEDPOINT_ABI.clone(),
                FIXEDPOINT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `Q112` (0x3bf7a83e) function
        pub fn q112(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 247, 168, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESOLUTION` (0x552f888a) function
        pub fn resolution(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([85, 47, 136, 138], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for FixedPoint<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `Q112` function with signature `Q112()` and selector `0x3bf7a83e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "Q112", abi = "Q112()")]
    pub struct Q112Call;
    ///Container type for all input parameters for the `RESOLUTION` function with signature `RESOLUTION()` and selector `0x552f888a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "RESOLUTION", abi = "RESOLUTION()")]
    pub struct ResolutionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FixedPointCalls {
        Q112(Q112Call),
        Resolution(ResolutionCall),
    }
    impl ::ethers::core::abi::AbiDecode for FixedPointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Q112Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Q112(decoded));
            }
            if let Ok(decoded) = <ResolutionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resolution(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FixedPointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Q112(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Resolution(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FixedPointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Q112(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolution(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Q112Call> for FixedPointCalls {
        fn from(value: Q112Call) -> Self {
            Self::Q112(value)
        }
    }
    impl ::core::convert::From<ResolutionCall> for FixedPointCalls {
        fn from(value: ResolutionCall) -> Self {
            Self::Resolution(value)
        }
    }
    ///Container type for all return fields from the `Q112` function with signature `Q112()` and selector `0x3bf7a83e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Q112Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RESOLUTION` function with signature `RESOLUTION()` and selector `0x552f888a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ResolutionReturn(pub u8);
}
