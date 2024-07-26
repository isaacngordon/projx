use super::enums::{CountryCode, CurrencyCode};
use super::scalar;

pub enum Query {
    /// Get the current OAuth application.
    OAuthApplication,
    // List currencies
    Currencies,
    // Get a currency
    Currency {
        code: CurrencyCode,
    },
    // List countries
    Countries,
    // Get a country
    Country {
        code: CountryCode,
    },
    // Get a province
    Province(String),
    // List businesses
    Businesses {
        page: scalar::Int,
        pageSize: scalar::Int,
    },
    // Get a business
    Business {
        id: scalar::ID,
    },
    // The currently authenticated user
    User,
    // List ty[es of accounts]
    AccountTypes,
    // List subtypes of accounts
    AccountSubTypes,
}
