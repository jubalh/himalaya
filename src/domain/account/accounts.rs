//! Account module.
//!
//! This module contains the definition of the printable account,
//! which is only used by the "accounts" command to list all available
//! accounts from the config file.

use anyhow::Result;
use serde::Serialize;
use std::{collections::hash_map::Iter, ops::Deref};

use crate::{
    printer::{PrintTable, PrintTableOpts, WriteColor},
    ui::Table,
};

use super::{Account, DeserializedAccountConfig};

/// Represents the list of printable accounts.
#[derive(Debug, Default, Serialize)]
pub struct Accounts(pub Vec<Account>);

impl Deref for Accounts {
    type Target = Vec<Account>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrintTable for Accounts {
    fn print_table(&self, writer: &mut dyn WriteColor, opts: PrintTableOpts) -> Result<()> {
        writeln!(writer)?;
        Table::print(writer, self, opts)?;
        writeln!(writer)?;
        Ok(())
    }
}

impl From<Iter<'_, String, DeserializedAccountConfig>> for Accounts {
    fn from(map: Iter<'_, String, DeserializedAccountConfig>) -> Self {
        let mut accounts: Vec<_> = map
            .map(|(name, account)| match account {
                #[cfg(feature = "imap-backend")]
                DeserializedAccountConfig::Imap(config) => {
                    Account::new(name, "imap", config.base.default.unwrap_or_default())
                }
                #[cfg(feature = "maildir-backend")]
                DeserializedAccountConfig::Maildir(config) => {
                    Account::new(name, "maildir", config.base.default.unwrap_or_default())
                }
                #[cfg(feature = "notmuch-backend")]
                DeserializedAccountConfig::Notmuch(config) => {
                    Account::new(name, "notmuch", config.base.default.unwrap_or_default())
                }
                DeserializedAccountConfig::None(..) => Account::new(name, "none", false),
            })
            .collect();
        accounts.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());
        Self(accounts)
    }
}