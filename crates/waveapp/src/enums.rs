/// Account balance type.
#[derive(Debug)]
pub enum AccountNormalBalanceType {
    /// Credit
    CREDIT,
    /// Debit
    DEBIT,
}

impl AccountNormalBalanceType {
    pub fn to_string(&self) -> String {
        match self {
            AccountNormalBalanceType::CREDIT => "CREDIT".to_string(),
            AccountNormalBalanceType::DEBIT => "DEBIT".to_string()
        }
    }
}

/// Subtypes of accounts, as used in the Chart of Accounts.
#[derive(Debug)]
pub enum AccountSubtypeValue {
    /// Cash & Bank
    CASH_AND_BANK,
    /// Cost of Goods Sold
    COST_OF_GOODS_SOLD,
    /// Credit Card
    CREDIT_CARD,
    /// Customer Prepayments and Customer Credits
    CUSTOMER_PREPAYMENTS_AND_CREDITS,
    /// Depreciation and Amortization
    DEPRECIATION_AND_AMORTIZATION,
    /// Discount
    DISCOUNTS,
    /// Due For Payroll
    DUE_FOR_PAYROLL,
    /// Due to You and Other Business Owners
    DUE_TO_YOU_AND_OTHER_OWNERS,
    /// Expense
    EXPENSE,
    /// Gain on Foreign Exchange
    GAIN_ON_FOREIGN_EXCHANGE,
    /// Income
    INCOME,
    /// Inventory
    INVENTORY,
    /// Loan and Line of Credit
    LOANS,
    /// Loss on Foreign Exchange
    LOSS_ON_FOREIGN_EXCHANGE,
    /// Money in Transit
    MONEY_IN_TRANSIT,
    /// Business Owner Contribution
    NON_RETAINED_EARNINGS,
    /// Other Short-Term Asset
    OTHER_CURRENT_ASSETS,
    /// Other Short-Term Liability
    OTHER_CURRENT_LIABILITY,
    /// Other Income
    OTHER_INCOME,
    /// Other Long-Term Asset
    OTHER_LONG_TERM_ASSETS,
    /// Other Long-Term Liability
    OTHER_LONG_TERM_LIABILITY,
    /// Payable
    PAYABLE,
    /// System Payable Bill
    PAYABLE_BILLS,
    /// System Payable Non-Bill
    PAYABLE_OTHER,
    /// Payment Processing Fee
    PAYMENT_PROCESSING_FEES,
    /// Payroll Expense
    PAYROLL_EXPENSES,
    /// Property, Plant, Equipment
    PROPERTY_PLANT_EQUIPMENT,
    /// Receivable
    RECEIVABLE,
    /// System Receivable Invoice
    RECEIVABLE_INVOICES,
    /// System Receivable Non-Invoice
    RECEIVABLE_OTHER,
    /// Retained Earnings: Profit and Business Owner Drawing
    RETAINED_EARNINGS,
    /// Sales Tax on Sales and Purchases
    SALES_TAX,
    /// System Customer Credits
    SYSTEM_CUSTOMER_CREDITS,
    /// Transfers
    TRANSFERS,
    /// Uncategorized Expense
    UNCATEGORIZED_EXPENSE,
    /// Uncategorized Income
    UNCATEGORIZED_INCOME,
    /// Unknown Account
    UNKNOWN_ACCOUNT,
    /// Vendor Prepayments and Vendor Credits
    VENDOR_PREPAYMENTS_AND_CREDITS,
}


impl AccountSubtypeValue {
    pub fn to_string(&self) -> String {
        match self {
            AccountSubtypeValue::CASH_AND_BANK => "CASH_AND_BANK".to_string(),
            AccountSubtypeValue::COST_OF_GOODS_SOLD => "COST_OF_GOODS_SOLD".to_string(),
            AccountSubtypeValue::CREDIT_CARD => "CREDIT_CARD".to_string(),
            AccountSubtypeValue::CUSTOMER_PREPAYMENTS_AND_CREDITS => "CUSTOMER_PREPAYMENTS_AND_CREDITS".to_string(),
            AccountSubtypeValue::DEPRECIATION_AND_AMORTIZATION => "DEPRECIATION_AND_AMORTIZATION".to_string(),
            AccountSubtypeValue::DISCOUNTS => "DISCOUNTS".to_string(),
            AccountSubtypeValue::DUE_FOR_PAYROLL => "DUE_FOR_PAYROLL".to_string(),
            AccountSubtypeValue::DUE_TO_YOU_AND_OTHER_OWNERS => "DUE_TO_YOU_AND_OTHER_OWNERS".to_string(),
            AccountSubtypeValue::EXPENSE => "EXPENSE".to_string(),
            AccountSubtypeValue::GAIN_ON_FOREIGN_EXCHANGE => "GAIN_ON_FOREIGN_EXCHANGE".to_string(),
            AccountSubtypeValue::INCOME => "INCOME".to_string(),
            AccountSubtypeValue::INVENTORY => "INVENTORY".to_string(),
            AccountSubtypeValue::LOANS => "LOANS".to_string(),
            AccountSubtypeValue::LOSS_ON_FOREIGN_EXCHANGE => "LOSS_ON_FOREIGN_EXCHANGE".to_string(),
            AccountSubtypeValue::MONEY_IN_TRANSIT => "MONEY_IN_TRANSIT".to_string(),
            AccountSubtypeValue::NON_RETAINED_EARNINGS => "NON_RETAINED_EARNINGS".to_string(),
            AccountSubtypeValue::OTHER_CURRENT_ASSETS => "OTHER_CURRENT_ASSETS".to_string(),
            AccountSubtypeValue::OTHER_CURRENT_LIABILITY => "OTHER_CURRENT_LIABILITY".to_string(),
            AccountSubtypeValue::OTHER_INCOME => "OTHER_INCOME".to_string(),
            AccountSubtypeValue::OTHER_LONG_TERM_ASSETS => "OTHER_LONG_TERM_ASSETS".to_string(),
            AccountSubtypeValue::OTHER_LONG_TERM_LIABILITY => "OTHER_LONG_TERM_LIABILITY".to_string(),
            AccountSubtypeValue::PAYABLE => "PAYABLE".to_string(),
            AccountSubtypeValue::PAYABLE_BILLS => "PAYABLE_BILLS".to_string(),
            AccountSubtypeValue::PAYABLE_OTHER => "PAYABLE_OTHER".to_string(),
            AccountSubtypeValue::PAYMENT_PROCESSING_FEES => "PAYMENT_PROCESSING_FEES".to_string(),
            AccountSubtypeValue::PAYROLL_EXPENSES => "PAYROLL_EXPENSES".to_string(),
            AccountSubtypeValue::PROPERTY_PLANT_EQUIPMENT => "PROPERTY_PLANT_EQUIPMENT".to_string(),
            AccountSubtypeValue::RECEIVABLE => "RECEIVABLE".to_string(),
            AccountSubtypeValue::RECEIVABLE_INVOICES => "RECEIVABLE_INVOICES".to_string(),
            AccountSubtypeValue::RECEIVABLE_OTHER => "RECEIVABLE_OTHER".to_string(),
            AccountSubtypeValue::RETAINED_EARNINGS => "RETAINED_EARNINGS".to_string(),
            AccountSubtypeValue::SALES_TAX => "SALES_TAX".to_string(),
            AccountSubtypeValue::SYSTEM_CUSTOMER_CREDITS => "SYSTEM_CUSTOMER_CREDITS".to_string(),
            AccountSubtypeValue::TRANSFERS => "TRANSFERS".to_string(),
            AccountSubtypeValue::UNCATEGORIZED_EXPENSE => "UNCATEGORIZED_EXPENSE".to_string(),
            AccountSubtypeValue::UNCATEGORIZED_INCOME => "UNCATEGORIZED_INCOME".to_string(),
            AccountSubtypeValue::UNKNOWN_ACCOUNT => "UNKNOWN_ACCOUNT".to_string(),
            AccountSubtypeValue::VENDOR_PREPAYMENTS_AND_CREDITS => "VENDOR_PREPAYMENTS_AND_CREDITS".to_string(),
        }
    }
}


/// Types of accounts, as used in the Chart of Accounts.
#[derive(Debug)]
pub enum AccountTypeValue {
    /// Represents the different types of economic resources owned or controlled by an entity.
    ASSET,
    /// Represents the residual equity of an entity.
    EQUITY,
    /// Represents the business's expenditures.
    EXPENSE,
    /// Represents the business's earnings.
    INCOME,
    /// Represents the different types of economic obligations of an entity.
    LIABILITY,
}

impl AccountTypeValue {
    pub fn to_string(&self) -> String {
        match self {
            AccountTypeValue::ASSET => "ASSET".to_string(),
            AccountTypeValue::EQUITY => "EQUITY".to_string(),
            AccountTypeValue::EXPENSE => "EXPENSE".to_string(),
            AccountTypeValue::INCOME => "INCOME".to_string(),
            AccountTypeValue::LIABILITY => "LIABILITY".to_string(),
        }
    }
}

/// Balance type that expresses how to change an account.
#[derive(Debug)]
pub enum BalanceType {
    /// Credit.
    CREDIT,
    /// Debit.
    DEBIT,
    /// Decrease using the inverse of the account's normal balance type. For contra accounts whose subtype is `DISCOUNTS` or `DEPRECIATION_AND_AMORTIZATION`, apply the amount in the account's normal balance type.
    DECREASE,
    /// Increase using the account's normal balance type. For contra accounts whose subtype is `DISCOUNTS` or `DEPRECIATION_AND_AMORTIZATION`, apply the amount in the inverse of the account's normal balance type.
    INCREASE,
}

impl BalanceType {
    pub fn to_string(&self) -> String {
        match self {
            BalanceType::CREDIT => "CREDIT".to_string(),
            BalanceType::DEBIT => "DEBIT".to_string(),
            BalanceType::DECREASE => "DECREASE".to_string(),
            BalanceType::INCREASE => "INCREASE".to_string(),
        }
    }
}

/// Granular area of focus of a business.
#[derive(Debug)]
pub enum BusinessSubtypeValue {
    ADVERTISING_PUBLIC_RELATIONS,
    AGRICULTURE_RANCHING_FARMING,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__ACTOR,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__AUDIO_VISUAL_PRODUCTION,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__CRAFTSPERSON,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__DANCER_CHOREOG,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__MUSICIAN,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__OTHER,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__PERFORMING_ARTS_ACTING_MUSIC_DANCE,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__PHOTOGRAPHER,
    ARTISTS_PHOTOGRAPHERS_CREATIVE__VISUAL_ARTIST,
    AUTOMOTIVE_SALES_AND_REPAIR,
    CHURCH_RELIGIOUS_ORGANIZATION,
    CONSTRUCTION_HOME_IMPROVEMENT__CONTRACTOR,
    CONSTRUCTION_HOME_IMPROVEMENT__ENGINEER,
    CONSTRUCTION_HOME_IMPROVEMENT__HOME_INSPECTOR,
    CONSTRUCTION_HOME_IMPROVEMENT__OTHER_TRADES,
    CONSULTANTS_PROFESSIONALS__ACCOUNTANTS_BOOKKEEPERS,
    CONSULTANTS_PROFESSIONALS__COMMUNICATIONS,
    CONSULTANTS_PROFESSIONALS__EXECUTIVE_COACH,
    CONSULTANTS_PROFESSIONALS__HR_RECRUITMENT_STAFFING,
    CONSULTANTS_PROFESSIONALS__IT_TECHNICAL,
    CONSULTANTS_PROFESSIONALS__OTHER,
    CONSULTANTS_PROFESSIONALS__SALES,
    DESIGN_ARCHITECTURE_ENGINEERING,
    FINANCIAL_SERVICES,
    HAIR_SPA_AESTHETICS__HAIR_SALON,
    HAIR_SPA_AESTHETICS__MASSAGE,
    HAIR_SPA_AESTHETICS__NAIL_SKIN_AESTHETICS,
    HAIR_SPA_AESTHETICS__OTHER,
    INSURANCE_AGENCY_BROKER,
    LANDLORD_PROPERTY_MANAGER__LANDLORD,
    LANDLORD_PROPERTY_MANAGER__PROPERTY_MANAGER,
    LAWN_CARE_LANDSCAPING,
    LEGAL_SERVICES,
    LODGING_HOTEL_MOTEL,
    MANUFACTURER_REPRESENTATIVE_AGENT,
    MEDICAL_DENTAL_HEALTH_SERVICE__CHIROPRACTOR,
    MEDICAL_DENTAL_HEALTH_SERVICE__DENTIST,
    MEDICAL_DENTAL_HEALTH_SERVICE__FITNESS,
    MEDICAL_DENTAL_HEALTH_SERVICE__MASSAGE_THERAPIST,
    MEDICAL_DENTAL_HEALTH_SERVICE__MENTAL_HEALTH,
    MEDICAL_DENTAL_HEALTH_SERVICE__NUTRITION,
    MEDICAL_DENTAL_HEALTH_SERVICE__OCCUP_THERAPIST,
    MEDICAL_DENTAL_HEALTH_SERVICE__OTHER,
    MEDICAL_DENTAL_HEALTH_SERVICE__PHYSICAL_THERAPIST,
    NONPROFIT_ASSOCIATIONS_GROUPS__ASSOCIATION,
    NONPROFIT_ASSOCIATIONS_GROUPS__CHARITABLE,
    NONPROFIT_ASSOCIATIONS_GROUPS__CLUB,
    NONPROFIT_ASSOCIATIONS_GROUPS__CONDO,
    NONPROFIT_ASSOCIATIONS_GROUPS__OTHER,
    NONPROFIT_ASSOCIATIONS_GROUPS__PARENT_BOOSTER,
    OTHER__OTHER_PLEASE_SPECIFY,
    PRODUCT_PROVIDER__MANUFACTURER,
    PRODUCT_PROVIDER__MANUFACTURER_AND_VENDOR,
    PRODUCT_PROVIDER__OTHER,
    PRODUCT_PROVIDER__VENDOR,
    REAL_ESTATE_SALES__AGENT,
    REAL_ESTATE_SALES__BROKER,
    REAL_ESTATE_SALES__OTHER,
    RENTAL,
    REPAIR_AND_MAINTENANCE,
    RESTAURANT_CATERER_BAR,
    RETAILERS_AND_RESELLERS__EBAY,
    RETAILERS_AND_RESELLERS__ETSY,
    RETAILERS_AND_RESELLERS__NON_STORE_RETAILER,
    RETAILERS_AND_RESELLERS__OTHER,
    RETAILERS_AND_RESELLERS__STORE_RETAILER,
    SALES_INDEPENDENT_AGENT,
    SERVICE_PROVIDER__CLEANING_JANITORIAL_SERVICES,
    SERVICE_PROVIDER__CUSTOMER_SERVICE_SUPPORT,
    SERVICE_PROVIDER__DOMESTIC_CAREGIVER_EMPLOYER,
    SERVICE_PROVIDER__FITNESS,
    SERVICE_PROVIDER__OFFICE_ADMIN_SUPPORT,
    SERVICE_PROVIDER__OTHER,
    SERVICE_PROVIDER__PERSONAL_CARE,
    SERVICE_PROVIDER__TELEMARKETING,
    SERVICE_PROVIDER__TRANSCRIPTION,
    TRANSPORTATION_TRUCKING_DELIVERY,
    WEB_MEDIA_FREELANCER__DESIGNER,
    WEB_MEDIA_FREELANCER__MARKETING_SOCIAL_MEDIA,
    WEB_MEDIA_FREELANCER__OTHER,
    WEB_MEDIA_FREELANCER__PROGRAMMER,
    WEB_MEDIA_FREELANCER__SEO,
    WEB_MEDIA_FREELANCER__WRITER,
    WHOLESALE_DISTRIBUTION_SALES,
}

/// Area of focus of a business.
#[derive(Debug)]
pub enum BusinessTypeValue {
    ARTISTS_PHOTOGRAPHERS_CREATIVE,
    CONSULTANTS_PROFESSIONALS,
    FINANCE_INSURANCE,
    HAIR_SPA_AESTHETICS,
    MEDICAL_DENTAL_HEALTH_SERVICE,
    NONPROFIT_ASSOCIATIONS_GROUPS,
    OTHER(String),
    PRODUCT_PROVIDER,
    REALESTATE_HOME,
    RETAILERS_AND_RESELLERS,
    SERVICE_PROVIDER,
    WEB_MEDIA_FREELANCER,
}

