use equities::{BalanceSheet, CashFlowStatement, IncomeStatement, ReportedItem};

#[test]
fn report() {

    let balance_sheets = vec![
        BalanceSheet {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2025, 1, 26).unwrap(),

            cash_and_cash_equivalents: 8_589_000_000.0,
            marketable_securities: 34_621_000_000.0,
            accounts_receivable_net: 23_065_000_000.0,
            inventories: 10_080_000_000.0,
            prepaid_expenses_and_other_current_assets: 3_771_000_000.0,

            property_and_equipment_net: 6_283_000_000.0,
            operating_lease_assets: 1_793_000_000.0,
            goodwill: 5_188_000_000.0,
            intangible_assets_net: 807_000_000.0,
            deferred_income_tax_assets: 10_979_000_000.0,
            other_assets: 6_425_000_000.0,

            accounts_payable: 6_310_000_000.0,
            accrued_and_other_current_liabilities: 11_737_000_000.0,
            short_term_debt: 0.0,

            long_term_debt: 8_463_000_000.0,
            long_term_operating_lease_liabilities: 1_519_000_000.0,
            other_long_term_liabilities: 4_245_000_000.0,
        },
        BalanceSheet {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),

            cash_and_cash_equivalents: 11_486_000_000.0,
            marketable_securities: 49_122_000_000.0,
            accounts_receivable_net: 33_391_000_000.0,
            inventories: 19_784_000_000.0,
            prepaid_expenses_and_other_current_assets: 2_709_000_000.0,

            property_and_equipment_net: 9_780_000_000.0,
            operating_lease_assets: 2_281_000_000.0,
            goodwill: 6_261_000_000.0,
            intangible_assets_net: 936_000_000.0,
            deferred_income_tax_assets: 13_674_000_000.0,
            other_assets: 11_724_000_000.0,

            accounts_payable: 8_624_000_000.0,
            accrued_and_other_current_liabilities: 16_452_000_000.0,
            short_term_debt: 999_000_000.0,

            long_term_debt: 7_468_000_000.0,
            long_term_operating_lease_liabilities: 2_014_000_000.0,
            other_long_term_liabilities: 6_694_000_000.0,
        },
    ];

    let income_statements = vec![
        IncomeStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2024, 10, 27).unwrap(),
            p: equities::Period::ThreeMonths,

            revenue: 35_082_000_000.0,
            cost_of_revenue: 8_926_000_000.0,

            research_and_development: 3_390_000_000.0,
            sales_general_and_administrative: 897_000_000.0,

            interest_income: 472_000_000.0,
            interest_expense: -61_000_000.0,
            other_income_net: 36_000_000.0,

            income_tax_expense: 3_007_000_000.0,
        },
        IncomeStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2024, 10, 27).unwrap(),
            p: equities::Period::NineMonths,

            revenue: 91_166_000_000.0,
            cost_of_revenue: 22_031_000_000.0,

            research_and_development: 9_200_000_000.0,
            sales_general_and_administrative: 2_516_000_000.0,

            interest_income: 1_275_000_000.0,
            interest_expense: -186_000_000.0,
            other_income_net: 301_000_000.0,

            income_tax_expense: 8_020_000_000.0,
        },

        IncomeStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),
            p: equities::Period::ThreeMonths,

            revenue: 57_006_000_000.0,
            cost_of_revenue: 15_157_000_000.0,

            research_and_development: 4_705_000_000.0,
            sales_general_and_administrative: 1_134_000_000.0,

            interest_income: 624_000_000.0,
            interest_expense: -61_000_000.0,
            other_income_net: 1_363_000_000.0,

            income_tax_expense: 6_026_000_000.0,
        },
        IncomeStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),
            p: equities::Period::NineMonths,

            revenue: 147_811_000_000.0,
            cost_of_revenue: 45_441_000_000.0,

            research_and_development: 12_985_000_000.0,
            sales_general_and_administrative: 3_297_000_000.0,

            interest_income: 1_732_000_000.0,
            interest_expense: -186_000_000.0,
            other_income_net: 3_418_000_000.0,

            income_tax_expense: 13_945_000_000.0,
        },
    ];

    let cash_flow_statements = vec![
        CashFlowStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2024, 10, 27).unwrap(),
            p: equities::Period::NineMonths,

            net_income: income_statements[1].net_income(),

            stock_based_compensation_expense: 3_416_000_000.0,
            depreciation_and_amoritization: 1_321_000_000.0,
            deferred_income_taxes: -3_879_000_000.0,
            gains_on_non_marketable_equity_securities_and_publicly_held_equity_securities_net: -302_000_000.0,
            other: -365_000_000.0,

            accounts_receivable: -7_694_000_000.0,
            inventories: -2_357_000_000.0,
            prepaid_expenses_and_other_assets: -726_000_000.0,
            accounts_payable: 2_490_000_000.0,
            accrued_and_other_current_liabilities: 3_918_000_000.0,
            other_long_term_liabilities: 849_000_000.0,

            proceeds_from_maturities_of_marketable_securities: 9_485_000_000.0,
            proceeds_from_sales_of_marketable_securities: 318_000_000.0,
            proceeds_from_sales_of_non_marketable_equity_securities: 171_000_000.0,
            purchases_of_marketable_securities: -19_565_000_000.0,
            purchases_related_to_property_and_equipment_and_intangible_assets: -2_159_000_000.0,
            purchases_of_non_marketable_equity_securities: -1_008_000_000.0,
            acquisitions_net_of_cash_acquired: -465_000_000.0,

            proceeds_related_to_employee_stock_plans: 489_000_000.0,
            payments_related_to_repurchases_of_common_stock: -25_895_000_000.0,
            payments_related_to_employee_stock_plan_taxes: -5_068_000_000.0,
            dividends_paid: -589_000_000.0,
            principal_payments_on_property_and_equipment_and_intangible_assets: -97_000_000.0,
            repayment_of_debt: -1_250_000_000.0,

            cash_and_cash_equivalents_at_beginning_of_period: 7_280_000_000.0,

            cash_paid_for_income_taxes_net: 10_989_000_000.0,
        },
        CashFlowStatement {
            ticker: equities::Ticker::NVDA,
            date: chrono::NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),
            p: equities::Period::NineMonths,

            net_income: income_statements[3].net_income(),

            stock_based_compensation_expense: 4_753_000_000.0,
            depreciation_and_amoritization: 2_031_000_000.0,
            deferred_income_taxes: -2_035_000_000.0,
            gains_on_non_marketable_equity_securities_and_publicly_held_equity_securities_net: -3_426_000_000.0,
            other: -276_000_000.0,

            accounts_receivable: -10_325_000_000.0,
            inventories: -9_703_000_000.0,
            prepaid_expenses_and_other_assets: 857_000_000.0,
            accounts_payable: 2_032_000_000.0,
            accrued_and_other_current_liabilities: 4_204_000_000.0,
            other_long_term_liabilities: 1_311_000_000.0,

            proceeds_from_maturities_of_marketable_securities: 8_980_000_000.0,
            proceeds_from_sales_of_marketable_securities: 487_000_000.0,
            proceeds_from_sales_of_non_marketable_equity_securities: 72_000_000.0,
            purchases_of_marketable_securities: -20_076_000_000.0,
            purchases_related_to_property_and_equipment_and_intangible_assets: -4_758_000_000.0,
            purchases_of_non_marketable_equity_securities: -4_702_000_000.0,
            acquisitions_net_of_cash_acquired: -1_370_000_000.0,

            proceeds_related_to_employee_stock_plans: 643_000_000.0,
            payments_related_to_repurchases_of_common_stock: -36_271_000_000.0,
            payments_related_to_employee_stock_plan_taxes: -5_809_000_000.0,
            dividends_paid: -732_000_000.0,
            principal_payments_on_property_and_equipment_and_intangible_assets: -97_000_000.0,
            repayment_of_debt: 0.0,

            cash_and_cash_equivalents_at_beginning_of_period: balance_sheets[0].cash_and_cash_equivalents,

            cash_paid_for_income_taxes_net: 13_309_000_000.0,
        },
    ];


    let mut r = equities::reader::new_reader(std::path::Path::new("nvda/2025-11-19.xlsx"), equities::Ticker::NVDA).unwrap();

    assert_reported_items(r.process_balance_sheet().unwrap(), balance_sheets.into_iter().map(|sheet| sheet.reported_items()).flatten().collect());
    assert_reported_items(r.process_income_statement().unwrap(), income_statements.into_iter().map(|stmt| stmt.reported_items()).flatten().collect());
    assert_reported_items(r.process_cash_flow_statement().unwrap(), cash_flow_statements.into_iter().map(|stmt| stmt.reported_items()).flatten().collect());
}

fn assert_reported_items(mut actual: Vec<ReportedItem>, expected: Vec<ReportedItem>) {
    actual.sort_by_cached_key(|item| (item.date, item.p, item.item));

    let zipped = actual.into_iter().zip(expected);
    println!("{:#?}", zipped);

    for (actual_reported_item, expected_reported_item) in zipped {
        assert!(
            actual_reported_item == expected_reported_item,
            "\nactual:\n{:#?}\n\nexpected:\n{:#?}\n",
            actual_reported_item, expected_reported_item,
        )
    }
}
