use crate::enums::{
    Authenticity, EtpFlag, FinancialStatus, IpoFlag, IssueClassification, IssueSubType,
    LuldRefPriceTier, MarketCategory, Side, ThresholdIndicator,
};
use crate::utils::{as_u16, as_u32, as_u48, as_u64};

#[derive(Debug)]
pub struct PacketHeader<'a> {
    session: &'a [u8],
    pub sequence_number: u64,
    pub message_count: u16,
}

impl<'a> PacketHeader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        PacketHeader {
            session: &bytes[..10],
            sequence_number: as_u64(&bytes[10..18]),
            message_count: as_u16(&bytes[18..20]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExecutedOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub reference: u64,
    pub executed_shares: u32,
    match_number: u64,
}

impl ExecutedOrder {
    fn new(bytes: &[u8]) -> Self {
        ExecutedOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
            executed_shares: as_u32(&bytes[18..22]),
            match_number: as_u64(&bytes[22..30]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExecutedWithPriceOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub reference: u64,
    pub executed_shares: u32,
    match_number: u64,
    printable: bool,
    pub price: u32,
}

impl ExecutedWithPriceOrder {
    fn new(bytes: &[u8]) -> Self {
        ExecutedWithPriceOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
            executed_shares: as_u32(&bytes[18..22]),
            match_number: as_u64(&bytes[22..30]),
            printable: match bytes[30] {
                b'Y' => true,
                b'N' => false,
                _ => unreachable!(),
            },
            price: as_u32(&bytes[31..39]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CancelOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub reference: u64,
    pub canceled_shares: u32,
}

impl CancelOrder {
    fn new(bytes: &[u8]) -> Self {
        CancelOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
            canceled_shares: as_u32(&bytes[18..22]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub reference: u64,
}

impl DeleteOrder {
    fn new(bytes: &[u8]) -> Self {
        DeleteOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ReplaceOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub original_reference: u64,
    pub new_reference: u64,
    pub price: u32,
    pub shares: u32,
}

impl ReplaceOrder {
    fn new(bytes: &[u8]) -> Self {
        ReplaceOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            original_reference: as_u64(&bytes[10..18]),
            new_reference: as_u64(&bytes[18..26]),
            price: as_u32(&bytes[26..30]),
            shares: as_u32(&bytes[30..34]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddOrder {
    pub stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    pub reference: u64,
    pub side: Side,
    pub shares: u32,
    pub stock: String,
    pub price: u32,
    attribution: Option<String>,
}

impl AddOrder {
    fn new(bytes: &[u8]) -> Self {
        AddOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
            side: match bytes[18] as char {
                'B' => Side::Buy,
                'S' => Side::Sell,
                _ => unreachable!(),
            },
            shares: as_u32(&bytes[19..23]),
            stock: String::from_utf8_lossy(&bytes[23..31]).to_string(),
            price: as_u32(&bytes[31..35]),
            attribution: None,
        }
    }

    fn new_with_attribution(bytes: &[u8]) -> Self {
        AddOrder {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            reference: as_u64(&bytes[10..18]),
            side: match bytes[18] as char {
                'B' => Side::Buy,
                'S' => Side::Sell,
                _ => unreachable!(),
            },
            shares: as_u32(&bytes[19..23]),
            stock: String::from_utf8_lossy(&bytes[23..31]).to_string(),
            price: as_u32(&bytes[31..35]),
            attribution: Some(String::from_utf8_lossy(&bytes[35..39]).to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StockDirectory {
    stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
    stock: String,
    market_category: MarketCategory,
    financial_status: FinancialStatus,
    round_lot_size: u32,
    round_lots_only: bool,
    issue_classification: IssueClassification,
    issue_subtype: IssueSubType,
    authenticity: Authenticity,
    short_sale_threshold: ThresholdIndicator,
    ipo_flag: IpoFlag,
    luld_ref_price_tier: LuldRefPriceTier,
    etp_flag: EtpFlag,
    etp_leverage_factor: u32,
    inverse_indicator: bool,
}

impl StockDirectory {
    fn new(bytes: &[u8]) -> Self {
        StockDirectory {
            stock_locate: as_u16(&bytes[..2]),
            tracking_number: as_u16(&bytes[2..4]),
            timestamp: as_u48(&bytes[4..10]),
            stock: String::from_utf8_lossy(&bytes[10..8]).to_string(),
            market_category: MarketCategory::new(bytes[8]),
            financial_status: FinancialStatus::new(bytes[9]),
            round_lot_size: as_u32(&bytes[10..14]),
            round_lots_only: match &bytes[14] {
                b'Y' => true,
                b'N' => false,
                _ => unreachable!(),
            },
            issue_classification: IssueClassification::new(bytes[15]),
            issue_subtype: IssueSubType::new(&bytes[16..18]),
            authenticity: match bytes[18] {
                b'P' => Authenticity::Production,
                b'T' => Authenticity::Test,
                _ => unreachable!(),
            },
            short_sale_threshold: match bytes[19] {
                b'Y' => ThresholdIndicator::Restricted,
                b'N' => ThresholdIndicator::NotRestricted,
                b' ' => ThresholdIndicator::Unavailable,
                _ => unreachable!(),
            },
            ipo_flag: match bytes[20] {
                b'Y' => IpoFlag::New,
                b'N' => IpoFlag::NotNew,
                b' ' => IpoFlag::Unavailable,
                _ => unreachable!(),
            },
            luld_ref_price_tier: match bytes[21] {
                b'1' => LuldRefPriceTier::Tier1,
                b'2' => LuldRefPriceTier::Tier2,
                b' ' => LuldRefPriceTier::Unavailable,
                _ => unreachable!(),
            },
            etp_flag: match bytes[22] {
                b'Y' => EtpFlag::Is,
                b'N' => EtpFlag::IsNot,
                b' ' => EtpFlag::Unavailable,
                _ => unreachable!(),
            },
            etp_leverage_factor: as_u32(&bytes[23..27]),
            inverse_indicator: match bytes[28] {
                b'Y' => true,
                b'N' => false,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Body {
    AddOrder(AddOrder),
    ExecutedWithPriceOrder(ExecutedWithPriceOrder),
    DeleteOrder(DeleteOrder),
    ExecutedOrder(ExecutedOrder),
    StockDirectory(StockDirectory),
    ReplaceOrder(ReplaceOrder),
    CancelOrder(CancelOrder),
    None,
}

#[derive(Debug)]
pub struct Message {
    pub length: u16,
    pub msg_type: u8,
    pub body: Body,
}

impl Message {
    pub fn new(bytes: &[u8]) -> Self {
        Message {
            length: as_u16(&bytes[..2]),
            msg_type: bytes[2],
            body: match bytes[2] {
                b'A' => Body::AddOrder(AddOrder::new(&bytes[3..])),
                b'C' => Body::ExecutedWithPriceOrder(ExecutedWithPriceOrder::new(&bytes[3..])),
                b'D' => Body::DeleteOrder(DeleteOrder::new(&bytes[3..])),
                b'E' => Body::ExecutedOrder(ExecutedOrder::new(&bytes[3..])),
                b'F' => Body::AddOrder(AddOrder::new_with_attribution(&bytes[3..])),
                b'R' => Body::StockDirectory(StockDirectory::new(&bytes[3..])),
                b'U' => Body::ReplaceOrder(ReplaceOrder::new(&bytes[3..])),
                b'X' => Body::CancelOrder(CancelOrder::new(&bytes[3..])),
                _ => Body::None,
            },
        }
    }
}
