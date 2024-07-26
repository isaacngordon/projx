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
            BusinessTypeValue::ARTISTS_PHOTOGRAPHERS_CREATIVE => "ARTISTS_PHOTOGRAPHERS_CREATIVE".to_string(),
            BusinessTypeValue::CONSULTANTS_PROFESSIONALS => "CONSULTANTS_PROFESSIONALS".to_string(),
            BusinessTypeValue::FINANCE_INSURANCE => "FINANCE_INSURANCE".to_string(),
            BusinessTypeValue::HAIR_SPA_AESTHETICS => "HAIR_SPA_AESTHETICS".to_string(),
            BusinessTypeValue::MEDICAL_DENTAL_HEALTH_SERVICE => "MEDICAL_DENTAL_HEALTH_SERVICE".to_string(),
            BusinessTypeValue::NONPROFIT_ASSOCIATIONS_GROUPS => "NONPROFIT_ASSOCIATIONS_GROUPS".to_string(),
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
     AD, // Andorra                                                                                             
     AE, // United Arab Emirates                                                                                
     AF, // Afghanistan                                                                                         
     AG, // Antigua and Barbuda                                                                                 
     AI, // Anguilla                                                                                            
     AL, // Albania                                                                                             
     AM, // Armenia                                                                                             
     AO, // Angola                                                                                              
     AQ, // Antarctica                                                                                          
     AR, // Argentina                                                                                           
     AS, // American Samoa                                                                                      
     AT, // Austria                                                                                             
     AU, // Australia                                                                                           
     AW, // Aruba                                                                                               
     AX, // Åland Islands                                                                                       
     AZ, // Azerbaijan                                                                                          
     BA, // Bosnia and Herzegovina                                                                              
     BB, // Barbados                                                                                            
     BD, // Bangladesh                                                                                          
     BE, // Belgium                                                                                             
     BF, // Burkina Faso                                                                                        
     BG, // Bulgaria                                                                                            
     BH, // Bahrain                                                                                             
     BI, // Burundi                                                                                             
     BJ, // Benin                                                                                               
     BL, // Saint Barthélemy                                                                                    
     BM, // Bermuda                                                                                             
     BN, // Brunei Darussalam                                                                                   
     BO, // Bolivia, Plurinational State of                                                                     
     BQ, // Bonaire, Sint Eustatius and Saba                                                                    
     BR, // Brazil                                                                                              
     BS, // Bahamas                                                                                             
     BT, // Bhutan                                                                                              
     BV, // Bouvet Island                                                                                       
     BW, // Botswana                                                                                            
     BY, // Belarus                                                                                             
     BZ, // Belize                                                                                              
     CA, // Canada                                                                                              
     CC, // Cocos (Keeling) Islands                                                                             
     CD, // Congo, The Democratic Republic of the                                                               
     CF, // Central African Republic                                                                            
     CG, // Congo                                                                                               
     CH, // Switzerland                                                                                         
     CI, // Côte d'Ivoire                                                                                       
     CK, // Cook Islands                                                                                        
     CL, // Chile                                                                                               
     CM, // Cameroon                                                                                            
     CN, // China                                                                                               
     CO, // Colombia                                                                                            
     CR, // Costa Rica                                                                                          
     CU, // Cuba                                                                                                
     CV, // Cape Verde                                                                                          
     CW, // Curaçao                                                                                             
     CX, // Christmas Island                                                                                    
     CY, // Cyprus                                                                                              
     CZ, // Czech Republic                                                                                      
     DE, // Germany                                                                                             
     DJ, // Djibouti                                                                                            
     DK, // Denmark                                                                                             
     DM, // Dominica                                                                                            
     DO, // Dominican Republic                                                                                  
     DZ, // Algeria                                                                                             
     EC, // Ecuador                                                                                             
     EE, // Estonia                                                                                             
     EG, // Egypt                                                                                               
     EH, // Western Sahara                                                                                      
     ER, // Eritrea                                                                                             
     ES, // Spain                                                                                               
     ET, // Ethiopia                                                                                            
     FI, // Finland                                                                                             
     FJ, // Fiji                                                                                                
     FK, // Falkland Islands                                                                                    
     FM, // Micronesia, Federated States of                                                                     
     FO, // Faroe Islands                                                                                       
     FR, // France                                                                                              
     GA, // Gabon                                                                                               
     GB, // United Kingdom                                                                                      
     GD, // Grenada                                                                                             
     GE, // Georgia                                                                                             
     GF, // French Guiana                                                                                       
     GG, // Guernsey                                                                                            
     GH, // Ghana                                                                                               
     GI, // Gibraltar                                                                                           
     GL, // Greenland                                                                                           
     GM, // Gambia                                                                                              
     GN, // Guinea                                                                                              
     GP, // Guadeloupe                                                                                          
     GQ, // Equatorial Guinea                                                                                   
     GR, // Greece                                                                                              
     GS, // South Georgia and the South Sandwich Islands                                                        
     GT, // Guatemala                                                                                           
     GU, // Guam                                                                                                
     GW, // Guinea-Bissau                                                                                       
     GY, // Guyana                                                                                              
     HK, // Hong Kong                                                                                           
     HM, // Heard Island and McDonald Islands                                                                   
     HN, // Honduras                                                                                            
     HR, // Croatia                                                                                             
     HT, // Haiti                                                                                               
     HU, // Hungary                                                                                             
     ID, // Indonesia                                                                                           
     IE, // Ireland                                                                                             
     IL, // Israel                                                                                              
     IM, // Isle of Man                                                                                         
     IN, // India                                                                                               
     IO, // British Indian Ocean Territory                                                                      
     IQ, // Iraq                                                                                                
     IR, // Iran                                                                                                
     IS, // Iceland                                                                                             
     IT, // Italy                                                                                               
     JE, // Jersey                                                                                              
     JM, // Jamaica                                                                                             
     JO, // Jordan                                                                                              
     JP, // Japan                                                                                               
     KE, // Kenya                                                                                               
     KG, // Kyrgyzstan                                                                                          
     KH, // Cambodia                                                                                            
     KI, // Kiribati                                                                                            
     KM, // Comoros                                                                                             
     KN, // Saint Kitts and Nevis                                                                               
     KP, // Korea, Democratic People's Republic of                                                              
     KR, // Korea, Republic of                                                                                  
     KW, // Kuwait                                                                                              
     KY, // Cayman Islands                                                                                      
     KZ, // Kazakhstan                                                                                          
     LA, // Lao People's Democratic Republic                                                                    
     LB, // Lebanon                                                                                             
     LC, // Saint Lucia                                                                                         
     LI, // Liechtenstein                                                                                       
     LK, // Sri Lanka                                                                                           
     LR, // Liberia                                                                                             
     LS, // Lesotho                                                                                             
     LT, // Lithuania                                                                                           
     LU, // Luxembourg                                                                                          
     LV, // Latvia                                                                                              
     LY, // Libya                                                                                               
     MA, // Morocco                                                                                             
     MC, // Monaco                                                                                              
     MD, // Moldova, Republic of                                                                                
     ME, // Montenegro                                                                                          
     MF, // Saint Martin                                                                                        
     MG, // Madagascar                                                                                          
     MH, // Marshall Islands                                                                                    
     MK, // North Macedonia                                                                                     
     ML, // Mali                                                                                                
     MM, // Myanmar                                                                                             
     MN, // Mongolia                                                                                            
     MO, // Macao                                                                                               
     MP, // Northern Mariana Islands                                                                            
     MQ, // Martinique                                                                                          
     MR, // Mauritania                                                                                          
     MS, // Montserrat                                                                                          
     MT, // Malta                                                                                               
     MU, // Mauritius                                                                                           
     MV, // Maldives                                                                                            
     MW, // Malawi                                                                                              
     MX, // Mexico                                                                                              
     MY, // Malaysia                                                                                            
     MZ, // Mozambique                                                                                          
     NA, // Namibia                                                                                             
     NC, // New Caledonia                                                                                       
     NE, // Niger                                                                                               
     NF, // Norfolk Island                                                                                      
     NG, // Nigeria                                                                                             
     NI, // Nicaragua                                                                                           
     NL, // Netherlands                                                                                         
     NO, // Norway                                                                                              
     NP, // Nepal                                                                                               
     NR, // Nauru                                                                                               
     NU, // Niue                                                                                                
     NZ, // New Zealand                                                                                         
     OM, // Oman                                                                                                
     PA, // Panama                                                                                              
     PE, // Peru                                                                                                
     PF, // French Polynesia                                                                                    
     PG, // Papua New Guinea                                                                                    
     PH, // Philippines                                                                                         
     PK, // Pakistan                                                                                            
     PL, // Poland                                                                                              
     PM, // Saint Pierre and Miquelon                                                                           
     PN, // Pitcairn                                                                                            
     PR, // Puerto Rico                                                                                         
     PS, // Palestine                                                                                           
     PT, // Portugal                                                                                            
     PW, // Palau                                                                                               
     PY, // Paraguay                                                                                            
     QA, // Qatar                                                                                               
     RE, // Réunion                                                                                             
     RO, // Romania                                                                                             
     RS, // Serbia                                                                                              
     RU, // Russian Federation                                                                                  
     RW, // Rwanda                                                                                              
     SA, // Saudi Arabia                                                                                        
     SB, // Solomon Islands                                                                                     
     SC, // Seychelles                                                                                          
     SD, // Sudan                                                                                               
     SE, // Sweden                                                                                              
     SG, // Singapore                                                                                           
     SH, // Saint Helena, Ascension and Tristan da Cunha                                                        
     SI, // Slovenia                                                                                            
     SJ, // Svalbard and Jan Mayen                                                                              
     SK, // Slovakia                                                                                            
     SL, // Sierra Leone                                                                                        
     SM, // San Marino                                                                                          
     SN, // Senegal                                                                                             
     SO, // Somalia                                                                                             
     SR, // Suriname                                                                                            
     SS, // South Sudan                                                                                         
     ST, // Sao Tome and Principe                                                                               
     SV, // El Salvador                                                                                         
     SX, // Sint Maarten                                                                                        
     SY, // Syria                                                                                               
     SZ, // Eswatini                                                                                            
     TC, // Turks and Caicos Islands                                                                            
     TD, // Chad                                                                                                
     TF, // French Southern Territories                                                                         
     TG, // Togo                                                                                                
     TH, // Thailand                                                                                            
     TJ, // Tajikistan                                                                                          
     TK, // Tokelau                                                                                             
     TL, // Timor-Leste                                                                                         
     TM, // Turkmenistan                                                                                        
     TN, // Tunisia                                                                                             
     TO, // Tonga                                                                                               
     TR, // Turkey                                                                                              
     TT, // Trinidad and Tobago                                                                                 
     TV, // Tuvalu                                                                                              
     TW, // Taiwan                                                                                              
     TZ, // Tanzania, United Republic of                                                                        
     UA, // Ukraine                                                                                             
     UG, // Uganda                                                                                              
     UM, // United States Minor Outlying Islands                                                                
     US, // United States                                                                                       
     UY, // Uruguay                                                                                             
     UZ, // Uzbekistan                                                                                          
     VA, // Holy See                                                                                            
     VC, // Saint Vincent and the Grenadines                                                                    
     VE, // Venezuela, Bolivarian Republic of                                                                   
     VG, // Virgin Islands (British)                                                                            
     VI, // Virgin Islands (U.S)                                                                                
     VN, // Viet Nam                                                                                            
     VU, // Vanuatu                                                                                             
     WF, // Wallis and Futuna                                                                                   
     WS, // Samoa                                                                                               
     YE, // Yemen                                                                                               
     YT, // Mayotte                                                                                             
     ZA, // South Africa                                                                                        
     ZM, // Zambia                                                                                              
     ZW, // Zimbabwe                                                                                            
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
             CountryCode::NF => "Norfolk Island".to_string