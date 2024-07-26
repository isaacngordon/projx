use super::enums::{CountryCode, CurrencyCode};
use super::scalar;

pub enum Query {
    /// Get the current OAuth application.
    OAuthApplication,
    /// List currencies
    Currencies,
    /// Get a currency
    Currency {
        // Code of currency
        code: CurrencyCode,
    },
    /// List countries
    Countries,
    /// Get a country
    Country {
        /// Code of country
        code: CountryCode,
    },
    /// Get a province
    Province {
        /// Code of province
        code: String
    },
    /// List businesses
    Businesses {
        /// 1-based page number to retrieve.
        page: scalar::Int,
        pageSize: scalar::Int,
    },
    /// Get a business
    Business {
        id: scalar::ID,
    },
    /// The currently authenticated user
    User,
    // List ty[es of accounts]
    AccountTypes,
    /// List subtypes of accounts
    AccountSubTypes,
}
