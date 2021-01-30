use crate::keys::*;

extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeStruct;
use std::fmt;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Clone, Deserialize, PartialEq)]
pub struct Transaction {
    pub identity: Bytes,
    pub method: String,
    pub data: String,
    pub pub_key: Bytes,
    pub signature: Bytes,
}

impl Transaction {
    pub fn from_str(identity: String, method: String, data: String, pub_key: Bytes) -> Self {
        let bytes = Self::hash_identity(&identity);
        return Self::new(bytes, method, data, pub_key);
    }

    pub fn new(identity: Bytes, method: String, data: String, pub_key: Bytes) -> Self {
        Transaction { identity, method, data, pub_key, signature: Bytes::zero64() }
    }

    pub fn from_json(json: &str) -> Option<Self> {
        match serde_json::from_str(json) {
            Ok(transaction) => Some(transaction),
            Err(_) => None
        }
    }

    pub fn set_signature(&mut self, hash: Bytes) {
        self.signature = hash;
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        // Let it panic if something is not okay
        serde_json::to_vec(&self).unwrap()
    }

    pub fn to_string(&self) -> String {
        // Let it panic if something is not okay
        serde_json::to_string(&self).unwrap()
    }

    pub fn hash_identity(identity: &str) -> Bytes {
        let mut buf: [u8; 32] = [0; 32];
        let mut digest = Sha256::new();
        digest.input_str(identity);
        digest.result(&mut buf);
        Bytes::from_bytes(&buf)
    }
}

impl fmt::Debug for Transaction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Transaction")
            .field("identity", &self.identity)
            .field("method", &self.method)
            .field("data", &self.data)
            .field("pub", &&self.pub_key)
            .field("sign", &&self.signature)
            .finish()
    }
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut structure = serializer.serialize_struct("Transaction", 3).unwrap();
        structure.serialize_field("identity", &self.identity);
        structure.serialize_field("method", &self.method);
        structure.serialize_field("data", &self.data);
        structure.serialize_field("pub_key", &self.pub_key);
        structure.serialize_field("signature", &self.signature);
        structure.end()
    }
}