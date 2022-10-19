#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Sell,
    Buy,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    AddOrder,
    ExecutedWithPriceOrder,
    DeleteOrder,
    ExecutedOrder,
    ReplaceOrder,
    CancelOrder,
}

#[derive(Debug, PartialEq)]
pub enum MarketCategory {
    NasdaqGlobalSelect,
    NasdaqGlobalMarket,
    NasdaqCapitalMarket,
    Nyse,
    NyseMkt,
    NyseArca,
    BatsZExchange,
    InvestorsExchange,
    Unavailable,
}

impl MarketCategory {
    pub fn new(byte: u8) -> Self {
        match byte {
            b'Q' => MarketCategory::NasdaqGlobalSelect,
            b'G' => MarketCategory::NasdaqCapitalMarket,
            b'S' => MarketCategory::NasdaqGlobalMarket,
            b'N' => MarketCategory::Nyse,
            b'A' => MarketCategory::NyseMkt,
            b'P' => MarketCategory::NyseArca,
            b'Z' => MarketCategory::BatsZExchange,
            b'V' => MarketCategory::InvestorsExchange,
            b' ' => MarketCategory::Unavailable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FinancialStatus {
    Normal,
    Deficient,
    Delinquent,
    Bankrupt,
    Suspended,
    DeficientBankrupt,
    DeficientDelinquent,
    DelinquentBankrupt,
    DeficientDelinquentBankrupt,
    EtpSuspended,
    Unavailable,
}

impl FinancialStatus {
    pub fn new(byte: u8) -> Self {
        match byte {
            b'D' => FinancialStatus::Normal,
            b'E' => FinancialStatus::Deficient,
            b'Q' => FinancialStatus::Delinquent,
            b'S' => FinancialStatus::Bankrupt,
            b'G' => FinancialStatus::Suspended,
            b'H' => FinancialStatus::DeficientBankrupt,
            b'J' => FinancialStatus::DeficientDelinquent,
            b'K' => FinancialStatus::DelinquentBankrupt,
            b'C' => FinancialStatus::DeficientDelinquentBankrupt,
            b'N' => FinancialStatus::EtpSuspended,
            b' ' => FinancialStatus::Unavailable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IssueClassification {
    AmericanDepositaryShare,
    Bond,
    CommonStock,
    DepositoryReceipt,
    A144,
    LimitedPartnership,
    Notes,
    OrdinaryShare,
    PreferredStock,
    OtherSecurities,
    Right,
    SharesOfBeneficialInterest,
    ConvertibleDebenture,
    Unit,
    UnitsPerBenifInt,
    Warrant,
}

impl IssueClassification {
    pub fn new(byte: u8) -> Self {
        match byte {
            b'A' => IssueClassification::AmericanDepositaryShare,
            b'B' => IssueClassification::Bond,
            b'C' => IssueClassification::CommonStock,
            b'F' => IssueClassification::DepositoryReceipt,
            b'I' => IssueClassification::A144,
            b'L' => IssueClassification::LimitedPartnership,
            b'N' => IssueClassification::Notes,
            b'O' => IssueClassification::OrdinaryShare,
            b'P' => IssueClassification::PreferredStock,
            b'Q' => IssueClassification::OtherSecurities,
            b'R' => IssueClassification::Right,
            b'S' => IssueClassification::SharesOfBeneficialInterest,
            b'T' => IssueClassification::ConvertibleDebenture,
            b'U' => IssueClassification::Unit,
            b'V' => IssueClassification::UnitsPerBenifInt,
            b'W' => IssueClassification::Warrant,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IssueSubType {
    PreferredTrustSecurities,
    AlphaIndexETNs,
    IndexBasedDerivative,
    CommonShares,
    CommodityBasedTrustShares,
    CommodityFuturesTrustShares,
    CommodityLinkedSecurities,
    CommodityIndexTrustShares,
    CollateralizedMortgageObligation,
    CurrencyTrustShares,
    CommodityCurrencyLinkedSecurities,
    CurrencyWarrants,
    GlobalDepositaryShares,
    ETFPortfolioDepositaryReceipt,
    EquityGoldShares,
    ETNEquityIndexLinkedSecurities,
    ExchangeTradedManagedFunds,
    ExchangeTradedNotes,
    EquityUnits,
    Holdrs,
    ETNFixedIncomeLinkedSecurities,
    ETNFuturesLinkedSecurities,
    GlobalShares,
    ETFIndexFundShares,
    InterestRate,
    IndexWarrant,
    IndexLinkedExchangeableNotes,
    CorporateBackedTrustSecurity,
    ContingentLitigationRight,
    Llc,
    EquityBasedDerivative,
    ManagedFundShares,
    ETNMultiFactorIndexLinkedSecurities,
    ManagedTrustSecurities,
    NYRegistryShares,
    OpenEndedMutualFund,
    PrivatelyHeldSecurity,
    PoisonPill,
    PartnershipUnits,
    ClosedEndFunds,
    RegS,
    CommodityRedeemableCommodityLinkedSecurities,
    ETNRedeemableFuturesLinkedSecurities,
    REIT,
    CommodityRedeemableCurrencyLinkedSecurities,
    Seed,
    SpotRateClosing,
    SpotRateIntraday,
    TrackingStock,
    TrustCertificates,
    TrustUnits,
    Portal,
    ContingentValueRight,
    TrustIssuedReceipts,
    WorldCurrencyOption,
    Trust,
    Other,
    NotApplicable,
}

impl IssueSubType {
    pub fn new(bytes: &[u8]) -> Self {
        match bytes {
            b"A " => IssueSubType::PreferredTrustSecurities,
            b"AI" => IssueSubType::AlphaIndexETNs,
            b"B " => IssueSubType::IndexBasedDerivative,
            b"C " => IssueSubType::CommonShares,
            b"CB" => IssueSubType::CommodityBasedTrustShares,
            b"CF" => IssueSubType::CommodityFuturesTrustShares,
            b"CL" => IssueSubType::CommodityLinkedSecurities,
            b"CM" => IssueSubType::CommodityIndexTrustShares,
            b"CO" => IssueSubType::CollateralizedMortgageObligation,
            b"CT" => IssueSubType::CurrencyTrustShares,
            b"CU" => IssueSubType::CommodityCurrencyLinkedSecurities,
            b"CW" => IssueSubType::CurrencyWarrants,
            b"D " => IssueSubType::GlobalDepositaryShares,
            b"E " => IssueSubType::ETFPortfolioDepositaryReceipt,
            b"EG" => IssueSubType::EquityGoldShares,
            b"EI" => IssueSubType::ETNEquityIndexLinkedSecurities,
            b"EM" => IssueSubType::ExchangeTradedManagedFunds,
            b"EN" => IssueSubType::ExchangeTradedNotes,
            b"EU" => IssueSubType::EquityUnits,
            b"F " => IssueSubType::Holdrs,
            b"FI" => IssueSubType::ETNFixedIncomeLinkedSecurities,
            b"FL" => IssueSubType::ETNFuturesLinkedSecurities,
            b"G " => IssueSubType::GlobalShares,
            b"I " => IssueSubType::ETFIndexFundShares,
            b"IR" => IssueSubType::InterestRate,
            b"IW" => IssueSubType::IndexWarrant,
            b"IX" => IssueSubType::IndexLinkedExchangeableNotes,
            b"J " => IssueSubType::CorporateBackedTrustSecurity,
            b"L " => IssueSubType::ContingentLitigationRight,
            b"LL" => IssueSubType::Llc,
            b"M " => IssueSubType::EquityBasedDerivative,
            b"MF" => IssueSubType::ManagedFundShares,
            b"ML" => IssueSubType::ETNMultiFactorIndexLinkedSecurities,
            b"MT" => IssueSubType::ManagedTrustSecurities,
            b"N " => IssueSubType::NYRegistryShares,
            b"O " => IssueSubType::OpenEndedMutualFund,
            b"P " => IssueSubType::PrivatelyHeldSecurity,
            b"PP" => IssueSubType::PoisonPill,
            b"PU" => IssueSubType::PartnershipUnits,
            b"Q " => IssueSubType::ClosedEndFunds,
            b"R " => IssueSubType::RegS,
            b"RC" => IssueSubType::CommodityRedeemableCommodityLinkedSecurities,
            b"RF" => IssueSubType::ETNRedeemableFuturesLinkedSecurities,
            b"RT" => IssueSubType::REIT,
            b"RU" => IssueSubType::CommodityRedeemableCurrencyLinkedSecurities,
            b"S " => IssueSubType::Seed,
            b"SC" => IssueSubType::SpotRateClosing,
            b"SI" => IssueSubType::SpotRateIntraday,
            b"T " => IssueSubType::TrackingStock,
            b"TC" => IssueSubType::TrustCertificates,
            b"TU" => IssueSubType::TrustUnits,
            b"U " => IssueSubType::Portal,
            b"V " => IssueSubType::ContingentValueRight,
            b"W " => IssueSubType::TrustIssuedReceipts,
            b"WC" => IssueSubType::WorldCurrencyOption,
            b"X " => IssueSubType::Trust,
            b"Y " => IssueSubType::Other,
            b"Z " => IssueSubType::NotApplicable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Authenticity {
    Production,
    Test,
}

#[derive(Debug, PartialEq)]
pub enum ThresholdIndicator {
    Restricted,
    NotRestricted,
    Unavailable,
}

#[derive(Debug, PartialEq)]
pub enum IpoFlag {
    New,
    NotNew,
    Unavailable,
}

#[derive(Debug, PartialEq)]
pub enum LuldRefPriceTier {
    Tier1,
    Tier2,
    Unavailable,
}

#[derive(Debug, PartialEq)]
pub enum EtpFlag {
    Is,
    IsNot,
    Unavailable,
}

