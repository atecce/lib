use std::fmt::Display;
use std::str::FromStr;

macro_rules! items {
    (
        pub enum $item:ident {
            $(
                $variant:ident $(=> [$($alias:literal),*])? $(@ [$($tag:literal),+ $(,)?])?
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, uniffi::Enum)]
        pub enum $item {
            $($variant),*
        }

        impl $item {
            pub const ALL: [$item; 85] = [
                $($item::$variant,)*
            ];

            /// Every XBRL element mapped to an item. us-gaap elements are
            /// written without their prefix; IFRS elements ("ifrs-full_...")
            /// and company extensions with theirs.
            pub const XBRL_TAGS: &'static [(&'static str, $item)] = &[
                $($($(($tag, $item::$variant),)+)?)*
            ];

            /// The item an XBRL element reports, e.g. "us-gaap_Revenues" or
            /// "nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets", as
            /// EDGAR's statement pages write them.
            pub fn from_xbrl_tag(tag: &str) -> Option<$item> {
                let name = tag.strip_prefix("us-gaap_").unwrap_or(tag);
                $($($(if name == $tag { return Some($item::$variant); })+)?)*
                None
            }
        }

        impl FromStr for $item {
            type Err = ItemError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let trimmed = s.trim();
                $(
                    $(
                        $(
                            if trimmed.eq_ignore_ascii_case($alias) {
                                return Ok($item::$variant);
                            }
                        )+
                    )?

                    if trimmed.eq_ignore_ascii_case(stringify!($variant)) {
                        return Ok($item::$variant);
                    }
                )*
                Err(ItemError::ItemNotFound)
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Unit {
    Money,
    Shares,
    PerShare,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

items! {
    pub enum Item {
        // 現金及び現金同等物 (genkin oyobi genkin dōtōbutsu): cash and cash equivalents
        CashAndCashEquivalents => ["Cash and cash equivalents", "Cash and Cash Equivalents", "現金及び現金同等物"] @ ["CashAndCashEquivalentsAtCarryingValue", "ifrs-full_CashAndCashEquivalents", "jpigp_cor:CashAndCashEquivalentsIFRS", "jppfs_cor:CashAndCashEquivalents"],
        // TODO(atec): Short-term investments
        MarketableSecurities => ["Marketable securities", "Short-term investments", "Marketable Securities", "Marketable debt securities", "Short-term marketable securities"] @ ["MarketableSecuritiesCurrent", "AvailableForSaleSecuritiesCurrent", "DebtSecuritiesCurrent", "ShortTermInvestments", "AvailableForSaleSecuritiesDebtSecuritiesCurrent", "DebtSecuritiesAvailableForSaleExcludingAccruedInterestCurrent", "MarketableSecurities"],
        // 営業債権及びその他の債権 (eigyō saiken oyobi sonota no saiken): trade and other receivables
        // 受取手形及び売掛金 (uketori tegata oyobi urikakekin): notes and accounts receivable
        AccountsReceivableNet => ["Accounts receivable, net", "Accounts Receivable, Net", "営業債権及びその他の債権", "受取手形及び売掛金"] @ ["AccountsReceivableNetCurrent", "jpigp_cor:TradeAndOtherReceivablesCAIFRS", "jppfs_cor:NotesAndAccountsReceivableTrade"],
        VendorNonTradeReceivables => ["Vendor non-trade receivables"] @ ["NontradeReceivablesCurrent"],
        // 棚卸資産 (tanaoroshi shisan): inventories
        Inventories => ["Inventories", "Inventory", "棚卸資産"] @ ["InventoryNet", "ifrs-full_Inventories", "jpigp_cor:InventoriesCAIFRS"],
        PrepaidExpensesAndOtherCurrentAssets => ["Prepaid expenses and other current assets", "Prepaid Expenses and Other Current Assets"] @ ["PrepaidExpenseAndOtherAssetsCurrent"],
        OtherCurrentAssets => ["Other current assets"] @ ["OtherAssetsCurrent", "ifrs-full_OtherCurrentAssets"],
        // 流動資産 (ryūdō shisan): current assets
        // 流動資産合計 (ryūdō shisan gōkei): total current assets
        TotalCurrentAssets => ["Total current assets", "Total Current Assets", "流動資産", "流動資産合計"] @ ["AssetsCurrent", "ifrs-full_CurrentAssets", "jpigp_cor:CurrentAssetsIFRS", "jppfs_cor:CurrentAssets"],

        OperatingLeaseVehiclesNet => ["Operating lease vehicles, net", "Operating Lease Vehicles [Member]"] @ ["PropertySubjectToOrAvailableForOperatingLeaseNet", "DeferredCostsLeasingNetNoncurrent"],
        SolarEnergySystemsNet => ["Solar energy systems, net", "Solar Energy Systems [Member]"],

        // 有形固定資産 (yūkei kotei shisan): property, plant and equipment
        PropertyAndEquipmentNet => ["Property and equipment, net", "Property, plant and equipment, net", "Property and Equipment, Net", "有形固定資産"] @ ["PropertyPlantAndEquipmentNet", "PropertyPlantAndEquipmentAndFinanceLeaseRightOfUseAssetAfterAccumulatedDepreciationAndAmortization", "ifrs-full_PropertyPlantAndEquipment", "jpigp_cor:PropertyPlantAndEquipmentIFRS", "jppfs_cor:PropertyPlantAndEquipment"],
        OperatingLeaseAssets => ["Operating lease assets", "Operating lease right-of-use assets", "Operating Lease Assets"] @ ["OperatingLeaseRightOfUseAsset", "ifrs-full_RightofuseAssets"],
        // のれん (noren): goodwill (literally the shop curtain)
        Goodwill => ["Goodwill", "のれん"] @ ["Goodwill", "jpigp_cor:GoodwillIFRS", "jppfs_cor:Goodwill"],
        DigitalAssets => ["Digital assets"] @ ["CryptoAssetFairValueNoncurrent"],
        // 無形資産 (mukei shisan): intangible assets
        IntangibleAssetsNet => ["Intangible assets, net", "Intangible Assets, Net", "無形資産"] @ ["IntangibleAssetsNetExcludingGoodwill", "jpigp_cor:IntangibleAssetsIFRS"],
        // 繰延税金資産 (kurinobe zeikin shisan): deferred tax assets
        DeferredIncomeTaxAssets => ["Deferred income tax assets", "Deferred tax assets", "Deferred Income Tax Assets", "繰延税金資産"] @ ["DeferredIncomeTaxAssetsNet", "DeferredTaxAssetsNetCurrent", "DeferredTaxAssetsLiabilitiesNetCurrent", "DeferredTaxAssetsLiabilitiesNetNoncurrent", "ifrs-full_DeferredTaxAssets", "jpigp_cor:DeferredTaxAssetsIFRS", "jppfs_cor:DeferredTaxAssets"],
        // TODO(atec): Digital assets, net
        NonMarketableEquitySecurities => ["Non-marketable equity securities", "Digital assets, net", "Non-Marketable Equity Securities"],
        OtherAssets => ["Other assets", "Other non-current assets", "Other Assets"] @ ["OtherAssetsNoncurrent", "OtherAssets", "ifrs-full_OtherNoncurrentAssets"],
        // 非流動資産 (hi-ryūdō shisan): non-current assets
        // 固定資産 (kotei shisan): fixed assets
        TotalNonCurrentAssets => ["Total non-current assets", "非流動資産", "固定資産"] @ ["AssetsNoncurrent", "ifrs-full_NoncurrentAssets", "jpigp_cor:NonCurrentAssetsIFRS", "jppfs_cor:NoncurrentAssets"],

        // 資産合計 (shisan gōkei): total assets
        // 総資産 (sō shisan): total assets
        TotalAssets => ["Total assets", "Total Assets", "資産合計", "総資産"] @ ["Assets", "ifrs-full_Assets", "jpigp_cor:AssetsIFRS", "jppfs_cor:Assets"],

        // 営業債務及びその他の債務 (eigyō saimu oyobi sonota no saimu): trade and other payables
        // 支払手形及び買掛金 (shiharai tegata oyobi kaikakekin): notes and accounts payable
        AccountsPayable => ["Accounts payable", "Accounts Payable", "営業債務及びその他の債務", "支払手形及び買掛金"] @ ["AccountsPayableCurrent", "ifrs-full_TradeAndOtherCurrentPayablesToTradeSuppliers", "jpigp_cor:TradeAndOtherPayablesCLIFRS", "jppfs_cor:NotesAndAccountsPayableTrade"],
        AccruedAndOtherCurrentLiabilities => ["Accrued and other current liabilities", "Accrued liabilities and other", "Accrued and Other Current Liabilities"] @ ["AccruedLiabilitiesCurrent"],
        DeferredRevenue => ["Deferred revenue"] @ ["ContractWithCustomerLiabilityCurrent", "DeferredRevenueCurrent"],
        // TODO(atec): Current portion of debt and finance leases
        ShortTermDebt => ["Short-term debt", "Current portion of debt and finance leases", "Short-Term Debt"] @ ["DebtCurrent", "LongTermDebtCurrent"],
        CommercialPaper => ["Commercial paper"] @ ["CommercialPaper"],
        TermDebt => ["Term debt"],
        OtherCurrentLiabilities => ["Other current liabilities"] @ ["OtherLiabilitiesCurrent"],
        // 流動負債 (ryūdō fusai): current liabilities
        // 流動負債合計 (ryūdō fusai gōkei): total current liabilities
        TotalCurrentLiabilities => ["Total current liabilities", "Total Current Liabilities", "流動負債", "流動負債合計"] @ ["LiabilitiesCurrent", "ifrs-full_CurrentLiabilities", "jpigp_cor:TotalCurrentLiabilitiesIFRS", "jppfs_cor:CurrentLiabilities"],
        LongTermDebt => ["Long-term debt", "Long-Term Debt"] @ ["LongTermDebtNoncurrent", "LongTermDebt"],
        DebtAndFinanceLeasesNetOfCurrentPortion => ["Debt and finance leases, net of current portion"],
        DeferredRevenueNetOfCurrentPortion => ["Deferred revenue, net of current portion"] @ ["ContractWithCustomerLiabilityNoncurrent", "DeferredRevenueNoncurrent"],
        LongTermOperatingLeaseLiabilities => ["Long-term operating lease liabilities", "Long-Term Operating Lease Liabilities"] @ ["OperatingLeaseLiabilityNoncurrent"],
        OtherLongTermLiabilities => ["Other long-term liabilities", "Other Long-Term Liabilities"] @ ["OtherLiabilitiesNoncurrent"],
        OtherNonCurrentLiabilities => ["Other non-current liabilities"],
        // 非流動負債 (hi-ryūdō fusai): non-current liabilities
        // 固定負債 (kotei fusai): fixed liabilities
        TotalNonCurrentLiabilities => ["Total non-current liabilities", "非流動負債", "固定負債"] @ ["LiabilitiesNoncurrent", "ifrs-full_NoncurrentLiabilities", "jpigp_cor:NonCurrentLabilitiesIFRS", "jpigp_cor:NonCurrentLiabilitiesIFRS", "jppfs_cor:NoncurrentLiabilities"],

        // 負債合計 (fusai gōkei): total liabilities
        TotalLiabilities => ["Total liabilities", "Total Liabilities", "負債合計"] @ ["Liabilities", "ifrs-full_Liabilities", "jpigp_cor:LiabilitiesIFRS", "jppfs_cor:Liabilities"],
        // Book value: the parent's equity first, total equity where that is all
        // a company reports.
        // 親会社の所有者に帰属する持分 (oyagaisha no shoyūsha ni kizoku suru mochibun): equity attributable to owners of the parent
        // 純資産合計 (jun shisan gōkei): total net assets
        StockholdersEquity => ["親会社の所有者に帰属する持分", "純資産合計"] @ ["StockholdersEquity", "StockholdersEquityIncludingPortionAttributableToNoncontrollingInterest", "ifrs-full_EquityAttributableToOwnersOfParent", "ifrs-full_Equity", "jpigp_cor:EquityAttributableToOwnersOfParentIFRS", "jpigp_cor:EquityIFRS", "jppfs_cor:NetAssets"],

        // 売上高 (uriagedaka): net sales
        // 売上収益 (uriage shūeki): revenue
        // 営業収益 (eigyō shūeki): operating revenue: banks, brokers, utilities
        Revenue => ["Revenue", "Revenues", "Total revenues", "Total Revenues", "Net sales", "Total net sales", "売上高", "売上収益", "営業収益"] @ ["Revenues", "RevenueFromContractWithCustomerExcludingAssessedTax", "SalesRevenueNet", "SalesRevenueGoodsNet", "ifrs-full_Revenue", "ifrs-full_RevenueFromContractsWithCustomers", "jpigp_cor:RevenueIFRS", "jpigp_cor:NetSalesIFRS", "jppfs_cor:NetSales", "jppfs_cor:OperatingRevenue1", "jppfs_cor:OperatingRevenue2", "E02144:TotalNetRevenuesIFRS", "jpcrp_cor:RevenueIFRSSummaryOfBusinessResults", "jpcrp_cor:NetSalesSummaryOfBusinessResults"],
        // 売上原価 (uriage genka): cost of sales
        CostOfRevenue => ["Cost of revenue", "Cost of revenues", "Total cost of revenue", "Total cost of revenues", "Total Cost of Revenue", "Total Cost of Revenues", "Cost of sales", "Total cost of sales", "売上原価"] @ ["CostOfRevenue", "CostOfGoodsAndServicesSold", "CostOfGoodsSold", "ifrs-full_CostOfSales", "jpigp_cor:CostOfSalesIFRS", "jppfs_cor:CostOfSales"],

//        AutomotiveSalesRevenue => ["Automotive sales"],
//        AutomotiveRegulatoryCredits => ["Automotive regulatory credits"],
//        AutomotiveLeasingRevenue => ["Automotive leasing"],
//        EnergyGenerationAndStorageRevenue => ["Energy generation and storage"],
//        ServicesAndOtherRevenue => ["Services and other"],
//
//        AutomotiveSalesCost => ["Automotive sales"],
//        AutomotiveLeasingCost => ["Automotive leasing"],
//        EnergyGenerationAndStorageCost => ["Energy generation and storage"],
//        ServicesAndOtherCost => ["Services and other"],

        // 売上総利益 (uriage sōrieki): gross profit
        GrossProfit => ["Gross profit", "Gross Profit", "Total gross profit", "Total Gross Profit", "売上総利益"] @ ["GrossProfit", "ifrs-full_GrossProfit", "jpigp_cor:GrossProfitIFRS", "jppfs_cor:GrossProfit"],

        ResearchAndDevelopment => ["Research and development", "Research and Development"] @ ["ResearchAndDevelopmentExpense", "ifrs-full_ResearchAndDevelopmentExpense"],
        // 販売費及び一般管理費 (hanbaihi oyobi ippan kanrihi): selling, general and administrative expenses
        SalesGeneralAndAdministrative => ["Sales, general and administrative", "Selling, general and administrative", "Sales, General and Administrative", "販売費及び一般管理費"] @ ["SellingGeneralAndAdministrativeExpense", "jpigp_cor:SellingGeneralAndAdministrativeExpensesIFRS", "jppfs_cor:SellingGeneralAndAdministrativeExpenses"],
        RestructuringAndOther => ["Restructuring and other"],
        TotalOperatingExpenses => ["Total operating expenses", "Total Operating Expenses"] @ ["OperatingExpenses", "ifrs-full_OperatingExpenseExcludingCostOfSales"],
        // 営業利益 (eigyō rieki): operating income
        OperatingIncome => ["Operating income", "Income from operations", "Operating Income", "Income (loss) from operations", "営業利益"] @ ["OperatingIncomeLoss", "ifrs-full_ProfitLossFromOperatingActivities", "jpigp_cor:OperatingProfitLossIFRS", "jppfs_cor:OperatingIncome"],
        // 経常利益 (keijō rieki): ordinary income, operating income plus
        // non-operating gains and losses, before extraordinary items. Japan's
        // headline profit; US GAAP and IFRS have no such line.
        OrdinaryIncome => ["経常利益", "Ordinary income"] @ ["jppfs_cor:OrdinaryIncome"],

        InterestIncome => ["Interest income", "Interest Income"] @ ["InvestmentIncomeInterest", "ifrs-full_RevenueFromInterest"],
        InterestExpense => ["Interest expense", "Interest Expense"] @ ["InterestExpense", "InterestExpenseNonoperating"],
        OtherIncomeNet => ["Other income, net", "Other income (expense), net", "Other (expense) income, net", "Other Income, Net", "Other expense, net"] @ ["OtherNonoperatingIncomeExpense", "OtherNonoperatingIncome"],
        TotalOtherIncomeNet => ["Total other income, net", "Total Other Income, Net"] @ ["NonoperatingIncomeExpense"],
        // 税引前利益 (zeibikimae rieki): profit before tax
        // 税金等調整前当期純利益 (zeikin-tō chōsei-mae tōki jun rieki): income before income taxes
        IncomeBeforeIncomeTax => ["Income before income tax", "Income before income taxes", "Income Before Income Tax", "Income (loss) before income taxes", "税引前利益", "税金等調整前当期純利益"] @ ["IncomeLossFromContinuingOperationsBeforeIncomeTaxesExtraordinaryItemsNoncontrollingInterest", "IncomeLossFromContinuingOperationsBeforeIncomeTaxesMinorityInterestAndIncomeLossFromEquityMethodInvestments", "ifrs-full_ProfitLossBeforeTax", "jpigp_cor:ProfitLossBeforeTaxIFRS", "jppfs_cor:IncomeBeforeIncomeTaxes"],

        // 法人所得税費用 (hōjin shotokuzei hiyō): income tax expense
        // 法人税等合計 (hōjinzei-tō gōkei): total income taxes
        IncomeTaxExpense => ["Income tax expense", "Provision for income taxes", "Provision for (benefit from) income taxes", "Income Tax Expense", "法人所得税費用", "法人税等合計"] @ ["IncomeTaxExpenseBenefit", "ifrs-full_IncomeTaxExpenseContinuingOperations", "jpigp_cor:IncomeTaxExpenseIFRS", "jppfs_cor:IncomeTaxes"],
        // 当期利益 (tōki rieki): profit for the period
        // 当期純利益 (tōki jun rieki): net income for the period
        // 親会社株主に帰属する当期純利益 (oyagaisha kabunushi ni kizoku suru tōki jun rieki): net income attributable to owners of the parent
        NetIncome => ["Net income", "Net income (loss)", "Net (loss) income", "Net loss", "当期利益", "当期純利益", "親会社株主に帰属する当期純利益"] @ ["ProfitLoss", "NetIncomeLoss", "ifrs-full_ProfitLoss", "jpigp_cor:ProfitLossIFRS", "jppfs_cor:ProfitLoss", "jpigp_cor:ProfitLossAttributableToOwnersOfParentIFRS", "jppfs_cor:ProfitLossAttributableToOwnersOfParent"],

        // Per share and share counts, for valuation (market cap, P/E). Read
        // by tag only: their labels ("Basic", "Diluted") say nothing alone.
        // 基本的１株当たり当期利益 (kihonteki hitokabu atari tōki rieki): basic earnings per share
        // １株当たり当期純利益 (hitokabu atari tōki jun rieki): net income per share
        EarningsPerShareBasic => ["基本的１株当たり当期利益", "１株当たり当期純利益"] @ ["EarningsPerShareBasic", "EarningsPerShareBasicAndDiluted", "ifrs-full_BasicEarningsLossPerShare", "jpigp_cor:BasicEarningsLossPerShareIFRS", "jpigp_cor:BasicAndDilutedEarningsLossPerShareIFRS", "jpcrp_cor:BasicEarningsLossPerShareIFRSSummaryOfBusinessResults", "jpcrp_cor:BasicEarningsLossPerShareSummaryOfBusinessResults"],
        // 希薄化後１株当たり当期利益 (kihakuka-go hitokabu atari tōki rieki): diluted earnings per share
        EarningsPerShareDiluted => ["希薄化後１株当たり当期利益"] @ ["EarningsPerShareDiluted", "ifrs-full_DilutedEarningsLossPerShare", "jpigp_cor:DilutedEarningsLossPerShareIFRS", "jpcrp_cor:DilutedEarningsPerShareIFRSSummaryOfBusinessResults", "jpcrp_cor:DilutedEarningsPerShareSummaryOfBusinessResults"],
        WeightedAverageSharesBasic @ ["WeightedAverageNumberOfSharesOutstandingBasic", "WeightedAverageNumberOfShareOutstandingBasicAndDiluted"],
        WeightedAverageSharesDiluted @ ["WeightedAverageNumberOfDilutedSharesOutstanding"],

        StockBasedCompensationExpense => ["Stock-based compensation expense"] @ ["ShareBasedCompensation", "AllocatedShareBasedCompensationExpense", "ifrs-full_AdjustmentsForSharebasedPayments"],
        // 減価償却費及び償却費 (genka shōkyakuhi oyobi shōkyakuhi): depreciation and amortization
        // 減価償却費 (genka shōkyakuhi): depreciation
        DepreciationAndAmoritization => ["Depreciation and amortization", "減価償却費及び償却費", "減価償却費"] @ ["DepreciationDepletionAndAmortization", "DepreciationAmortizationAndAccretionNet", "DepreciationAndAmortization", "tsla_DepreciationAmortizationAndImpairment", "jpigp_cor:DepreciationAndAmortizationOpeCFIFRS", "jppfs_cor:DepreciationAndAmortizationOpeCF"],
        DeferredIncomeTaxes => ["Deferred income taxes"],
        GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet => ["securities, net"],
        Other,
        AccountsReceivable => ["Accounts receivable"],
        ChangeInInventories @ ["IncreaseDecreaseInInventories", "ifrs-full_AdjustmentsForDecreaseIncreaseInInventories"],
        PrepaidExpensesAndOtherAssets => ["Prepaid expenses and other assets"],
        ChangeInAccountsPayable @ ["IncreaseDecreaseInAccountsPayable"],
        ChangeInAccruedAndOtherCurrentLiabilities,
        ChangeInOtherLongTermLiabilities,
        // 営業活動によるキャッシュ・フロー (eigyō katsudō ni yoru kyasshu furō): cash flows from operating activities
        NetCashProvidedByOperatingActivities => ["Net cash provided by operating activities", "営業活動によるキャッシュ・フロー"] @ ["NetCashProvidedByUsedInOperatingActivities", "NetCashProvidedByUsedInOperatingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInOperatingActivities", "jpigp_cor:NetCashProvidedByUsedInOperatingActivitiesIFRS", "jppfs_cor:NetCashProvidedByUsedInOperatingActivities"],
        ProceedsFromMaturitiesOfMarketableSecurities => ["Proceeds from maturities of marketable securities"],
        ProceedsFromSalesOfMarketableSecurities => ["Proceeds from sales of marketable securities"],
        ProceedsFromSalesOfNonMarketableEquitySecurities => ["Proceeds from sales of non-marketable equity securities"],
        PurchasesOfMarketableSecurities => ["Purchases of marketable securities"],
        // 有形固定資産の取得による支出 (yūkei kotei shisan no shutoku ni yoru shishutsu): purchase of property, plant and equipment
        PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets => ["Purchases related to property and equipment and intangible assets", "有形固定資産の取得による支出"] @ ["PaymentsToAcquirePropertyPlantAndEquipment", "PaymentsToAcquireProductiveAssets", "nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets", "ifrs-full_PurchaseOfPropertyPlantAndEquipmentClassifiedAsInvestingActivities", "jpigp_cor:PurchaseOfPropertyPlantAndEquipmentInvCFIFRS", "jppfs_cor:PurchaseOfPropertyPlantAndEquipmentInvCF"],
        PurchasesOfNonMarketableEquitySecurities => ["Purchases of non-marketable equity securities"],
        AcquisitionsNetOfCashAcquired => ["Acquisitions, net of cash acquired"] @ ["PaymentsToAcquireBusinessesNetOfCashAcquired", "PaymentsToAcquireBusinessesAndInterestInAffiliates", "PaymentsToAcquireBusinessesGross"],
        // 投資活動によるキャッシュ・フロー (tōshi katsudō ni yoru kyasshu furō): cash flows from investing activities
        NetCashUsedInInvestingActivities => ["Net cash used in investing activities", "投資活動によるキャッシュ・フロー"] @ ["NetCashProvidedByUsedInInvestingActivities", "NetCashProvidedByUsedInInvestingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInInvestingActivities", "jpigp_cor:NetCashProvidedByUsedInInvestingActivitiesIFRS", "jppfs_cor:NetCashProvidedByUsedInInvestingActivities"],
        // 財務活動によるキャッシュ・フロー (zaimu katsudō ni yoru kyasshu furō): cash flows from financing activities
        NetCashUsedInFinancingActivities => ["Net cash used in financing activities", "財務活動によるキャッシュ・フロー"] @ ["NetCashProvidedByUsedInFinancingActivities", "NetCashProvidedByUsedInFinancingActivitiesContinuingOperations", "ifrs-full_CashFlowsFromUsedInFinancingActivities", "jpigp_cor:NetCashProvidedByUsedInFinancingActivitiesIFRS", "jppfs_cor:NetCashProvidedByUsedInFinancingActivities"],
        ProceedsRelatedToEmployeeStockPlans => ["Proceeds related to employee stock plans"],
        // 自己株式の取得による支出 (jiko kabushiki no shutoku ni yoru shishutsu): purchase of treasury shares
        PaymentsRelatedToRepurchasesOfCommonStock => ["Payments related to repurchases of common stock", "自己株式の取得による支出"] @ ["PaymentsForRepurchaseOfCommonStock", "ifrs-full_PaymentsToAcquireOrRedeemEntitysShares", "jpigp_cor:PurchaseOfTreasurySharesFinCFIFRS", "jppfs_cor:PurchaseOfTreasuryStockFinCF"],
        PaymentsRelatedToEmployeeStockPlanTaxes => ["Payments related to employee stock plan taxes"],
        // 配当金の支払額 (haitōkin no shiharaigaku): dividends paid
        // 親会社の所有者への配当金の支払額 (oyagaisha no shoyūsha e no haitōkin no shiharaigaku): dividends paid to owners of the parent
        DividendsPaid => ["Dividends paid", "配当金の支払額", "親会社の所有者への配当金の支払額"] @ ["PaymentsOfDividends", "PaymentsOfDividendsCommonStock", "ifrs-full_DividendsPaidClassifiedAsFinancingActivities", "jpigp_cor:DividendsPaidToOwnersOfParentFinCFIFRS", "jppfs_cor:CashDividendsPaidFinCF"],
        PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets => ["Principal payments on property and equipment and intangible assets"],
        RepaymentOfDebt => ["Repayment of debt"],
        // 現金及び現金同等物の増減額 (genkin oyobi genkin dōtōbutsu no zōgengaku): net change in cash and cash equivalents
        ChangeInCashAndCashEquivalents => ["Change in cash and cash equivalents", "現金及び現金同等物の増減額"] @ ["CashAndCashEquivalentsPeriodIncreaseDecrease", "CashAndCashEquivalentsPeriodIncreaseDecreaseExcludingExchangeRateEffect", "CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalentsPeriodIncreaseDecreaseIncludingExchangeRateEffect", "CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalentsPeriodIncreaseDecreaseExcludingExchangeRateEffect", "NetCashProvidedByUsedInContinuingOperations", "ifrs-full_IncreaseDecreaseInCashAndCashEquivalents", "jpigp_cor:NetIncreaseDecreaseInCashAndCashEquivalentsIFRS", "jppfs_cor:NetIncreaseDecreaseInCashAndCashEquivalents"],
    }
}

impl Item {
    /// The item an XBRL element reports and the element's rank among that
    /// item's elements: 0 for the first listed. Elements are listed most
    /// inclusive first, so when a statement reports an item twice the total
    /// wins over a component (Berkshire tags "Sales and service revenues" with
    /// the element other companies use for total revenue, and its total with
    /// `Revenues`).
    pub fn from_xbrl_tag_ranked(tag: &str) -> Option<(Item, usize)> {
        let item = Item::from_xbrl_tag(tag)?;
        let name = tag.strip_prefix("us-gaap_").unwrap_or(tag);
        let rank = Item::XBRL_TAGS.iter()
            .filter(|(_, i)| *i == item)
            .position(|(t, _)| *t == name)?;
        Some((item, rank))
    }

    /// What the item's values count: money in the column's currency, shares,
    /// or money per share. Each is scaled differently ("shares in Thousands,
    /// $ in Millions"), and only money adds up across periods.
    pub fn unit(&self) -> Unit {
        use Item::*;
        match self {
            EarningsPerShareBasic | EarningsPerShareDiluted => Unit::PerShare,
            WeightedAverageSharesBasic | WeightedAverageSharesDiluted => Unit::Shares,
            _ => Unit::Money,
        }
    }

    /// Items reported on the cash flow statement. Some share labels or
    /// elements with balance sheet and income statement items ("Inventories",
    /// "Net income"), so each statement only reports its own.
    pub fn is_cash_flow(&self) -> bool {
        use Item::*;
        matches!(self,
            StockBasedCompensationExpense | DepreciationAndAmoritization | DeferredIncomeTaxes
            | GainsOnNonMarketableEquitySecuritiesAndPubliclyHeldSecuritiesNet | Other
            | AccountsReceivable | ChangeInInventories | PrepaidExpensesAndOtherAssets
            | ChangeInAccountsPayable | ChangeInAccruedAndOtherCurrentLiabilities
            | ChangeInOtherLongTermLiabilities | NetCashProvidedByOperatingActivities
            | ProceedsFromMaturitiesOfMarketableSecurities | ProceedsFromSalesOfMarketableSecurities
            | ProceedsFromSalesOfNonMarketableEquitySecurities | PurchasesOfMarketableSecurities
            | PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets
            | PurchasesOfNonMarketableEquitySecurities | AcquisitionsNetOfCashAcquired
            | NetCashUsedInInvestingActivities | NetCashUsedInFinancingActivities
            | ProceedsRelatedToEmployeeStockPlans | PaymentsRelatedToRepurchasesOfCommonStock
            | PaymentsRelatedToEmployeeStockPlanTaxes | DividendsPaid
            | PrincipalPaymentsOnPropertyAndEquipmentAndIntangibleAssets | RepaymentOfDebt
            | ChangeInCashAndCashEquivalents)
    }
}

#[derive(Debug)]
pub enum ItemError {
    ItemNotFound,
}

impl Display for ItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Reported {
    pub ticker: crate::Ticker,
    pub date: chrono::NaiveDate,
    pub p: crate::Period,
    pub item: Item,
    pub currency: crate::Currency,
    pub val: f64,
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn each_xbrl_tag_maps_to_one_item() {
        let mut seen = std::collections::HashMap::new();
        for (tag, item) in Item::XBRL_TAGS {
            if let Some(other) = seen.insert(*tag, *item) {
                panic!("{} is listed under both {:?} and {:?}", tag, other, item);
            }
        }
    }

    #[test]
    fn tags() {
        assert_eq!(Item::from_xbrl_tag("us-gaap_Revenues"), Some(Item::Revenue));
        assert_eq!(Item::from_xbrl_tag("us-gaap_RevenuesAbstract"), None);
        assert_eq!(Item::from_xbrl_tag("nvda_PurchasesOfPropertyAndEquipmentAndIntangibleAssets"),
            Some(Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets));
        assert_eq!(Item::from_xbrl_tag("tsla_Revenues"), None);
    }
}
