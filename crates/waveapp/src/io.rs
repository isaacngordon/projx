pub trait Input {
    fn foo(){}
}
pub trait Output {}

/// Input to the customerCreate mutation.
pub struct CustomerCreateInput {
    /// The unique identifier for the business.
    businessId: crate::scalar::ID,
    /// Name or business name of the customer.
    name: String,
    /// First name of the principal contact.
    firstName: Option<String>,
    /// Last name of the principal contact.
    lastName: Option<String>,
    /// Address of the customer.
    address: AddressInput,
    /// User defined id for the customer.
    displayId: Option<String>,
    /// Email of the principal contact.
    email: Option<String>,
    /// Mobile telephone number of the principal contact.
    mobile: Option<String>,
    /// Telephone number of the customer.
    phone: Option<String>,
    /// Fax number of the customer.
    fax: Option<String>,
    /// Toll-free number of the customer.
    tollFree: Option<String>,
    /// Website address of the customer.
    website: Option<String>,
    /// Internal notes about the customer.
    internalNotes: Option<String>,
    /// Default currency used by the customer.
    currency: crate::enums::CurrencyCode,
    /// Details for shipping to the customer.
    shippingDetails: Option<CustomerShippingDetailsInput>,
}
impl Input for CustomerCreateInput {
    
}

pub struct CustomerCreateOutput {}
impl Output for CustomerCreateOutput {

}

/// An address.
pub struct AddressInput {
    /// Address line 1 (Street address/PO Box/Company name).
    addressLine1: String,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    addressLine2: String,
    /// City/District/Suburb/Town/Village.
    city: String,
    /// State/County/Province/Region Code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    provinceCode: String,
    /// Country Code.
    countryCode: crate::enums::CountryCode,
    /// Zip/Postal Code.
    postalCode: String,
}


pub struct CustomerShippingDetailsInput{
    /// Name or business name of the customer.
    name: String,
    /// Address of the customer.
    address: AddressInput,
    /// Telephone number of the customer.
    phone: String,
    /// Delivery instructions for handling.
    instructions: String,
}