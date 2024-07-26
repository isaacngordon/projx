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
            AccountNormalBalanceType::DEBIT => "DEBIT".to_string(),
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
            AccountSubtypeValue::CUSTOMER_PREPAYMENTS_AND_CREDITS => {
                "CUSTOMER_PREPAYMENTS_AND_CREDITS".to_string()
            }
            AccountSubtypeValue::DEPRECIATION_AND_AMORTIZATION => {
                "DEPRECIATION_AND_AMORTIZATION".to_string()
            }
            AccountSubtypeValue::DISCOUNTS => "DISCOUNTS".to_string(),
            AccountSubtypeValue::DUE_FOR_PAYROLL => "DUE_FOR_PAYROLL".to_string(),
            AccountSubtypeValue::DUE_TO_YOU_AND_OTHER_OWNERS => {
                "DUE_TO_YOU_AND_OTHER_OWNERS".to_string()
            }
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
            AccountSubtypeValue::OTHER_LONG_TERM_LIABILITY => {
                "OTHER_LONG_TERM_LIABILITY".to_string()
            }
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
            AccountSubtypeValue::VENDOR_PREPAYMENTS_AND_CREDITS => {
                "VENDOR_PREPAYMENTS_AND_CREDITS".to_string()
            }
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
    /// Advertising, Public Relations                                                                          
    ADVERTISING_PUBLIC_RELATIONS,
    /// Agriculture, Ranching and Farming                                                                      
    AGRICULTURE_RANCHING_FARMING,
    /// Actor                                                                                                  
    ARTISTS_PHOTOGRAPHERS_CREATIVE__ACTOR,
    /// Audio/Visual Production                                                                                
    ARTISTS_PHOTOGRAPHERS_CREATIVE__AUDIO_VISUAL_PRODUCTION,
    /// Craftsperson                                                                                           
    ARTISTS_PHOTOGRAPHERS_CREATIVE__CRAFTSPERSON,
    /// Dancer, Choreographer                                                                                  
    ARTISTS_PHOTOGRAPHERS_CREATIVE__DANCER_CHOREOG,
    /// Musician                                                                                               
    ARTISTS_PHOTOGRAPHERS_CREATIVE__MUSICIAN,
    /// Other Creative                                                                                         
    ARTISTS_PHOTOGRAPHERS_CREATIVE__OTHER,
    /// Performing Arts (acting, music, dance)                                                                 
    ARTISTS_PHOTOGRAPHERS_CREATIVE__PERFORMING_ARTS_ACTING_MUSIC_DANCE,
    /// Photographer                                                                                           
    ARTISTS_PHOTOGRAPHERS_CREATIVE__PHOTOGRAPHER,
    /// Visual Artist                                                                                          
    ARTISTS_PHOTOGRAPHERS_CREATIVE__VISUAL_ARTIST,
    /// Automotive Repair & Sales                                                                              
    AUTOMOTIVE_SALES_AND_REPAIR,
    /// Church, Religious Organization                                                                         
    CHURCH_RELIGIOUS_ORGANIZATION,
    /// Contractor                                                                                             
    CONSTRUCTION_HOME_IMPROVEMENT__CONTRACTOR,
    /// Engineer                                                                                               
    CONSTRUCTION_HOME_IMPROVEMENT__ENGINEER,
    /// Home Inspector                                                                                         
    CONSTRUCTION_HOME_IMPROVEMENT__HOME_INSPECTOR,
    /// Trade                                                                                                  
    CONSTRUCTION_HOME_IMPROVEMENT__OTHER_TRADES,
    /// Accountant, Bookkeeper                                                                                 
    CONSULTANTS_PROFESSIONALS__ACCOUNTANTS_BOOKKEEPERS,
    /// Communications, Marketing, PR                                                                          
    CONSULTANTS_PROFESSIONALS__COMMUNICATIONS,
    /// Executive Coach                                                                                        
    CONSULTANTS_PROFESSIONALS__EXECUTIVE_COACH,
    /// HR, Recruitment, Staffing                                                                              
    CONSULTANTS_PROFESSIONALS__HR_RECRUITMENT_STAFFING,
    /// IT, Technical                                                                                          
    CONSULTANTS_PROFESSIONALS__IT_TECHNICAL,
    /// Other Consultant                                                                                       
    CONSULTANTS_PROFESSIONALS__OTHER,
    /// Sales                                                                                                  
    CONSULTANTS_PROFESSIONALS__SALES,
    /// Design, Architecture, Engineering                                                                      
    DESIGN_ARCHITECTURE_ENGINEERING,
    /// Other Financial Service                                                                                
    FINANCIAL_SERVICES,
    /// Salon, Spa                                                                                             
    HAIR_SPA_AESTHETICS__HAIR_SALON,
    /// Massage                                                                                                
    HAIR_SPA_AESTHETICS__MASSAGE,
    /// Nails, Skin, Aesthetics                                                                                
    HAIR_SPA_AESTHETICS__NAIL_SKIN_AESTHETICS,
    /// Other Aesthetics/Spa                                                                                   
    HAIR_SPA_AESTHETICS__OTHER,
    /// Insurance Agency, Broker                                                                               
    INSURANCE_AGENCY_BROKER,
    /// Landlord                                                                                               
    LANDLORD_PROPERTY_MANAGER__LANDLORD,
    /// Property Manager                                                                                       
    LANDLORD_PROPERTY_MANAGER__PROPERTY_MANAGER,
    /// Lawn Care, Landscaping                                                                                 
    LAWN_CARE_LANDSCAPING,
    /// Legal Services                                                                                         
    LEGAL_SERVICES,
    /// Lodging, Hotel, Motel                                                                                  
    LODGING_HOTEL_MOTEL,
    /// Manufacturing Representative, Agent                                                                    
    MANUFACTURER_REPRESENTATIVE_AGENT,
    /// Chiropractor                                                                                           
    MEDICAL_DENTAL_HEALTH_SERVICE__CHIROPRACTOR,
    /// Dentist                                                                                                
    MEDICAL_DENTAL_HEALTH_SERVICE__DENTIST,
    /// Fitness                                                                                                
    MEDICAL_DENTAL_HEALTH_SERVICE__FITNESS,
    /// Massage Therapist                                                                                      
    MEDICAL_DENTAL_HEALTH_SERVICE__MASSAGE_THERAPIST,
    /// Mental Health                                                                                          
    MEDICAL_DENTAL_HEALTH_SERVICE__MENTAL_HEALTH,
    /// Nutrition                                                                                              
    MEDICAL_DENTAL_HEALTH_SERVICE__NUTRITION,
    /// Occupational Therapist                                                                                 
    MEDICAL_DENTAL_HEALTH_SERVICE__OCCUP_THERAPIST,
    /// Other Health                                                                                           
    MEDICAL_DENTAL_HEALTH_SERVICE__OTHER,
    /// Physical Therapist                                                                                     
    MEDICAL_DENTAL_HEALTH_SERVICE__PHYSICAL_THERAPIST,
    /// Association                                                                                            
    NONPROFIT_ASSOCIATIONS_GROUPS__ASSOCIATION,
    /// Charity                                                                                                
    NONPROFIT_ASSOCIATIONS_GROUPS__CHARITABLE,
    /// Club                                                                                                   
    NONPROFIT_ASSOCIATIONS_GROUPS__CLUB,
    /// Condo                                                                                                  
    NONPROFIT_ASSOCIATIONS_GROUPS__CONDO,
    /// Other Non-Profit                                                                                       
    NONPROFIT_ASSOCIATIONS_GROUPS__OTHER,
    /// Parent Booster USA                                                                                     
    NONPROFIT_ASSOCIATIONS_GROUPS__PARENT_BOOSTER,
    /// Other (please specify)                                                                                 
    OTHER__OTHER_PLEASE_SPECIFY,
    /// Manufacturer                                                                                           
    PRODUCT_PROVIDER__MANUFACTURER,
    /// Manufacturer and Vendor                                                                                
    PRODUCT_PROVIDER__MANUFACTURER_AND_VENDOR,
    /// Other Product-based Business                                                                           
    PRODUCT_PROVIDER__OTHER,
    /// Vendor                                                                                                 
    PRODUCT_PROVIDER__VENDOR,
    /// Real Estate Agent                                                                                      
    REAL_ESTATE_SALES__AGENT,
    /// Real Estate Broker                                                                                     
    REAL_ESTATE_SALES__BROKER,
    /// Other Real Estate                                                                                      
    REAL_ESTATE_SALES__OTHER,
    /// Real Estate Rental                                                                                     
    RENTAL,
    /// Repairs/Maintenance                                                                                    
    REPAIR_AND_MAINTENANCE,
    /// Restaurant, Caterer, Bar                                                                               
    RESTAURANT_CATERER_BAR,
    /// eBay Resellers                                                                                         
    RETAILERS_AND_RESELLERS__EBAY,
    /// Etsy Vendors                                                                                           
    RETAILERS_AND_RESELLERS__ETSY,
    /// Non-Store Retailers                                                                                    
    RETAILERS_AND_RESELLERS__NON_STORE_RETAILER,
    /// Other Retailers                                                                                        
    RETAILERS_AND_RESELLERS__OTHER,
    /// Store Retailers                                                                                        
    RETAILERS_AND_RESELLERS__STORE_RETAILER,
    /// Sales: Independent Agent                                                                               
    SALES_INDEPENDENT_AGENT,
    /// Cleaning, Janitorial Services                                                                          
    SERVICE_PROVIDER__CLEANING_JANITORIAL_SERVICES,
    /// Customer Service/Support                                                                               
    SERVICE_PROVIDER__CUSTOMER_SERVICE_SUPPORT,
    /// Household Employer                                                                                     
    SERVICE_PROVIDER__DOMESTIC_CAREGIVER_EMPLOYER,
    /// Fitness                                                                                                
    SERVICE_PROVIDER__FITNESS,
    /// Office Admin/Support                                                                                   
    SERVICE_PROVIDER__OFFICE_ADMIN_SUPPORT,
    /// Other Service-based Business                                                                           
    SERVICE_PROVIDER__OTHER,
    /// Personal Care                                                                                          
    SERVICE_PROVIDER__PERSONAL_CARE,
    /// Telemarketing                                                                                          
    SERVICE_PROVIDER__TELEMARKETING,
    /// Transcription                                                                                          
    SERVICE_PROVIDER__TRANSCRIPTION,
    /// Transportation, Trucking, Deliver                                                                      
    TRANSPORTATION_TRUCKING_DELIVERY,
    /// Designer                                                                                               
    WEB_MEDIA_FREELANCER__DESIGNER,
    /// Marketing, Social Media                                                                                
    WEB_MEDIA_FREELANCER__MARKETING_SOCIAL_MEDIA,
    /// Other Media/Tech                                                                                       
    WEB_MEDIA_FREELANCER__OTHER,
    /// Programmer                                                                                             
    WEB_MEDIA_FREELANCER__PROGRAMMER,
    /// SEO                                                                                                    
    WEB_MEDIA_FREELANCER__SEO,
    /// Writer                                                                                                 
    WEB_MEDIA_FREELANCER__WRITER,
    /// Wholesale Distribution and Sales                                                                       
    WHOLESALE_DISTRIBUTION_SALES,
}

impl BusinessSubtypeValue {
    pub fn to_string(&self) -> String {
        match self {
            BusinessSubtypeValue::ADVERTISING_PUBLIC_RELATIONS => "ADVERTISING_PUBLIC_RELATIONS".to_string(),
            BusinessSubtypeValue::AGRICULTURE_RANCHING_FARMING => "AGRICULTURE_RANCHING_FARMING".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__ACTOR => "ARTISTS_PHOTOGRAPHERS_CREATIVE__ACTOR".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__AUDIO_VISUAL_PRODUCTION => "ARTISTS_PHOTOGRAPHERS_CREATIVE__AUDIO_VISUAL_PRODUCTION".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__CRAFTSPERSON => "ARTISTS_PHOTOGRAPHERS_CREATIVE__CRAFTSPERSON".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__DANCER_CHOREOG => "ARTISTS_PHOTOGRAPHERS_CREATIVE__DANCER_CHOREOG".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__MUSICIAN => "ARTISTS_PHOTOGRAPHERS_CREATIVE__MUSICIAN".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__OTHER => "ARTISTS_PHOTOGRAPHERS_CREATIVE__OTHER".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__PERFORMING_ARTS_ACTING_MUSIC_DANCE => "ARTISTS_PHOTOGRAPHERS_CREATIVE__PERFORMING_ARTS_ACTING_MUSIC_DANCE".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__PHOTOGRAPHER => "ARTISTS_PHOTOGRAPHERS_CREATIVE__PHOTOGRAPHER".to_string(),
            BusinessSubtypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE__VISUAL_ARTIST => "ARTISTS_PHOTOGRAPHERS_CREATIVE__VISUAL_ARTIST".to_string(),
            BusinessSubtypeValue::AUTOMOTIVE_SALES_AND_REPAIR => "AUTOMOTIVE_SALES_AND_REPAIR".to_string(),
            BusinessSubtypeValue::CHURCH_RELIGIOUS_ORGANIZATION => "CHURCH_RELIGIOUS_ORGANIZATION".to_string(),
            BusinessSubtypeValue::CONSTRUCTION_HOME_IMPROVEMENT__CONTRACTOR => "CONSTRUCTION_HOME_IMPROVEMENT__CONTRACTOR".to_string(),
            BusinessSubtypeValue::CONSTRUCTION_HOME_IMPROVEMENT__ENGINEER => "CONSTRUCTION_HOME_IMPROVEMENT__ENGINEER".to_string(),
            BusinessSubtypeValue::CONSTRUCTION_HOME_IMPROVEMENT__HOME_INSPECTOR => "CONSTRUCTION_HOME_IMPROVEMENT__HOME_INSPECTOR".to_string(),
            BusinessSubtypeValue::CONSTRUCTION_HOME_IMPROVEMENT__OTHER_TRADES => "CONSTRUCTION_HOME_IMPROVEMENT__OTHER_TRADES".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__ACCOUNTANTS_BOOKKEEPERS => "CONSULTANTS_PROFESSIONALS__ACCOUNTANTS_BOOKKEEPERS".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__COMMUNICATIONS => "CONSULTANTS_PROFESSIONALS__COMMUNICATIONS".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__EXECUTIVE_COACH => "CONSULTANTS_PROFESSIONALS__EXECUTIVE_COACH".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__HR_RECRUITMENT_STAFFING => "CONSULTANTS_PROFESSIONALS__HR_RECRUITMENT_STAFFING".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__IT_TECHNICAL => "CONSULTANTS_PROFESSIONALS__IT_TECHNICAL".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__OTHER => "CONSULTANTS_PROFESSIONALS__OTHER".to_string(),
            BusinessSubtypeValue::CONSULTANTS_PROFESSIONALS__SALES => "CONSULTANTS_PROFESSIONALS__SALES".to_string(),
            BusinessSubtypeValue::DESIGN_ARCHITECTURE_ENGINEERING => "DESIGN_ARCHITECTURE_ENGINEERING".to_string(),
            BusinessSubtypeValue::FINANCIAL_SERVICES => "FINANCIAL_SERVICES".to_string(),
            BusinessSubtypeValue::HAIR_SPA_AESTHETICS__HAIR_SALON => "HAIR_SPA_AESTHETICS__HAIR_SALON".to_string(),
            BusinessSubtypeValue::HAIR_SPA_AESTHETICS__MASSAGE => "HAIR_SPA_AESTHETICS__MASSAGE".to_string(),
            BusinessSubtypeValue::HAIR_SPA_AESTHETICS__NAIL_SKIN_AESTHETICS => "HAIR_SPA_AESTHETICS__NAIL_SKIN_AESTHETICS".to_string(),
            BusinessSubtypeValue::HAIR_SPA_AESTHETICS__OTHER => "HAIR_SPA_AESTHETICS__OTHER".to_string(),
            BusinessSubtypeValue::INSURANCE_AGENCY_BROKER => "INSURANCE_AGENCY_BROKER".to_string(),
            BusinessSubtypeValue::LANDLORD_PROPERTY_MANAGER__LANDLORD => "LANDLORD_PROPERTY_MANAGER__LANDLORD".to_string(),
            BusinessSubtypeValue::LANDLORD_PROPERTY_MANAGER__PROPERTY_MANAGER => "LANDLORD_PROPERTY_MANAGER__PROPERTY_MANAGER".to_string(),
            BusinessSubtypeValue::LAWN_CARE_LANDSCAPING => "LAWN_CARE_LANDSCAPING".to_string(),
            BusinessSubtypeValue::LEGAL_SERVICES => "LEGAL_SERVICES".to_string(),
            BusinessSubtypeValue::LODGING_HOTEL_MOTEL => "LODGING_HOTEL_MOTEL".to_string(),
            BusinessSubtypeValue::MANUFACTURER_REPRESENTATIVE_AGENT => "MANUFACTURER_REPRESENTATIVE_AGENT".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__CHIROPRACTOR => "MEDICAL_DENTAL_HEALTH_SERVICE__CHIROPRACTOR".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__DENTIST => "MEDICAL_DENTAL_HEALTH_SERVICE__DENTIST".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__FITNESS => "MEDICAL_DENTAL_HEALTH_SERVICE__FITNESS".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__MASSAGE_THERAPIST => "MEDICAL_DENTAL_HEALTH_SERVICE__MASSAGE_THERAPIST".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__MENTAL_HEALTH => "MEDICAL_DENTAL_HEALTH_SERVICE__MENTAL_HEALTH".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__NUTRITION => "MEDICAL_DENTAL_HEALTH_SERVICE__NUTRITION".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__OCCUP_THERAPIST => "MEDICAL_DENTAL_HEALTH_SERVICE__OCCUP_THERAPIST".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__OTHER => "MEDICAL_DENTAL_HEALTH_SERVICE__OTHER".to_string(),
            BusinessSubtypeValue::MEDICAL_DENTAL_HEALTH_SERVICE__PHYSICAL_THERAPIST => "MEDICAL_DENTAL_HEALTH_SERVICE__PHYSICAL_THERAPIST".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__ASSOCIATION => "NONPROFIT_ASSOCIATIONS_GROUPS__ASSOCIATION".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__CHARITABLE => "NONPROFIT_ASSOCIATIONS_GROUPS__CHARITABLE".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__CLUB => "NONPROFIT_ASSOCIATIONS_GROUPS__CLUB".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__CONDO => "NONPROFIT_ASSOCIATIONS_GROUPS__CONDO".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__OTHER => "NONPROFIT_ASSOCIATIONS_GROUPS__OTHER".to_string(),
            BusinessSubtypeValue::NONPROFIT_ASSOCIATIONS_GROUPS__PARENT_BOOSTER => "NONPROFIT_ASSOCIATIONS_GROUPS__PARENT_BOOSTER".to_string(),
            BusinessSubtypeValue::OTHER__OTHER_PLEASE_SPECIFY => "OTHER__OTHER_PLEASE_SPECIFY".to_string(),
            BusinessSubtypeValue::PRODUCT_PROVIDER__MANUFACTURER => "PRODUCT_PROVIDER__MANUFACTURER".to_string(),
            BusinessSubtypeValue::PRODUCT_PROVIDER__MANUFACTURER_AND_VENDOR => "PRODUCT_PROVIDER__MANUFACTURER_AND_VENDOR".to_string(),
            BusinessSubtypeValue::PRODUCT_PROVIDER__OTHER => "PRODUCT_PROVIDER__OTHER".to_string(),
            BusinessSubtypeValue::PRODUCT_PROVIDER__VENDOR => "PRODUCT_PROVIDER__VENDOR".to_string(),
            BusinessSubtypeValue::REAL_ESTATE_SALES__AGENT => "REAL_ESTATE_SALES__AGENT".to_string(),
            BusinessSubtypeValue::REAL_ESTATE_SALES__BROKER => "REAL_ESTATE_SALES__BROKER".to_string(),
            BusinessSubtypeValue::REAL_ESTATE_SALES__OTHER => "REAL_ESTATE_SALES__OTHER".to_string(),
            BusinessSubtypeValue::RENTAL => "RENTAL".to_string(),
            BusinessSubtypeValue::REPAIR_AND_MAINTENANCE => "REPAIR_AND_MAINTENANCE".to_string(),
            BusinessSubtypeValue::RESTAURANT_CATERER_BAR => "RESTAURANT_CATERER_BAR".to_string(),
            BusinessSubtypeValue::RETAILERS_AND_RESELLERS__EBAY => "RETAILERS_AND_RESELLERS__EBAY".to_string(),
            BusinessSubtypeValue::RETAILERS_AND_RESELLERS__ETSY => "RETAILERS_AND_RESELLERS__ETSY".to_string(),
            BusinessSubtypeValue::RETAILERS_AND_RESELLERS__NON_STORE_RETAILER => "RETAILERS_AND_RESELLERS__NON_STORE_RETAILER".to_string(),
            BusinessSubtypeValue::RETAILERS_AND_RESELLERS__OTHER => "RETAILERS_AND_RESELLERS__OTHER".to_string(),
            BusinessSubtypeValue::RETAILERS_AND_RESELLERS__STORE_RETAILER => "RETAILERS_AND_RESELLERS__STORE_RETAILER".to_string(),
            BusinessSubtypeValue::SALES_INDEPENDENT_AGENT => "SALES_INDEPENDENT_AGENT".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__CLEANING_JANITORIAL_SERVICES => "SERVICE_PROVIDER__CLEANING_JANITORIAL_SERVICES".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__CUSTOMER_SERVICE_SUPPORT => "SERVICE_PROVIDER__CUSTOMER_SERVICE_SUPPORT".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__DOMESTIC_CAREGIVER_EMPLOYER => "SERVICE_PROVIDER__DOMESTIC_CAREGIVER_EMPLOYER".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__FITNESS => "SERVICE_PROVIDER__FITNESS".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__OFFICE_ADMIN_SUPPORT => "SERVICE_PROVIDER__OFFICE_ADMIN_SUPPORT".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__OTHER => "SERVICE_PROVIDER__OTHER".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__PERSONAL_CARE => "SERVICE_PROVIDER__PERSONAL_CARE".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__TELEMARKETING => "SERVICE_PROVIDER__TELEMARKETING".to_string(),
            BusinessSubtypeValue::SERVICE_PROVIDER__TRANSCRIPTION => "SERVICE_PROVIDER__TRANSCRIPTION".to_string(),
            BusinessSubtypeValue::TRANSPORTATION_TRUCKING_DELIVERY => "TRANSPORTATION_TRUCKING_DELIVERY".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__DESIGNER => "WEB_MEDIA_FREELANCER__DESIGNER".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__MARKETING_SOCIAL_MEDIA => "WEB_MEDIA_FREELANCER__MARKETING_SOCIAL_MEDIA".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__OTHER => "WEB_MEDIA_FREELANCER__OTHER".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__PROGRAMMER => "WEB_MEDIA_FREELANCER__PROGRAMMER".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__SEO => "WEB_MEDIA_FREELANCER__SEO".to_string(),
            BusinessSubtypeValue::WEB_MEDIA_FREELANCER__WRITER => "WEB_MEDIA_FREELANCER__WRITER".to_string(),
            BusinessSubtypeValue::WHOLESALE_DISTRIBUTION_SALES => "WHOLESALE_DISTRIBUTION_SALES".to_string(),
        }
    }
}

/// Area of focus of a business.
#[derive(Debug)]
/// Area of focus of a business.
pub enum BusinessTypeValue {
    /// Artists, Photographers & Creative Types
    ARTISTS_PHOTOGRAPHERS_CREATIVE,
    /// Consultants & Professionals
    CONSULTANTS_PROFESSIONALS,
    /// Financial Services
    FINANCE_INSURANCE,
    /// Hair, Spa & Aesthetics
    HAIR_SPA_AESTHETICS,
    /// Medical, Dental, Health
    MEDICAL_DENTAL_HEALTH_SERVICE,
    /// Non-profits, Associations & Groups
    NONPROFIT_ASSOCIATIONS_GROUPS,
    /// Other (please specify)
    OTHER(String),
    /// General: I make or sell a PRODUCT
    PRODUCT_PROVIDER,
    /// Real Estate, Construction & Home Improvement
    REALESTATE_HOME,
    /// Retailers, Resellers & Sales
    RETAILERS_AND_RESELLERS,
    /// General: I provide a SERVICE
    SERVICE_PROVIDER,
    /// Web, Tech & Media
    WEB_MEDIA_FREELANCER,
}

impl BusinessTypeValue {
    pub fn to_string(&self) -> String {
        match self {
            BusinessTypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE => {
                "ARTISTS_PHOTOGRAPHERS_CREATIVE".to_string()
            }
            BusinessTypeValue::CONSULTANTS_PROFESSIONALS => "CONSULTANTS_PROFESSIONALS".to_string(),
            BusinessTypeValue::FINANCE_INSURANCE => "FINANCE_INSURANCE".to_string(),
            BusinessTypeValue::HAIR_SPA_AESTHETICS => "HAIR_SPA_AESTHETICS".to_string(),
            BusinessTypeValue::MEDICAL_DENTAL_HEALTH_SERVICE => {
                "MEDICAL_DENTAL_HEALTH_SERVICE".to_string()
            }
            BusinessTypeValue::NONPROFIT_ASSOCIATIONS_GROUPS => {
                "NONPROFIT_ASSOCIATIONS_GROUPS".to_string()
            }
            BusinessTypeValue::OTHER(value) => format!("OTHER({})", value),
            BusinessTypeValue::PRODUCT_PROVIDER => "PRODUCT_PROVIDER".to_string(),
            BusinessTypeValue::REALESTATE_HOME => "REALESTATE_HOME".to_string(),
            BusinessTypeValue::RETAILERS_AND_RESELLERS => "RETAILERS_AND_RESELLERS".to_string(),
            BusinessTypeValue::SERVICE_PROVIDER => "SERVICE_PROVIDER".to_string(),
            BusinessTypeValue::WEB_MEDIA_FREELANCER => "WEB_MEDIA_FREELANCER".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum CountryCode {
    /// Andorra
    AD,
    /// United Arab Emirates
    AE,
    /// Afghanistan
    AF,
    /// Antigua and Barbuda
    AG,
    /// Anguilla
    AI,
    /// Albania
    AL,
    /// Armenia
    AM,
    /// Angola
    AO,
    /// Antarctica
    AQ,
    /// Argentina
    AR,
    /// American Samoa
    AS,
    /// Austria
    AT,
    /// Australia
    AU,
    /// Aruba
    AW,
    /// Åland Islands
    AX,
    /// Azerbaijan
    AZ,
    /// Bosnia and Herzegovina
    BA,
    /// Barbados
    BB,
    /// Bangladesh
    BD,
    /// Belgium
    BE,
    /// Burkina Faso
    BF,
    /// Bulgaria
    BG,
    /// Bahrain
    BH,
    /// Burundi
    BI,
    /// Benin
    BJ,
    /// Saint Barthélemy
    BL,
    /// Bermuda
    BM,
    /// Brunei Darussalam
    BN,
    /// Bolivia, Plurinational State of
    BO,
    /// Bonaire, Sint Eustatius and Saba
    BQ,
    /// Brazil
    BR,
    /// Bahamas
    BS,
    /// Bhutan
    BT,
    /// Bouvet Island
    BV,
    /// Botswana
    BW,
    /// Belarus
    BY,
    /// Belize
    BZ,
    /// Canada
    CA,
    /// Cocos (Keeling) Islands
    CC,
    /// Congo, The Democratic Republic of the
    CD,
    /// Central African Republic
    CF,
    /// Congo
    CG,
    /// Switzerland
    CH,
    /// Côte d'Ivoire
    CI,
    /// Cook Islands
    CK,
    /// Chile
    CL,
    /// Cameroon
    CM,
    /// China
    CN,
    /// Colombia
    CO,
    /// Costa Rica
    CR,
    /// Cuba
    CU,
    /// Cape Verde
    CV,
    /// Curaçao
    CW,
    /// Christmas Island
    CX,
    /// Cyprus
    CY,
    /// Czech Republic
    CZ,
    /// Germany
    DE,
    /// Djibouti
    DJ,
    /// Denmark
    DK,
    /// Dominica
    DM,
    /// Dominican Republic
    DO,
    /// Algeria
    DZ,
    /// Ecuador
    EC,
    /// Estonia
    EE,
    /// Egypt
    EG,
    /// Western Sahara
    EH,
    /// Eritrea
    ER,
    /// Spain
    ES,
    /// Ethiopia
    ET,
    /// Finland
    FI,
    /// Fiji
    FJ,
    /// Falkland Islands
    FK,
    /// Micronesia, Federated States of
    FM,
    /// Faroe Islands
    FO,
    /// France
    FR,
    /// Gabon
    GA,
    /// United Kingdom
    GB,
    /// Grenada
    GD,
    /// Georgia
    GE,
    /// French Guiana
    GF,
    /// Guernsey
    GG,
    /// Ghana
    GH,
    /// Gibraltar
    GI,
    /// Greenland
    GL,
    /// Gambia
    GM,
    /// Guinea
    GN,
    /// Guadeloupe
    GP,
    /// Equatorial Guinea
    GQ,
    /// Greece
    GR,
    /// South Georgia and the South Sandwich Islands
    GS,
    /// Guatemala
    GT,
    /// Guam
    GU,
    /// Guinea-Bissau
    GW,
    /// Guyana
    GY,
    /// Hong Kong
    HK,
    /// Heard Island and McDonald Islands
    HM,
    /// Honduras
    HN,
    /// Croatia
    HR,
    /// Haiti
    HT,
    /// Hungary
    HU,
    /// Indonesia
    ID,
    /// Ireland
    IE,
    /// Israel
    IL,
    /// Isle of Man
    IM,
    /// India
    IN,
    /// British Indian Ocean Territory
    IO,
    /// Iraq
    IQ,
    /// Iran
    IR,
    /// Iceland
    IS,
    /// Italy
    IT,
    /// Jersey
    JE,
    /// Jamaica
    JM,
    /// Jordan
    JO,
    /// Japan
    JP,
    /// Kenya
    KE,
    /// Kyrgyzstan
    KG,
    /// Cambodia
    KH,
    /// Kiribati
    KI,
    /// Comoros
    KM,
    /// Saint Kitts and Nevis
    KN,
    /// Korea, Democratic People's Republic of
    KP,
    /// Korea, Republic of
    KR,
    /// Kuwait
    KW,
    /// Cayman Islands
    KY,
    /// Kazakhstan
    KZ,
    /// Lao People's Democratic Republic
    LA,
    /// Lebanon
    LB,
    /// Saint Lucia
    LC,
    /// Liechtenstein
    LI,
    /// Sri Lanka
    LK,
    /// Liberia
    LR,
    /// Lesotho
    LS,
    /// Lithuania
    LT,
    /// Luxembourg
    LU,
    /// Latvia
    LV,
    /// Libya
    LY,
    /// Morocco
    MA,
    /// Monaco
    MC,
    /// Moldova, Republic of
    MD,
    /// Montenegro
    ME,
    /// Saint Martin
    MF,
    /// Madagascar
    MG,
    /// Marshall Islands
    MH,
    /// North Macedonia
    MK,
    /// Mali
    ML,
    /// Myanmar
    MM,
    /// Mongolia
    MN,
    /// Macao
    MO,
    /// Northern Mariana Islands
    MP,
    /// Martinique
    MQ,
    /// Mauritania
    MR,
    /// Montserrat
    MS,
    /// Malta
    MT,
    /// Mauritius
    MU,
    /// Maldives
    MV,
    /// Malawi
    MW,
    /// Mexico
    MX,
    /// Malaysia
    MY,
    /// Mozambique
    MZ,
    /// Namibia
    NA,
    /// New Caledonia
    NC,
    /// Niger
    NE,
    /// Norfolk Island
    NF,
    /// Nigeria
    NG,
    /// Nicaragua
    NI,
    /// Netherlands
    NL,
    /// Norway
    NO,
    /// Nepal
    NP,
    /// Nauru
    NR,
    /// Niue
    NU,
    /// New Zealand
    NZ,
    /// Oman
    OM,
    /// Panama
    PA,
    /// Peru
    PE,
    /// French Polynesia
    PF,
    /// Papua New Guinea
    PG,
    /// Philippines
    PH,
    /// Pakistan
    PK,
    /// Poland
    PL,
    /// Saint Pierre and Miquelon
    PM,
    /// Pitcairn
    PN,
    /// Puerto Rico
    PR,
    /// Palestine
    PS,
    /// Portugal
    PT,
    /// Palau
    PW,
    /// Paraguay
    PY,
    /// Qatar
    QA,
    /// Réunion
    RE,
    /// Romania
    RO,
    /// Serbia
    RS,
    /// Russian Federation
    RU,
    /// Rwanda
    RW,
    /// Saudi Arabia
    SA,
    /// Solomon Islands
    SB,
    /// Seychelles
    SC,
    /// Sudan
    SD,
    /// Sweden
    SE,
    /// Singapore
    SG,
    /// Saint Helena, Ascension and Tristan da Cunha
    SH,
    /// Slovenia
    SI,
    /// Svalbard and Jan Mayen
    SJ,
    /// Slovakia
    SK,
    /// Sierra Leone
    SL,
    /// San Marino
    SM,
    /// Senegal
    SN,
    /// Somalia
    SO,
    /// Suriname
    SR,
    /// South Sudan
    SS,
    /// Sao Tome and Principe
    ST,
    /// El Salvador
    SV,
    /// Sint Maarten
    SX,
    /// Syria
    SY,
    /// Eswatini
    SZ,
    /// Turks and Caicos Islands
    TC,
    /// Chad
    TD,
    /// French Southern Territories
    TF,
    /// Togo
    TG,
    /// Thailand
    TH,
    /// Tajikistan
    TJ,
    /// Tokelau
    TK,
    /// Timor-Leste
    TL,
    /// Turkmenistan
    TM,
    /// Tunisia
    TN,
    /// Tonga
    TO,
    /// Turkey
    TR,
    /// Trinidad and Tobago
    TT,
    /// Tuvalu
    TV,
    /// Taiwan
    TW,
    /// Tanzania, United Republic of
    TZ,
    /// Ukraine
    UA,
    /// Uganda
    UG,
    /// United States Minor Outlying Islands
    UM,
    /// United States
    US,
    /// Uruguay
    UY,
    /// Uzbekistan
    UZ,
    /// Holy See
    VA,
    /// Saint Vincent and the Grenadines
    VC,
    /// Venezuela, Bolivarian Republic of
    VE,
    /// Virgin Islands (British)
    VG,
    /// Virgin Islands (U.S)
    VI,
    /// Viet Nam
    VN,
    /// Vanuatu
    VU,
    /// Wallis and Futuna
    WF,
    /// Samoa
    WS,
    /// Yemen
    YE,
    /// Mayotte
    YT,
    /// South Africa
    ZA,
    /// Zambia
    ZM,
    /// Zimbabwe
    ZW,
}

impl CountryCode {
    pub fn to_name(&self) -> String {
        match self {
            CountryCode::AD => "Andorra".to_string(),
            CountryCode::AE => "United Arab Emirates".to_string(),
            CountryCode::AF => "Afghanistan".to_string(),
            CountryCode::AG => "Antigua and Barbuda".to_string(),
            CountryCode::AI => "Anguilla".to_string(),
            CountryCode::AL => "Albania".to_string(),
            CountryCode::AM => "Armenia".to_string(),
            CountryCode::AO => "Angola".to_string(),
            CountryCode::AQ => "Antarctica".to_string(),
            CountryCode::AR => "Argentina".to_string(),
            CountryCode::AS => "American Samoa".to_string(),
            CountryCode::AT => "Austria".to_string(),
            CountryCode::AU => "Australia".to_string(),
            CountryCode::AW => "Aruba".to_string(),
            CountryCode::AX => "Åland Islands".to_string(),
            CountryCode::AZ => "Azerbaijan".to_string(),
            CountryCode::BA => "Bosnia and Herzegovina".to_string(),
            CountryCode::BB => "Barbados".to_string(),
            CountryCode::BD => "Bangladesh".to_string(),
            CountryCode::BE => "Belgium".to_string(),
            CountryCode::BF => "Burkina Faso".to_string(),
            CountryCode::BG => "Bulgaria".to_string(),
            CountryCode::BH => "Bahrain".to_string(),
            CountryCode::BI => "Burundi".to_string(),
            CountryCode::BJ => "Benin".to_string(),
            CountryCode::BL => "Saint Barthélemy".to_string(),
            CountryCode::BM => "Bermuda".to_string(),
            CountryCode::BN => "Brunei Darussalam".to_string(),
            CountryCode::BO => "Bolivia, Plurinational State of".to_string(),
            CountryCode::BQ => "Bonaire, Sint Eustatius and Saba".to_string(),
            CountryCode::BR => "Brazil".to_string(),
            CountryCode::BS => "Bahamas".to_string(),
            CountryCode::BT => "Bhutan".to_string(),
            CountryCode::BV => "Bouvet Island".to_string(),
            CountryCode::BW => "Botswana".to_string(),
            CountryCode::BY => "Belarus".to_string(),
            CountryCode::BZ => "Belize".to_string(),
            CountryCode::CA => "Canada".to_string(),
            CountryCode::CC => "Cocos (Keeling) Islands".to_string(),
            CountryCode::CD => "Congo, The Democratic Republic of the".to_string(),
            CountryCode::CF => "Central African Republic".to_string(),
            CountryCode::CG => "Congo".to_string(),
            CountryCode::CH => "Switzerland".to_string(),
            CountryCode::CI => "Côte d'Ivoire".to_string(),
            CountryCode::CK => "Cook Islands".to_string(),
            CountryCode::CL => "Chile".to_string(),
            CountryCode::CM => "Cameroon".to_string(),
            CountryCode::CN => "China".to_string(),
            CountryCode::CO => "Colombia".to_string(),
            CountryCode::CR => "Costa Rica".to_string(),
            CountryCode::CU => "Cuba".to_string(),
            CountryCode::CV => "Cape Verde".to_string(),
            CountryCode::CW => "Curaçao".to_string(),
            CountryCode::CX => "Christmas Island".to_string(),
            CountryCode::CY => "Cyprus".to_string(),
            CountryCode::CZ => "Czech Republic".to_string(),
            CountryCode::DE => "Germany".to_string(),
            CountryCode::DJ => "Djibouti".to_string(),
            CountryCode::DK => "Denmark".to_string(),
            CountryCode::DM => "Dominica".to_string(),
            CountryCode::DO => "Dominican Republic".to_string(),
            CountryCode::DZ => "Algeria".to_string(),
            CountryCode::EC => "Ecuador".to_string(),
            CountryCode::EE => "Estonia".to_string(),
            CountryCode::EG => "Egypt".to_string(),
            CountryCode::EH => "Western Sahara".to_string(),
            CountryCode::ER => "Eritrea".to_string(),
            CountryCode::ES => "Spain".to_string(),
            CountryCode::ET => "Ethiopia".to_string(),
            CountryCode::FI => "Finland".to_string(),
            CountryCode::FJ => "Fiji".to_string(),
            CountryCode::FK => "Falkland Islands".to_string(),
            CountryCode::FM => "Micronesia, Federated States of".to_string(),
            CountryCode::FO => "Faroe Islands".to_string(),
            CountryCode::FR => "France".to_string(),
            CountryCode::GA => "Gabon".to_string(),
            CountryCode::GB => "United Kingdom".to_string(),
            CountryCode::GD => "Grenada".to_string(),
            CountryCode::GE => "Georgia".to_string(),
            CountryCode::GF => "French Guiana".to_string(),
            CountryCode::GG => "Guernsey".to_string(),
            CountryCode::GH => "Ghana".to_string(),
            CountryCode::GI => "Gibraltar".to_string(),
            CountryCode::GL => "Greenland".to_string(),
            CountryCode::GM => "Gambia".to_string(),
            CountryCode::GN => "Guinea".to_string(),
            CountryCode::GP => "Guadeloupe".to_string(),
            CountryCode::GQ => "Equatorial Guinea".to_string(),
            CountryCode::GR => "Greece".to_string(),
            CountryCode::GS => "South Georgia and the South Sandwich Islands".to_string(),
            CountryCode::GT => "Guatemala".to_string(),
            CountryCode::GU => "Guam".to_string(),
            CountryCode::GW => "Guinea-Bissau".to_string(),
            CountryCode::GY => "Guyana".to_string(),
            CountryCode::HK => "Hong Kong".to_string(),
            CountryCode::HM => "Heard Island and McDonald Islands".to_string(),
            CountryCode::HN => "Honduras".to_string(),
            CountryCode::HR => "Croatia".to_string(),
            CountryCode::HT => "Haiti".to_string(),
            CountryCode::HU => "Hungary".to_string(),
            CountryCode::ID => "Indonesia".to_string(),
            CountryCode::IE => "Ireland".to_string(),
            CountryCode::IL => "Israel".to_string(),
            CountryCode::IM => "Isle of Man".to_string(),
            CountryCode::IN => "India".to_string(),
            CountryCode::IO => "British Indian Ocean Territory".to_string(),
            CountryCode::IQ => "Iraq".to_string(),
            CountryCode::IR => "Iran".to_string(),
            CountryCode::IS => "Iceland".to_string(),
            CountryCode::IT => "Italy".to_string(),
            CountryCode::JE => "Jersey".to_string(),
            CountryCode::JM => "Jamaica".to_string(),
            CountryCode::JO => "Jordan".to_string(),
            CountryCode::JP => "Japan".to_string(),
            CountryCode::KE => "Kenya".to_string(),
            CountryCode::KG => "Kyrgyzstan".to_string(),
            CountryCode::KH => "Cambodia".to_string(),
            CountryCode::KI => "Kiribati".to_string(),
            CountryCode::KM => "Comoros".to_string(),
            CountryCode::KN => "Saint Kitts and Nevis".to_string(),
            CountryCode::KP => "Korea, Democratic People's Republic of".to_string(),
            CountryCode::KR => "Korea, Republic of".to_string(),
            CountryCode::KW => "Kuwait".to_string(),
            CountryCode::KY => "Cayman Islands".to_string(),
            CountryCode::KZ => "Kazakhstan".to_string(),
            CountryCode::LA => "Lao People's Democratic Republic".to_string(),
            CountryCode::LB => "Lebanon".to_string(),
            CountryCode::LC => "Saint Lucia".to_string(),
            CountryCode::LI => "Liechtenstein".to_string(),
            CountryCode::LK => "Sri Lanka".to_string(),
            CountryCode::LR => "Liberia".to_string(),
            CountryCode::LS => "Lesotho".to_string(),
            CountryCode::LT => "Lithuania".to_string(),
            CountryCode::LU => "Luxembourg".to_string(),
            CountryCode::LV => "Latvia".to_string(),
            CountryCode::LY => "Libya".to_string(),
            CountryCode::MA => "Morocco".to_string(),
            CountryCode::MC => "Monaco".to_string(),
            CountryCode::MD => "Moldova, Republic of".to_string(),
            CountryCode::ME => "Montenegro".to_string(),
            CountryCode::MF => "Saint Martin".to_string(),
            CountryCode::MG => "Madagascar".to_string(),
            CountryCode::MH => "Marshall Islands".to_string(),
            CountryCode::MK => "North Macedonia".to_string(),
            CountryCode::ML => "Mali".to_string(),
            CountryCode::MM => "Myanmar".to_string(),
            CountryCode::MN => "Mongolia".to_string(),
            CountryCode::MO => "Macao".to_string(),
            CountryCode::MP => "Northern Mariana Islands".to_string(),
            CountryCode::MQ => "Martinique".to_string(),
            CountryCode::MR => "Mauritania".to_string(),
            CountryCode::MS => "Montserrat".to_string(),
            CountryCode::MT => "Malta".to_string(),
            CountryCode::MU => "Mauritius".to_string(),
            CountryCode::MV => "Maldives".to_string(),
            CountryCode::MW => "Malawi".to_string(),
            CountryCode::MX => "Mexico".to_string(),
            CountryCode::MY => "Malaysia".to_string(),
            CountryCode::MZ => "Mozambique".to_string(),
            CountryCode::NA => "Namibia".to_string(),
            CountryCode::NC => "New Caledonia".to_string(),
            CountryCode::NE => "Niger".to_string(),
            CountryCode::NF => "Norfolk Island".to_string(),
            CountryCode::NG => "Nigeria".to_string(),
            CountryCode::NI => "Nicaragua".to_string(),
            CountryCode::NL => "Netherlands".to_string(),
            CountryCode::NO => "Norway".to_string(),
            CountryCode::NP => "Nepal".to_string(),
            CountryCode::NR => "Nauru".to_string(),
            CountryCode::NU => "Niue".to_string(),
            CountryCode::NZ => "New Zealand".to_string(),
            CountryCode::OM => "Oman".to_string(),
            CountryCode::PA => "Panama".to_string(),
            CountryCode::PE => "Peru".to_string(),
            CountryCode::PF => "French Polynesia".to_string(),
            CountryCode::PG => "Papua New Guinea".to_string(),
            CountryCode::PH => "Philippines".to_string(),
            CountryCode::PK => "Pakistan".to_string(),
            CountryCode::PL => "Poland".to_string(),
            CountryCode::PM => "Saint Pierre and Miquelon".to_string(),
            CountryCode::PN => "Pitcairn".to_string(),
            CountryCode::PR => "Puerto Rico".to_string(),
            CountryCode::PS => "Palestine".to_string(),
            CountryCode::PT => "Portugal".to_string(),
            CountryCode::PW => "Palau".to_string(),
            CountryCode::PY => "Paraguay".to_string(),
            CountryCode::QA => "Qatar".to_string(),
            CountryCode::RE => "Réunion".to_string(),
            CountryCode::RO => "Romania".to_string(),
            CountryCode::RS => "Serbia".to_string(),
            CountryCode::RU => "Russian Federation".to_string(),
            CountryCode::RW => "Rwanda".to_string(),
            CountryCode::SA => "Saudi Arabia".to_string(),
            CountryCode::SB => "Solomon Islands".to_string(),
            CountryCode::SC => "Seychelles".to_string(),
            CountryCode::SD => "Sudan".to_string(),
            CountryCode::SE => "Sweden".to_string(),
            CountryCode::SG => "Singapore".to_string(),
            CountryCode::SH => "Saint Helena, Ascension and Tristan da Cunha".to_string(),
            CountryCode::SI => "Slovenia".to_string(),
            CountryCode::SJ => "Svalbard and Jan Mayen".to_string(),
            CountryCode::SK => "Slovakia".to_string(),
            CountryCode::SL => "Sierra Leone".to_string(),
            CountryCode::SM => "San Marino".to_string(),
            CountryCode::SN => "Senegal".to_string(),
            CountryCode::SO => "Somalia".to_string(),
            CountryCode::SR => "Suriname".to_string(),
            CountryCode::SS => "South Sudan".to_string(),
            CountryCode::ST => "Sao Tome and Principe".to_string(),
            CountryCode::SV => "El Salvador".to_string(),
            CountryCode::SX => "Sint Maarten".to_string(),
            CountryCode::SY => "Syria".to_string(),
            CountryCode::SZ => "Eswatini".to_string(),
            CountryCode::TC => "Turks and Caicos Islands".to_string(),
            CountryCode::TD => "Chad".to_string(),
            CountryCode::TF => "French Southern Territories".to_string(),
            CountryCode::TG => "Togo".to_string(),
            CountryCode::TH => "Thailand".to_string(),
            CountryCode::TJ => "Tajikistan".to_string(),
            CountryCode::TK => "Tokelau".to_string(),
            CountryCode::TL => "Timor-Leste".to_string(),
            CountryCode::TM => "Turkmenistan".to_string(),
            CountryCode::TN => "Tunisia".to_string(),
            CountryCode::TO => "Tonga".to_string(),
            CountryCode::TR => "Turkey".to_string(),
            CountryCode::TT => "Trinidad and Tobago".to_string(),
            CountryCode::TV => "Tuvalu".to_string(),
            CountryCode::TW => "Taiwan".to_string(),
            CountryCode::TZ => "Tanzania, United Republic of".to_string(),
            CountryCode::UA => "Ukraine".to_string(),
            CountryCode::UG => "Uganda".to_string(),
            CountryCode::UM => "United States Minor Outlying Islands".to_string(),
            CountryCode::US => "United States".to_string(),
            CountryCode::UY => "Uruguay".to_string(),
            CountryCode::UZ => "Uzbekistan".to_string(),
            CountryCode::VA => "Holy See".to_string(),
            CountryCode::VC => "Saint Vincent and the Grenadines".to_string(),
            CountryCode::VE => "Venezuela, Bolivarian Republic of".to_string(),
            CountryCode::VG => "Virgin Islands (British)".to_string(),
            CountryCode::VI => "Virgin Islands (U.S)".to_string(),
            CountryCode::VN => "Viet Nam".to_string(),
            CountryCode::VU => "Vanuatu".to_string(),
            CountryCode::WF => "Wallis and Futuna".to_string(),
            CountryCode::WS => "Samoa".to_string(),
            CountryCode::YE => "Yemen".to_string(),
            CountryCode::YT => "Mayotte".to_string(),
            CountryCode::ZA => "South Africa".to_string(),
            CountryCode::ZM => "Zambia".to_string(),
            CountryCode::ZW => "Zimbabwe".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CountryCode::AD => "AD".to_string(),
            CountryCode::AE => "AE".to_string(),
            CountryCode::AF => "AF".to_string(),
            CountryCode::AG => "AG".to_string(),
            CountryCode::AI => "AI".to_string(),
            CountryCode::AL => "AL".to_string(),
            CountryCode::AM => "AM".to_string(),
            CountryCode::AO => "AO".to_string(),
            CountryCode::AQ => "AQ".to_string(),
            CountryCode::AR => "AR".to_string(),
            CountryCode::AS => "AS".to_string(),
            CountryCode::AT => "AT".to_string(),
            CountryCode::AU => "AU".to_string(),
            CountryCode::AW => "AW".to_string(),
            CountryCode::AX => "AX".to_string(),
            CountryCode::AZ => "AZ".to_string(),
            CountryCode::BA => "BA".to_string(),
            CountryCode::BB => "BB".to_string(),
            CountryCode::BD => "BD".to_string(),
            CountryCode::BE => "BE".to_string(),
            CountryCode::BF => "BF".to_string(),
            CountryCode::BG => "BG".to_string(),
            CountryCode::BH => "BH".to_string(),
            CountryCode::BI => "BI".to_string(),
            CountryCode::BJ => "BJ".to_string(),
            CountryCode::BL => "BL".to_string(),
            CountryCode::BM => "BM".to_string(),
            CountryCode::BN => "BN".to_string(),
            CountryCode::BO => "BO".to_string(),
            CountryCode::BQ => "BQ".to_string(),
            CountryCode::BR => "BR".to_string(),
            CountryCode::BS => "BS".to_string(),
            CountryCode::BT => "BT".to_string(),
            CountryCode::BV => "BV".to_string(),
            CountryCode::BW => "BW".to_string(),
            CountryCode::BY => "BY".to_string(),
            CountryCode::BZ => "BZ".to_string(),
            CountryCode::CA => "CA".to_string(),
            CountryCode::CC => "CC".to_string(),
            CountryCode::CD => "CD".to_string(),
            CountryCode::CF => "CF".to_string(),
            CountryCode::CG => "CG".to_string(),
            CountryCode::CH => "CH".to_string(),
            CountryCode::CI => "CI".to_string(),
            CountryCode::CK => "CK".to_string(),
            CountryCode::CL => "CL".to_string(),
            CountryCode::CM => "CM".to_string(),
            CountryCode::CN => "CN".to_string(),
            CountryCode::CO => "CO".to_string(),
            CountryCode::CR => "CR".to_string(),
            CountryCode::CU => "CU".to_string(),
            CountryCode::CV => "CV".to_string(),
            CountryCode::CW => "CW".to_string(),
            CountryCode::CX => "CX".to_string(),
            CountryCode::CY => "CY".to_string(),
            CountryCode::CZ => "CZ".to_string(),
            CountryCode::DE => "DE".to_string(),
            CountryCode::DJ => "DJ".to_string(),
            CountryCode::DK => "DK".to_string(),
            CountryCode::DM => "DM".to_string(),
            CountryCode::DO => "DO".to_string(),
            CountryCode::DZ => "DZ".to_string(),
            CountryCode::EC => "EC".to_string(),
            CountryCode::EE => "EE".to_string(),
            CountryCode::EG => "EG".to_string(),
            CountryCode::EH => "EH".to_string(),
            CountryCode::ER => "ER".to_string(),
            CountryCode::ES => "ES".to_string(),
            CountryCode::ET => "ET".to_string(),
            CountryCode::FI => "FI".to_string(),
            CountryCode::FJ => "FJ".to_string(),
            CountryCode::FK => "FK".to_string(),
            CountryCode::FM => "FM".to_string(),
            CountryCode::FO => "FO".to_string(),
            CountryCode::FR => "FR".to_string(),
            CountryCode::GA => "GA".to_string(),
            CountryCode::GB => "GB".to_string(),
            CountryCode::GD => "GD".to_string(),
            CountryCode::GE => "GE".to_string(),
            CountryCode::GF => "GF".to_string(),
            CountryCode::GG => "GG".to_string(),
            CountryCode::GH => "GH".to_string(),
            CountryCode::GI => "GI".to_string(),
            CountryCode::GL => "GL".to_string(),
            CountryCode::GM => "GM".to_string(),
            CountryCode::GN => "GN".to_string(),
            CountryCode::GP => "GP".to_string(),
            CountryCode::GQ => "GQ".to_string(),
            CountryCode::GR => "GR".to_string(),
            CountryCode::GS => "GS".to_string(),
            CountryCode::GT => "GT".to_string(),
            CountryCode::GU => "GU".to_string(),
            CountryCode::GW => "GW".to_string(),
            CountryCode::GY => "GY".to_string(),
            CountryCode::HK => "HK".to_string(),
            CountryCode::HM => "HM".to_string(),
            CountryCode::HN => "HN".to_string(),
            CountryCode::HR => "HR".to_string(),
            CountryCode::HT => "HT".to_string(),
            CountryCode::HU => "HU".to_string(),
            CountryCode::ID => "ID".to_string(),
            CountryCode::IE => "IE".to_string(),
            CountryCode::IL => "IL".to_string(),
            CountryCode::IM => "IM".to_string(),
            CountryCode::IN => "IN".to_string(),
            CountryCode::IO => "IO".to_string(),
            CountryCode::IQ => "IQ".to_string(),
            CountryCode::IR => "IR".to_string(),
            CountryCode::IS => "IS".to_string(),
            CountryCode::IT => "IT".to_string(),
            CountryCode::JE => "JE".to_string(),
            CountryCode::JM => "JM".to_string(),
            CountryCode::JO => "JO".to_string(),
            CountryCode::JP => "JP".to_string(),
            CountryCode::KE => "KE".to_string(),
            CountryCode::KG => "KG".to_string(),
            CountryCode::KH => "KH".to_string(),
            CountryCode::KI => "KI".to_string(),
            CountryCode::KM => "KM".to_string(),
            CountryCode::KN => "KN".to_string(),
            CountryCode::KP => "KP".to_string(),
            CountryCode::KR => "KR".to_string(),
            CountryCode::KW => "KW".to_string(),
            CountryCode::KY => "KY".to_string(),
            CountryCode::KZ => "KZ".to_string(),
            CountryCode::LA => "LA".to_string(),
            CountryCode::LB => "LB".to_string(),
            CountryCode::LC => "LC".to_string(),
            CountryCode::LI => "LI".to_string(),
            CountryCode::LK => "LK".to_string(),
            CountryCode::LR => "LR".to_string(),
            CountryCode::LS => "LS".to_string(),
            CountryCode::LT => "LT".to_string(),
            CountryCode::LU => "LU".to_string(),
            CountryCode::LV => "LV".to_string(),
            CountryCode::LY => "LY".to_string(),
            CountryCode::MA => "MA".to_string(),
            CountryCode::MC => "MC".to_string(),
            CountryCode::MD => "MD".to_string(),
            CountryCode::ME => "ME".to_string(),
            CountryCode::MF => "MF".to_string(),
            CountryCode::MG => "MG".to_string(),
            CountryCode::MH => "MH".to_string(),
            CountryCode::MK => "MK".to_string(),
            CountryCode::ML => "ML".to_string(),
            CountryCode::MM => "MM".to_string(),
            CountryCode::MN => "MN".to_string(),
            CountryCode::MO => "MO".to_string(),
            CountryCode::MP => "MP".to_string(),
            CountryCode::MQ => "MQ".to_string(),
            CountryCode::MR => "MR".to_string(),
            CountryCode::MS => "MS".to_string(),
            CountryCode::MT => "MT".to_string(),
            CountryCode::MU => "MU".to_string(),
            CountryCode::MV => "MV".to_string(),
            CountryCode::MW => "MW".to_string(),
            CountryCode::MX => "MX".to_string(),
            CountryCode::MY => "MY".to_string(),
            CountryCode::MZ => "MZ".to_string(),
            CountryCode::NA => "NA".to_string(),
            CountryCode::NC => "NC".to_string(),
            CountryCode::NE => "NE".to_string(),
            CountryCode::NF => "NF".to_string(),
            CountryCode::NG => "NG".to_string(),
            CountryCode::NI => "NI".to_string(),
            CountryCode::NL => "NL".to_string(),
            CountryCode::NO => "NO".to_string(),
            CountryCode::NP => "NP".to_string(),
            CountryCode::NR => "NR".to_string(),
            CountryCode::NU => "NU".to_string(),
            CountryCode::NZ => "NZ".to_string(),
            CountryCode::OM => "OM".to_string(),
            CountryCode::PA => "PA".to_string(),
            CountryCode::PE => "PE".to_string(),
            CountryCode::PF => "PF".to_string(),
            CountryCode::PG => "PG".to_string(),
            CountryCode::PH => "PH".to_string(),
            CountryCode::PK => "PK".to_string(),
            CountryCode::PL => "PL".to_string(),
            CountryCode::PM => "PM".to_string(),
            CountryCode::PN => "PN".to_string(),
            CountryCode::PR => "PR".to_string(),
            CountryCode::PS => "PS".to_string(),
            CountryCode::PT => "PT".to_string(),
            CountryCode::PW => "PW".to_string(),
            CountryCode::PY => "PY".to_string(),
            CountryCode::QA => "QA".to_string(),
            CountryCode::RE => "RE".to_string(),
            CountryCode::RO => "RO".to_string(),
            CountryCode::RS => "RS".to_string(),
            CountryCode::RU => "RU".to_string(),
            CountryCode::RW => "RW".to_string(),
            CountryCode::SA => "SA".to_string(),
            CountryCode::SB => "SB".to_string(),
            CountryCode::SC => "SC".to_string(),
            CountryCode::SD => "SD".to_string(),
            CountryCode::SE => "SE".to_string(),
            CountryCode::SG => "SG".to_string(),
            CountryCode::SH => "SH".to_string(),
            CountryCode::SI => "SI".to_string(),
            CountryCode::SJ => "SJ".to_string(),
            CountryCode::SK => "SK".to_string(),
            CountryCode::SL => "SL".to_string(),
            CountryCode::SM => "SM".to_string(),
            CountryCode::SN => "SN".to_string(),
            CountryCode::SO => "SO".to_string(),
            CountryCode::SR => "SR".to_string(),
            CountryCode::SS => "SS".to_string(),
            CountryCode::ST => "ST".to_string(),
            CountryCode::SV => "SV".to_string(),
            CountryCode::SX => "SX".to_string(),
            CountryCode::SY => "SY".to_string(),
            CountryCode::SZ => "SZ".to_string(),
            CountryCode::TC => "TC".to_string(),
            CountryCode::TD => "TD".to_string(),
            CountryCode::TF => "TF".to_string(),
            CountryCode::TG => "TG".to_string(),
            CountryCode::TH => "TH".to_string(),
            CountryCode::TJ => "TJ".to_string(),
            CountryCode::TK => "TK".to_string(),
            CountryCode::TL => "TL".to_string(),
            CountryCode::TM => "TM".to_string(),
            CountryCode::TN => "TN".to_string(),
            CountryCode::TO => "TO".to_string(),
            CountryCode::TR => "TR".to_string(),
            CountryCode::TT => "TT".to_string(),
            CountryCode::TV => "TV".to_string(),
            CountryCode::TW => "TW".to_string(),
            CountryCode::TZ => "TZ".to_string(),
            CountryCode::UA => "UA".to_string(),
            CountryCode::UG => "UG".to_string(),
            CountryCode::UM => "UM".to_string(),
            CountryCode::US => "US".to_string(),
            CountryCode::UY => "UY".to_string(),
            CountryCode::UZ => "UZ".to_string(),
            CountryCode::VA => "VA".to_string(),
            CountryCode::VC => "VC".to_string(),
            CountryCode::VE => "VE".to_string(),
            CountryCode::VG => "VG".to_string(),
            CountryCode::VI => "VI".to_string(),
            CountryCode::VN => "VN".to_string(),
            CountryCode::VU => "VU".to_string(),
            CountryCode::WF => "WF".to_string(),
            CountryCode::WS => "WS".to_string(),
            CountryCode::YE => "YE".to_string(),
            CountryCode::YT => "YT".to_string(),
            CountryCode::ZA => "ZA".to_string(),
            CountryCode::ZM => "ZM".to_string(),
            CountryCode::ZW => "ZW".to_string(),
        }
    }
}

/// Currency codes based on ISO 4217.
#[derive(Debug)]
pub enum CurrencyCode {
    /// UAE dirham
    AED,
    /// Afghani
    AFN,
    /// Lek
    ALL,
    /// Armenian dram
    AMD,
    /// Netherlands Antillean Guilder
    ANG,
    /// Kwanza
    AOA,
    /// Argentinian peso
    ARS,
    /// Australian dollar
    AUD,
    /// Aruban Guilder
    AWG,
    /// New Manat
    AZN,
    /// Convertible Marks
    BAM,
    /// Barbados dollar
    BBD,
    /// Taka
    BDT,
    /// Lev
    BGN,
    /// Bahraini dinar
    BHD,
    /// Burundi franc
    BIF,
    /// Bermuda dollar
    BMD,
    /// Brunei dollar
    BND,
    /// Boliviano
    BOB,
    /// Real
    BRL,
    /// Bahamian dollar
    BSD,
    /// Ngultrum
    BTN,
    /// Pula
    BWP,
    /// Belarussian rouble
    BYR,
    /// Belize dollar
    BZD,
    /// Canadian dollar
    CAD,
    /// Franc congolais
    CDF,
    /// Swiss franc
    CHF,
    /// Chilean peso
    CLP,
    /// Ren-Min-Bi yuan
    CNY,
    /// Colombian peso
    COP,
    /// Costa Rican colon
    CRC,
    /// Cuban peso
    CUP,
    /// Cape Verde escudo
    CVE,
    /// Czech koruna
    CZK,
    /// Djibouti franc
    DJF,
    /// Danish krone
    DKK,
    /// Dominican peso
    DOP,
    /// Algerian dinar
    DZD,
    /// Estonian kroon
    EEK,
    /// Egyptian pound
    EGP,
    /// Nakfa
    ERN,
    /// Ethiopian birr
    ETB,
    /// Euro
    EUR,
    /// Fiji dollar
    FJD,
    /// Falkland Islands (Malvinas) Pound
    FKP,
    /// Pound sterling
    GBP,
    /// Lari
    GEL,
    /// Ghana Cedi
    GHS,
    /// Gibraltar pound
    GIP,
    /// Dalasi
    GMD,
    /// Guinean franc
    GNF,
    /// Quetzal
    GTQ,
    /// Guinean bissau Peso
    GWP,
    /// Guyana dollar
    GYD,
    /// Hong Kong dollar
    HKD,
    /// Lempira
    HNL,
    /// Kuna
    HRK,
    /// Haitian gourde
    HTG,
    /// Forint
    HUF,
    /// Rupiah
    IDR,
    /// New Israeli sheqel
    ILS,
    /// Indian rupee
    INR,
    /// Iraqi dinar
    IQD,
    /// Iranian rial
    IRR,
    /// Icelandic Krona
    ISK,
    /// Jamaican dollar
    JMD,
    /// Jordanian dinar
    JOD,
    /// Yen
    JPY,
    /// Kenyan shilling
    KES,
    /// Kyrgyz Som
    KGS,
    /// Riel
    KHR,
    /// Comoro franc
    KMF,
    /// Won
    KRW,
    /// Kuwaiti dinar
    KWD,
    /// Cayman Islands dollar
    KYD,
    /// Tenge
    KZT,
    /// Kip
    LAK,
    /// Lebanese pound
    LBP,
    /// Sri Lankan rupee
    LKR,
    /// Liberian dollar
    LRD,
    /// Loti
    LSL,
    /// Lithuanian litus
    LTL,
    /// Latvian lats
    LVL,
    /// Libyan dinar
    LYD,
    /// Moroccan dirham
    MAD,
    /// Moldovan leu
    MDL,
    /// Malagasy Ariary
    MGA,
    /// Denar
    MKD,
    /// Kyat
    MMK,
    /// Tugrik
    MNT,
    /// Pataca
    MOP,
    /// Ouguiya
    MRO,
    /// Ouguiya
    MRU,
    /// Mauritian rupee
    MUR,
    /// Rufiyaa
    MVR,
    /// Kwacha
    MWK,
    /// Mexican peso
    MXN,
    /// Malaysian ringgit
    MYR,
    /// Metical
    MZN,
    /// Namibian dollar
    NAD,
    /// Naira
    NGN,
    /// Cordoba Oro
    NIO,
    /// Norwegian krone
    NOK,
    /// Nepalese rupee
    NPR,
    /// New Zealand dollar
    NZD,
    /// Omani rial
    OMR,
    /// Balboa
    PAB,
    /// Nuevo Sol
    PEN,
    /// Kina
    PGK,
    /// Philippine peso
    PHP,
    /// Pakistani rupee
    PKR,
    /// Zloty
    PLN,
    /// Guarani
    PYG,
    /// Qatari riyal
    QAR,
    /// New Leu
    RON,
    /// Serbian Dinar
    RSD,
    /// Russian rouble
    RUB,
    /// Rwanda franc
    RWF,
    /// Saudi riyal
    SAR,
    /// Solomon Islands Dollar
    SBD,
    /// Seychelles rupee
    SCR,
    /// Sudanese Pound
    SDG,
    /// Swedish Krona
    SEK,
    /// Singapore dollar
    SGD,
    /// Saint Helena Pound
    SHP,
    /// Leone
    SLL,
    /// Somali shilling
    SOS,
    /// Surinam dollar
    SRD,
    /// South Sudanese pound
    SSP,
    /// Dobra
    STD,
    /// El Salvador colon
    SVC,
    /// Syrian pound
    SYP,
    /// Lilangeni
    SZL,
    /// Baht
    THB,
    /// Somoni
    TJS,
    /// Manat
    TMM,
    /// Tunisian dinar
    TND,
    /// Pa'anga
    TOP,
    /// Turkish Lira
    TRY,
    /// Trinidad and Tobago dollar
    TTD,
    /// Taiwan New Dollar
    TWD,
    /// Tanzanian shilling
    TZS,
    /// Hryvnia
    UAH,
    /// Ugandan shilling
    UGX,
    /// United States dollar
    USD,
    /// Uruguayo peso
    UYU,
    /// Uzbekistan sum
    UZS,
    /// Bolivar Fuerte
    VEF,
    /// Dong
    VND,
    /// Vatu
    VUV,
    /// Samoan Tala
    WST,
    /// CFA Franc - BEAC
    XAF,
    /// Eastern Caribbean dollar
    XCD,
    /// CFA franc - BCEAO
    XOF,
    /// Comptoirs Francais du Pacifique Francs
    XPF,
    /// Yemeni rial
    YER,
    /// Rand
    ZAR,
    /// Kwacha
    ZMK,
    /// Kwacha
    ZMW,
    /// Zimbabwean dollar
    ZWD,
}

impl CurrencyCode {
    pub fn to_name(&self) -> String {
        match self {
            CurrencyCode::AED => "UAE dirham".to_string,
            CurrencyCode::AFN => "Afghani".to_string,
            CurrencyCode::ALL => "Lek".to_string,
            CurrencyCode::AMD => "Armenian dram".to_string,
            CurrencyCode::ANG => "Netherlands Antillean Guilder".to_string,
            CurrencyCode::AOA => "Kwanza".to_string,
            CurrencyCode::ARS => "Argentinian peso".to_string,
            CurrencyCode::AUD => "Australian dollar".to_string,
            CurrencyCode::AWG => "Aruban Guilder".to_string,
            CurrencyCode::AZN => "New Manat".to_string,
            CurrencyCode::BAM => "Convertible Marks".to_string,
            CurrencyCode::BBD => "Barbados dollar".to_string,
            CurrencyCode::BDT => "Taka".to_string,
            CurrencyCode::BGN => "Lev".to_string,
            CurrencyCode::BHD => "Bahraini dinar".to_string,
            CurrencyCode::BIF => "Burundi franc".to_string,
            CurrencyCode::BMD => "Bermuda dollar".to_string,
            CurrencyCode::BND => "Brunei dollar".to_string,
            CurrencyCode::BOB => "Boliviano".to_string,
            CurrencyCode::BRL => "Real".to_string,
            CurrencyCode::BSD => "Bahamian dollar".to_string,
            CurrencyCode::BTN => "Ngultrum".to_string,
            CurrencyCode::BWP => "Pula".to_string,
            CurrencyCode::BYR => "Belarussian rouble".to_string,
            CurrencyCode::BZD => "Belize dollar".to_string,
            CurrencyCode::CAD => "Canadian dollar".to_string,
            CurrencyCode::CDF => "Franc congolais".to_string,
            CurrencyCode::CHF => "Swiss franc".to_string,
            CurrencyCode::CLP => "Chilean peso".to_string,
            CurrencyCode::CNY => "Ren-Min-Bi yuan".to_string,
            CurrencyCode::COP => "Colombian peso".to_string,
            CurrencyCode::CRC => "Costa Rican colon".to_string,
            CurrencyCode::CUP => "Cuban peso".to_string,
            CurrencyCode::CVE => "Cape Verde escudo".to_string,
            CurrencyCode::CZK => "Czech koruna".to_string,
            CurrencyCode::DJF => "Djibouti franc".to_string,
            CurrencyCode::DKK => "Danish krone".to_string,
            CurrencyCode::DOP => "Dominican peso".to_string,
            CurrencyCode::DZD => "Algerian dinar".to_string,
            CurrencyCode::EEK => "Estonian kroon".to_string,
            CurrencyCode::EGP => "Egyptian pound".to_string,
            CurrencyCode::ERN => "Nakfa".to_string,
            CurrencyCode::ETB => "Ethiopian birr".to_string,
            CurrencyCode::EUR => "Euro".to_string,
            CurrencyCode::FJD => "Fiji dollar".to_string,
            CurrencyCode::FKP => "Falkland Islands (Malvinas) Pound".to_string,
            CurrencyCode::GBP => "Pound sterling".to_string,
            CurrencyCode::GEL => "Lari".to_string,
            CurrencyCode::GHS => "Ghana Cedi".to_string,
            CurrencyCode::GIP => "Gibraltar pound".to_string,
            CurrencyCode::GMD => "Dalasi".to_string,
            CurrencyCode::GNF => "Guinean franc".to_string,
            CurrencyCode::GTQ => "Quetzal".to_string,
            CurrencyCode::GWP => "Guinean bissau Peso".to_string,
            CurrencyCode::GYD => "Guyana dollar".to_string,
            CurrencyCode::HKD => "Hong Kong dollar".to_string,
            CurrencyCode::HNL => "Lempira".to_string,
            CurrencyCode::HRK => "Kuna".to_string,
            CurrencyCode::HTG => "Haitian gourde".to_string,
            CurrencyCode::HUF => "Forint".to_string,
            CurrencyCode::IDR => "Rupiah".to_string,
            CurrencyCode::ILS => "New Israeli sheqel".to_string,
            CurrencyCode::INR => "Indian rupee".to_string,
            CurrencyCode::IQD => "Iraqi dinar".to_string,
            CurrencyCode::IRR => "Iranian rial".to_string,
            CurrencyCode::ISK => "Icelandic Krona".to_string,
            CurrencyCode::JMD => "Jamaican dollar".to_string,
            CurrencyCode::JOD => "Jordanian dinar".to_string,
            CurrencyCode::JPY => "Yen".to_string,
            CurrencyCode::KES => "Kenyan shilling".to_string,
            CurrencyCode::KGS => "Kyrgyz Som".to_string,
            CurrencyCode::KHR => "Riel".to_string,
            CurrencyCode::KMF => "Comoro franc".to_string,
            CurrencyCode::KRW => "Won".to_string,
            CurrencyCode::KWD => "Kuwaiti dinar".to_string,
            CurrencyCode::KYD => "Cayman Islands dollar".to_string,
            CurrencyCode::KZT => "Tenge".to_string,
            CurrencyCode::LAK => "Kip".to_string,
            CurrencyCode::LBP => "Lebanese pound".to_string,
            CurrencyCode::LKR => "Sri Lankan rupee".to_string,
            CurrencyCode::LRD => "Liberian dollar".to_string,
            CurrencyCode::LSL => "Loti".to_string,
            CurrencyCode::LTL => "Lithuanian litus".to_string,
            CurrencyCode::LVL => "Latvian lats".to_string,
            CurrencyCode::LYD => "Libyan dinar".to_string,
            CurrencyCode::MAD => "Moroccan dirham".to_string,
            CurrencyCode::MDL => "Moldovan leu".to_string,
            CurrencyCode::MGA => "Malagasy Ariary".to_string,
            CurrencyCode::MKD => "Denar".to_string,
            CurrencyCode::MMK => "Kyat".to_string,
            CurrencyCode::MNT => "Tugrik".to_string,
            CurrencyCode::MOP => "Pataca".to_string,
            CurrencyCode::MRO => "Ouguiya".to_string,
            CurrencyCode::MRU => "Ouguiya".to_string,
            CurrencyCode::MUR => "Mauritian rupee".to_string,
            CurrencyCode::MVR => "Rufiyaa".to_string,
            CurrencyCode::MWK => "Kwacha".to_string,
            CurrencyCode::MXN => "Mexican peso".to_string,
            CurrencyCode::MYR => "Malaysian ringgit".to_string,
            CurrencyCode::MZN => "Metical".to_string,
            CurrencyCode::NAD => "Namibian dollar".to_string,
            CurrencyCode::NGN => "Naira".to_string,
            CurrencyCode::NIO => "Cordoba Oro".to_string,
            CurrencyCode::NOK => "Norwegian krone".to_string,
            CurrencyCode::NPR => "Nepalese rupee".to_string,
            CurrencyCode::NZD => "New Zealand dollar".to_string,
            CurrencyCode::OMR => "Omani rial".to_string,
            CurrencyCode::PAB => "Balboa".to_string,
            CurrencyCode::PEN => "Nuevo Sol".to_string,
            CurrencyCode::PGK => "Kina".to_string,
            CurrencyCode::PHP => "Philippine peso".to_string,
            CurrencyCode::PKR => "Pakistani rupee".to_string,
            CurrencyCode::PLN => "Zloty".to_string,
            CurrencyCode::PYG => "Guarani".to_string,
            CurrencyCode::QAR => "Qatari riyal".to_string,
            CurrencyCode::RON => "New Leu".to_string,
            CurrencyCode::RSD => "Serbian Dinar".to_string,
            CurrencyCode::RUB => "Russian rouble".to_string,
            CurrencyCode::RWF => "Rwanda franc".to_string,
            CurrencyCode::SAR => "Saudi riyal".to_string,
            CurrencyCode::SBD => "Solomon Islands Dollar".to_string,
            CurrencyCode::SCR => "Seychelles rupee".to_string,
            CurrencyCode::SDG => "Sudanese Pound".to_string,
            CurrencyCode::SEK => "Swedish Krona".to_string,
            CurrencyCode::SGD => "Singapore dollar".to_string,
            CurrencyCode::SHP => "Saint Helena Pound".to_string,
            CurrencyCode::SLL => "Leone".to_string,
            CurrencyCode::SOS => "Somali shilling".to_string,
            CurrencyCode::SRD => "Surinam dollar".to_string,
            CurrencyCode::SSP => "South Sudanese pound".to_string,
            CurrencyCode::STD => "Dobra".to_string,
            CurrencyCode::SVC => "El Salvador colon".to_string,
            CurrencyCode::SYP => "Syrian pound".to_string,
            CurrencyCode::SZL => "Lilangeni".to_string,
            CurrencyCode::THB => "Baht".to_string,
            CurrencyCode::TJS => "Somoni".to_string,
            CurrencyCode::TMM => "Manat".to_string,
            CurrencyCode::TND => "Tunisian dinar".to_string,
            CurrencyCode::TOP => "Pa'anga".to_string,
            CurrencyCode::TRY => "Turkish Lira".to_string,
            CurrencyCode::TTD => "Trinidad and Tobago dollar".to_string,
            CurrencyCode::TWD => "Taiwan New Dollar".to_string,
            CurrencyCode::TZS => "Tanzanian shilling".to_string,
            CurrencyCode::UAH => "Hryvnia".to_string,
            CurrencyCode::UGX => "Ugandan shilling".to_string,
            CurrencyCode::USD => "United States dollar".to_string,
            CurrencyCode::UYU => "Uruguayo peso".to_string,
            CurrencyCode::UZS => "Uzbekistan sum".to_string,
            CurrencyCode::VEF => "Bolivar Fuerte".to_string,
            CurrencyCode::VND => "Dong".to_string,
            CurrencyCode::VUV => "Vatu".to_string,
            CurrencyCode::WST => "Samoan Tala".to_string,
            CurrencyCode::XAF => "CFA Franc - BEAC".to_string,
            CurrencyCode::XCD => "Eastern Caribbean dollar".to_string,
            CurrencyCode::XOF => "CFA franc - BCEAO".to_string,
            CurrencyCode::XPF => "Comptoirs Francais du Pacifique Francs".to_string,
            CurrencyCode::YER => "Yemeni rial".to_string,
            CurrencyCode::ZAR => "Rand".to_string,
            CurrencyCode::ZMK => "Kwacha".to_string,
            CurrencyCode::ZMW => "Kwacha".to_string,
            CurrencyCode::ZWD => "Zimbabwean dollar".to_string,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CurrencyCode::AED => "AED".to_string(),
            CurrencyCode::AFN => "AFN".to_string(),
            CurrencyCode::ALL => "ALL".to_string(),
            CurrencyCode::AMD => "AMD".to_string(),
            CurrencyCode::ANG => "ANG".to_string(),
            CurrencyCode::AOA => "AOA".to_string(),
            CurrencyCode::ARS => "ARS".to_string(),
            CurrencyCode::AUD => "AUD".to_string(),
            CurrencyCode::AWG => "AWG".to_string(),
            CurrencyCode::AZN => "AZN".to_string(),
            CurrencyCode::BAM => "BAM".to_string(),
            CurrencyCode::BBD => "BBD".to_string(),
            CurrencyCode::BDT => "BDT".to_string(),
            CurrencyCode::BGN => "BGN".to_string(),
            CurrencyCode::BHD => "BHD".to_string(),
            CurrencyCode::BIF => "BIF".to_string(),
            CurrencyCode::BMD => "BMD".to_string(),
            CurrencyCode::BND => "BND".to_string(),
            CurrencyCode::BOB => "BOB".to_string(),
            CurrencyCode::BRL => "BRL".to_string(),
            CurrencyCode::BSD => "BSD".to_string(),
            CurrencyCode::BTN => "BTN".to_string(),
            CurrencyCode::BWP => "BWP".to_string(),
            CurrencyCode::BYR => "BYR".to_string(),
            CurrencyCode::BZD => "BZD".to_string(),
            CurrencyCode::CAD => "CAD".to_string(),
            CurrencyCode::CDF => "CDF".to_string(),
            CurrencyCode::CHF => "CHF".to_string(),
            CurrencyCode::CLP => "CLP".to_string(),
            CurrencyCode::CNY => "CNY".to_string(),
            CurrencyCode::COP => "COP".to_string(),
            CurrencyCode::CRC => "CRC".to_string(),
            CurrencyCode::CUP => "CUP".to_string(),
            CurrencyCode::CVE => "CVE".to_string(),
            CurrencyCode::CZK => "CZK".to_string(),
            CurrencyCode::DJF => "DJF".to_string(),
            CurrencyCode::DKK => "DKK".to_string(),
            CurrencyCode::DOP => "DOP".to_string(),
            CurrencyCode::DZD => "DZD".to_string(),
            CurrencyCode::EEK => "EEK".to_string(),
            CurrencyCode::EGP => "EGP".to_string(),
            CurrencyCode::ERN => "ERN".to_string(),
            CurrencyCode::ETB => "ETB".to_string(),
            CurrencyCode::EUR => "EUR".to_string(),
            CurrencyCode::FJD => "FJD".to_string(),
            CurrencyCode::FKP => "FKP".to_string(),
            CurrencyCode::GBP => "GBP".to_string(),
            CurrencyCode::GEL => "GEL".to_string(),
            CurrencyCode::GHS => "GHS".to_string(),
            CurrencyCode::GIP => "GIP".to_string(),
            CurrencyCode::GMD => "GMD".to_string(),
            CurrencyCode::GNF => "GNF".to_string(),
            CurrencyCode::GTQ => "GTQ".to_string(),
            CurrencyCode::GWP => "GWP".to_string(),
            CurrencyCode::GYD => "GYD".to_string(),
            CurrencyCode::HKD => "HKD".to_string(),
            CurrencyCode::HNL => "HNL".to_string(),
            CurrencyCode::HRK => "HRK".to_string(),
            CurrencyCode::HTG => "HTG".to_string(),
            CurrencyCode::HUF => "HUF".to_string(),
            CurrencyCode::IDR => "IDR".to_string(),
            CurrencyCode::ILS => "ILS".to_string(),
            CurrencyCode::INR => "INR".to_string(),
            CurrencyCode::IQD => "IQD".to_string(),
            CurrencyCode::IRR => "IRR".to_string(),
            CurrencyCode::ISK => "ISK".to_string(),
            CurrencyCode::JMD => "JMD".to_string(),
            CurrencyCode::JOD => "JOD".to_string(),
            CurrencyCode::JPY => "JPY".to_string(),
            CurrencyCode::KES => "KES".to_string(),
            CurrencyCode::KGS => "KGS".to_string(),
            CurrencyCode::KHR => "KHR".to_string(),
            CurrencyCode::KMF => "KMF".to_string(),
            CurrencyCode::KRW => "KRW".to_string(),
            CurrencyCode::KWD => "KWD".to_string(),
            CurrencyCode::KYD => "KYD".to_string(),
            CurrencyCode::KZT => "KZT".to_string(),
            CurrencyCode::LAK => "LAK".to_string(),
            CurrencyCode::LBP => "LBP".to_string(),
            CurrencyCode::LKR => "LKR".to_string(),
            CurrencyCode::LRD => "LRD".to_string(),
            CurrencyCode::LSL => "LSL".to_string(),
            CurrencyCode::LTL => "LTL".to_string(),
            CurrencyCode::LVL => "LVL".to_string(),
            CurrencyCode::LYD => "LYD".to_string(),
            CurrencyCode::MAD => "MAD".to_string(),
            CurrencyCode::MDL => "MDL".to_string(),
            CurrencyCode::MGA => "MGA".to_string(),
            CurrencyCode::MKD => "MKD".to_string(),
            CurrencyCode::MMK => "MMK".to_string(),
            CurrencyCode::MNT => "MNT".to_string(),
            CurrencyCode::MOP => "MOP".to_string(),
            CurrencyCode::MRO => "MRO".to_string(),
            CurrencyCode::MRU => "MRU".to_string(),
            CurrencyCode::MUR => "MUR".to_string(),
            CurrencyCode::MVR => "MVR".to_string(),
            CurrencyCode::MWK => "MWK".to_string(),
            CurrencyCode::MXN => "MXN".to_string(),
            CurrencyCode::MYR => "MYR".to_string(),
            CurrencyCode::MZN => "MZN".to_string(),
            CurrencyCode::NAD => "NAD".to_string(),
            CurrencyCode::NGN => "NGN".to_string(),
            CurrencyCode::NIO => "NIO".to_string(),
            CurrencyCode::NOK => "NOK".to_string(),
            CurrencyCode::NPR => "NPR".to_string(),
            CurrencyCode::NZD => "NZD".to_string(),
            CurrencyCode::OMR => "OMR".to_string(),
            CurrencyCode::PAB => "PAB".to_string(),
            CurrencyCode::PEN => "PEN".to_string(),
            CurrencyCode::PGK => "PGK".to_string(),
            CurrencyCode::PHP => "PHP".to_string(),
            CurrencyCode::PKR => "PKR".to_string(),
            CurrencyCode::PLN => "PLN".to_string(),
            CurrencyCode::PYG => "PYG".to_string(),
            CurrencyCode::QAR => "QAR".to_string(),
            CurrencyCode::RON => "RON".to_string(),
            CurrencyCode::RSD => "RSD".to_string(),
            CurrencyCode::RUB => "RUB".to_string(),
            CurrencyCode::RWF => "RWF".to_string(),
            CurrencyCode::SAR => "SAR".to_string(),
            CurrencyCode::SBD => "SBD".to_string(),
            CurrencyCode::SCR => "SCR".to_string(),
            CurrencyCode::SDG => "SDG".to_string(),
            CurrencyCode::SEK => "SEK".to_string(),
            CurrencyCode::SGD => "SGD".to_string(),
            CurrencyCode::SHP => "SHP".to_string(),
            CurrencyCode::SLL => "SLL".to_string(),
            CurrencyCode::SOS => "SOS".to_string(),
            CurrencyCode::SRD => "SRD".to_string(),
            CurrencyCode::SSP => "SSP".to_string(),
            CurrencyCode::STD => "STD".to_string(),
            CurrencyCode::SVC => "SVC".to_string(),
            CurrencyCode::SYP => "SYP".to_string(),
            CurrencyCode::SZL => "SZL".to_string(),
            CurrencyCode::THB => "THB".to_string(),
            CurrencyCode::TJS => "TJS".to_string(),
            CurrencyCode::TMM => "TMM".to_string(),
            CurrencyCode::TND => "TND".to_string(),
            CurrencyCode::TOP => "TOP".to_string(),
            CurrencyCode::TRY => "TRY".to_string(),
            CurrencyCode::TTD => "TTD".to_string(),
            CurrencyCode::TWD => "TWD".to_string(),
            CurrencyCode::TZS => "TZS".to_string(),
            CurrencyCode::UAH => "UAH".to_string(),
            CurrencyCode::UGX => "UGX".to_string(),
            CurrencyCode::USD => "USD".to_string(),
            CurrencyCode::UYU => "UYU".to_string(),
            CurrencyCode::UZS => "UZS".to_string(),
            CurrencyCode::VEF => "VEF".to_string(),
            CurrencyCode::VND => "VND".to_string(),
            CurrencyCode::VUV => "VUV".to_string(),
            CurrencyCode::WST => "WST".to_string(),
            CurrencyCode::XAF => "XAF".to_string(),
            CurrencyCode::XCD => "XCD".to_string(),
            CurrencyCode::XOF => "XOF".to_string(),
            CurrencyCode::XPF => "XPF".to_string(),
            CurrencyCode::YER => "YER".to_string(),
            CurrencyCode::ZAR => "ZAR".to_string(),
            CurrencyCode::ZMK => "ZMK".to_string(),
            CurrencyCode::ZMW => "ZMW".to_string(),
            CurrencyCode::ZWD => "ZWD".to_string(),
        }
    }
}
