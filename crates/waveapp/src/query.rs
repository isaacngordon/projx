use super::enums::{CountryCode, CurrencyCode};
use super::scalar;

pub enum Query {
    OAuthApplication,
    Currencies, 
    Currency{
        code: CurrencyCode
    }, 
    Countries,
    Country {
        code: CountryCode
    }, 
    Province(String), 
    Businesses{
        page: scalar::Int,
        pageSize: scalar::Int
    }, 
    Business {
        id: scalar::ID
    }, 
    User, 
    AccountTypes, 
    AccountSubTypes
}