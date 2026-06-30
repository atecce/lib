mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;
use equities::tsla::IncomeStatement;

use equities::item::Item::RestructuringAndOther;

#[test]
fn Q3_2020_report() {

    let balance_sheets = vec![
        equities::tsla::Q4_2019_BALANCE_SHEET,
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2020, 09, 30).unwrap(),

            cash_and_cash_equivalents: 14_531_000_000.0,
            marketable_securities: 0.0,
            accounts_receivable_net: 1_757_000_000.0,
            inventories: 4_218_000_000.0,
            prepaid_expenses_and_other_current_assets: 1_238_000_000.0,

            operating_lease_vehicles_net: 2_742_000_000.0,
            solar_energy_systems_net: 6_025_000_000.0,

            property_and_equipment_net: 11_848_000_000.0,
            operating_lease_assets: 1_375_000_000.0,
            digital_assets: 0.0,
            intangible_assets_net: 318_000_000.0,
            goodwill: 203_000_000.0,
            deferred_income_tax_assets: 0.0,
            other_assets: 1_436_000_000.0,

            accounts_payable: 4_958_000_000.0,
            accrued_and_other_current_liabilities: 3_252_000_000.0,
            deferred_revenue: 1_258_000_000.0,
            customer_deposits: 708_000_000.0,
            current_portion_of_debt_and_finance_leases: 3_126_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 10_559_000_000.0,
            deferred_revenue_net_of_current_portion: 1_233_000_000.0,
            other_long_term_liabilities: 3_049_000_000.0,
        },
    ];

//    let income_statements = vec![
//        IncomeStatement {
//            date: chrono::NaiveDate::from_ymd_opt(2019, 6, 30).unwrap(),
//            period: equities::Period::ThreeMonths,
//
//            automotive_sales_revenue: 5_168_000_000.0,
//            automotive_regulatory_credits: 0.0,
//            automotive_leasing_revenue: 208_000_000.0,
//
//            energy_generation_and_storage_revenue: 369_000_000.0,
//            services_and_other_revenue: 605_000_000.0,
//
//            automotive_sales_cost_of_revenue: 4_254_000_000.0,
//            automotive_leasing_cost_of_revenue: 106_000_000.0,
//
//            energy_generation_and_storage_cost_of_revenue: 326_000_000.0,
//            services_and_other_cost_of_revenue: 743_000_000.0,
//
//            research_and_development: 324_000_000.0,
//            selling_general_and_administrative: 647_000_000.0,
//            restructuring_and_other: 117_000_000.0,
//
//            interest_income: 10_000_000.0,
//            interest_expense: -172_000_000.0,
//            other_expense_income_net: -41_000_000.0,
//
//            provision_for_income_taxes: 19_000_000.0,
//        },
//        IncomeStatement {
//            date: chrono::NaiveDate::from_ymd_opt(2019, 6, 30).unwrap(),
//            period: equities::Period::SixMonths,
//
//            automotive_sales_revenue: 8_677_000_000.0,
//            automotive_regulatory_credits: 0.0,
//            automotive_leasing_revenue: 423_000_000.0,
//
//            energy_generation_and_storage_revenue: 693_000_000.0,
//            services_and_other_revenue: 1098_000_000.0,
//
//            automotive_sales_cost_of_revenue: 7_110_000_000.0,
//            automotive_leasing_cost_of_revenue: 223_000_000.0,
//
//            energy_generation_and_storage_cost_of_revenue: 642_000_000.0,
//            services_and_other_cost_of_revenue: 1_429_000_000.0,
//
//            research_and_development: 664_000_000.0,
//            selling_general_and_administrative: 1_351_000_000.0,
//            restructuring_and_other: 161_000_000.0,
//
//            interest_income: 19_000_000.0,
//            interest_expense: -330_000_000.0,
//            other_expense_income_net: -15_000_000.0,
//
//            provision_for_income_taxes: 42_000_000.0,
//        },
//        IncomeStatement {
//            date: chrono::NaiveDate::from_ymd_opt(2020, 6, 30).unwrap(),
//            period: equities::Period::ThreeMonths,
//
//            automotive_sales_revenue: 4_911_000_000.0,
//            automotive_regulatory_credits: 0.0,
//            automotive_leasing_revenue: 268_000_000.0,
//
//            energy_generation_and_storage_revenue: 370_000_000.0,
//            services_and_other_revenue: 487_000_000.0,
//
//            automotive_sales_cost_of_revenue: 3_714_000_000.0,
//            automotive_leasing_cost_of_revenue: 148_000_000.0,
//
//            energy_generation_and_storage_cost_of_revenue: 349_000_000.0,
//            services_and_other_cost_of_revenue: 558_000_000.0,
//
//            research_and_development: 279_000_000.0,
//            selling_general_and_administrative: 661_000_000.0,
//            restructuring_and_other: 0.0,
//
//            interest_income: 8_000_000.0,
//            interest_expense: -170_000_000.0,
//            other_expense_income_net: -15_000_000.0,
//
//            provision_for_income_taxes: 21_000_000.0,
//        },
//        IncomeStatement {
//            date: chrono::NaiveDate::from_ymd_opt(2020, 6, 30).unwrap(),
//            period: equities::Period::SixMonths,
//
//            automotive_sales_revenue: 9_804_000_000.0,
//            automotive_regulatory_credits: 0.0,
//            automotive_leasing_revenue: 507_000_000.0,
//
//            energy_generation_and_storage_revenue: 663_000_000.0,
//            services_and_other_revenue: 1_047_000_000.0,
//
//            automotive_sales_cost_of_revenue: 7_413_000_000.0,
//            automotive_leasing_cost_of_revenue: 270_000_000.0,
//
//            energy_generation_and_storage_cost_of_revenue: 631_000_000.0,
//            services_and_other_cost_of_revenue: 1_206_000_000.0,
//
//            research_and_development: 603_000_000.0,
//            selling_general_and_administrative: 1_288_000_000.0,
//            restructuring_and_other: 0.0,
//
//            interest_income: 18_000_000.0,
//            interest_expense: -339_000_000.0,
//            other_expense_income_net: -69_000_000.0,
//
//            provision_for_income_taxes: 23_000_000.0,
//        },
//    ];

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-10q_20200930-gen.pdf"), equities::Ticker::TSLA).unwrap();

    reported::assert(r.process_balance_sheet().unwrap(), balance_sheets.into_iter()
        .map(|sheet| sheet.reported()).flatten()
        .filter(|report| report.val != 0.0)
        .collect());
//    reported::assert(r.process_income_statement().unwrap(), income_statements.into_iter()
//        .map(|stmt| stmt.reported()).flatten()
//        .filter(|report| report.val != 0.0 || report.item == RestructuringAndOther)
//        .collect());
}
