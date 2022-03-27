use core::fmt;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub code: String,
    pub country: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvidersResponse {
    pub providers: Vec<Provider>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub key: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub document: String,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Prefs {
    api_key: String,
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub number: String,
    pub branch: String,
    pub currency: String,
    pub balance: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsResponse {
    pub status: String,
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCard {
    pub id: String,
    pub name: String,
    pub number: String,
    pub close_date: String,
    pub due_date: String,
    pub balance_local: u32,
    pub balance_dollar: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCardsResponse {
    pub status: String,
    pub credit_cards: Vec<CreditCard>,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} ({})", self.name, self.country, self.code)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{}| N° {} ({})\n$    {} {}\n------------------------------------",
            self.id.bold(),
            self.number.bold(),
            self.name.bold().bright_cyan(),
            self.currency.bold().green(),
            self.balance.to_string().bold().green()
        )
    }
}

impl fmt::Display for CreditCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{}| N° {} ({}) {} - {}\n$    {} (local)\n$    {} (dollar)\n------------------------------------",
            self.id.bold(),
            self.number.bold(),
            self.name.bold().bright_cyan(),
            self.close_date.bold().green(),
            self.due_date.to_string().bold().green(),
            self.balance_local.to_string().bold().green(),
            self.balance_dollar.to_string().bold().green(),
        )
    }
}
