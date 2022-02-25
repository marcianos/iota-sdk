// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Address types used in the account
pub(crate) mod address;
/// Custom de/serialization for [`address::AddressWrapper`]
pub(crate) mod address_serde;

use crypto::keys::slip10::Chain;
use iota_client::{
    bee_message::{address::Address, output::OutputId, payload::transaction::TransactionPayload, MessageId},
    bee_rest_api::types::responses::OutputResponse,
    signing::types::InputSigningData,
};

use serde::{Deserialize, Deserializer, Serialize};

use std::{collections::HashMap, str::FromStr};

/// The balance of an account, returned from [`crate::account::handle::AccountHandle::sync()`] and
/// [`crate::account::handle::AccountHandle::balance()`].
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub(crate) total: u64,
    // balance that can currently spend
    pub(crate) available: u64,
    // currently required amount for the byte cost
    pub(crate) byte_cost_deposit: u64,
    pub(crate) native_tokens: HashMap<String, u128>,
    // todo: should it look like this?
    pub(crate) nfts: HashMap<String, u128>,
    // todo: should it look like this?
    pub(crate) foundrys: HashMap<String, u128>,
    // todo: should it look like this?
    pub(crate) alias_outputs: HashMap<String, u128>,
}

/// An output with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputData {
    /// The output id
    #[serde(rename = "outputId")]
    pub output_id: OutputId,
    /// The output response
    #[serde(rename = "outputResponse")]
    pub output_response: OutputResponse,
    pub amount: u64,
    /// If an output is spent
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    /// Associated account address.
    pub address: Address,
    /// Network ID
    #[serde(rename = "networkId")]
    pub network_id: u64,
    pub remainder: bool,
    // bip32 path
    pub chain: Option<Chain>,
}

impl OutputData {
    pub fn input_signing_data(&self) -> crate::Result<InputSigningData> {
        Ok(InputSigningData {
            output_response: self.output_response.clone(),
            chain: self.chain.clone(),
            bech32_address: self.address.to_bech32("atoi"),
        })
    }
}

/// A transaction with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub payload: TransactionPayload,
    pub message_id: Option<MessageId>,
    pub inclusion_state: InclusionState,
    // remove because we have a timestamp in the outputs?
    pub timestamp: u128,
    // network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: u64,
    // set if the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
}

/// Possible InclusionStates for transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InclusionState {
    Pending,
    Confirmed,
    Conflicting,
}

/// The output kind enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputKind {
    /// Alias output.
    Alias,
    /// Basic output.
    Basic,
    /// Foundry output.
    Foundry,
    /// Nft output.
    Nft,
    /// Treasury output.
    Treasury,
}

impl FromStr for OutputKind {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "Alias" => Self::Alias,
            "Basic" => Self::Basic,
            "Foundry" => Self::Foundry,
            "Nft" => Self::Nft,
            "Treasury" => Self::Treasury,
            _ => return Err(crate::Error::InvalidOutputKind(s.to_string())),
        };
        Ok(kind)
    }
}

/// The account identifier.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AccountIdentifier {
    /// Account alias as identifier.
    Alias(String),
    /// An index identifier.
    Index(u32),
}

impl<'de> Deserialize<'de> for AccountIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(AccountIdentifier::from(s))
    }
}

// When the identifier is a string.
impl From<&str> for AccountIdentifier {
    fn from(value: &str) -> Self {
        match u32::from_str(value) {
            Ok(index) => Self::Index(index),
            Err(_) => Self::Alias(value.to_string()),
        }
    }
}

impl From<String> for AccountIdentifier {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for AccountIdentifier {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

// When the identifier is an index.
impl From<u32> for AccountIdentifier {
    fn from(value: u32) -> Self {
        Self::Index(value)
    }
}