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
