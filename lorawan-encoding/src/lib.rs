//! Library for parsing and handling LoRaWAN packets.
#![no_std]
#![deny(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::upper_case_acronyms)]
#![doc = include_str!("../README.md")]

pub mod creator;
pub mod keys;
pub mod maccommandcreator;
pub mod maccommands;
pub mod multicast;
pub mod packet_length;
pub mod parser;
pub mod string;
pub mod types;

#[cfg(feature = "full")]
pub mod extra;

#[cfg(feature = "default-crypto")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-crypto")))]
pub mod default_crypto;

mod securityhelpers;
