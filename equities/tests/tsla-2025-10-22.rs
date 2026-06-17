mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;
use equities::tsla::IncomeStatement;

#[test]
fn report() {

    let balance_sheets = vec![
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),

            cash_and_cash_equivalents: 16_139_000_000.0,
            marketable_securities: 20_424_000_000.0,
            accounts_receivable_net: 4_418_000_000.0,
            inventories: 12_017_000_000.0,
            prepaid_expenses_and_other_current_assets: 5_362_000_000.0,

            operating_lease_vehicles_net: 5_581_000_000.0,
            solar_energy_systems_net: 4_924_000_000.0,

            property_and_equipment_net: 35_836_000_000.0,
            operating_lease_assets: 5_160_000_000.0,
            digital_assets: 1_076_000_000.0,
            intangible_assets_net: 150_000_000.0,
            goodwill: 244_000_000.0,
            deferred_income_tax_assets: 6_524_000_000.0,
            other_assets: 4_215_000_000.0,

            accounts_payable: 12_474_000_000.0,
            accrued_and_other_current_liabilities: 10_723_000_000.0,
            deferred_revenue: 3_168_000_000.0,
            current_portion_of_debt_and_finance_leases: 2_456_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 5_757_000_000.0,
            deferred_revenue_net_of_current_portion: 3_317_000_000.0,
            other_long_term_liabilities: 10_495_000_000.0,
        },
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2025, 9, 30).unwrap(),

            cash_and_cash_equivalents: 18_289_000_000.0,
            marketable_securities: 23_358_000_000.0,
            accounts_receivable_net: 4_703_000_000.0,
            inventories: 12_276_000_000.0,
            prepaid_expenses_and_other_current_assets: 6_027_000_000.0,

            operating_lease_vehicles_net: 5_019_000_000.0,
            solar_energy_systems_net: 4_673_000_000.0,

            property_and_equipment_net: 39_407_000_000.0,
            operating_lease_assets: 5_783_000_000.0,
            digital_assets: 1_315_000_000.0,
            intangible_assets_net: 131_000_000.0,
            goodwill: 257_000_000.0,
            deferred_income_tax_assets: 6_637_000_000.0,
            other_assets: 5_860_000_000.0,

            accounts_payable: 12_819_000_000.0,
            accrued_and_other_current_liabilities: 12_791_000_000.0,
            deferred_revenue: 3_756_000_000.0,
            current_portion_of_debt_and_finance_leases: 1_924_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 5_778_000_000.0,
            deferred_revenue_net_of_current_portion: 3_746_000_000.0,
            other_long_term_liabilities: 12_205_000_000.0,
        },
    ];

    let income_statements = vec![
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 18_831_000_000.0,
            automotive_regulatory_credits: 739_000_000.0,
            automotive_leasing_revenue: 446_000_000.0,

            energy_generation_and_storage_revenue: 2_376_000_000.0,
            services_and_other_revenue: 2_790_000_000.0,

            automotive_sales_cost_of_revenue: 15_743_000_000.0,
            automotive_leasing_cost_of_revenue: 247_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 1_651_000_000.0,
            services_and_other_cost_of_revenue: 2_544_000_000.0,

            research_and_development: 1_039_000_000.0,
            selling_general_and_administrative: 1_186_000_000.0,
            restructuring_and_other: 55_000_000.0,

            interest_income: 429_000_000.0,
            interest_expense: -92_000_000.0,
            other_expense_income_net: -263_000_000.0,

            provision_for_income_taxes: 602_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(),
            period: equities::Period::NineMonths,

            automotive_sales_revenue: 53_821_000_000.0,
            automotive_regulatory_credits: 2_071_000_000.0,
            automotive_leasing_revenue: 1_380_000_000.0,

            energy_generation_and_storage_revenue: 7_025_000_000.0,
            services_and_other_revenue: 7_686_000_000.0,

            automotive_sales_cost_of_revenue: 45_602_000_000.0,
            automotive_leasing_cost_of_revenue: 761_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 5_157_000_000.0,
            services_and_other_cost_of_revenue: 7_192_000_000.0,

            research_and_development: 3_264_000_000.0,
            selling_general_and_administrative: 3_837_000_000.0,
            restructuring_and_other: 677_000_000.0,

            interest_income: 1_127_000_000.0,
            interest_expense: -254_000_000.0,
            other_expense_income_net: 100_000_000.0,

            provision_for_income_taxes: 1_456_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2025, 9, 30).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 20_359_000_000.0,
            automotive_regulatory_credits: 417_000_000.0,
            automotive_leasing_revenue: 429_000_000.0,

            energy_generation_and_storage_revenue: 3_415_000_000.0,
            services_and_other_revenue: 3_475_000_000.0,

            automotive_sales_cost_of_revenue: 17_365_000_000.0,
            automotive_leasing_cost_of_revenue: 225_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 2_342_000_000.0,
            services_and_other_cost_of_revenue: 3_109_000_000.0,

            research_and_development: 1_630_000_000.0,
            selling_general_and_administrative: 1_562_000_000.0,
            restructuring_and_other: 238_000_000.0,

            interest_income: 439_000_000.0,
            interest_expense: -76_000_000.0,
            other_expense_income_net: -28_000_000.0,

            provision_for_income_taxes: 570_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2025, 9, 30).unwrap(),
            period: equities::Period::NineMonths,

            automotive_sales_revenue: 49_071_000_000.0,
            automotive_regulatory_credits: 1_451_000_000.0,
            automotive_leasing_revenue: 1_311_000_000.0,

            energy_generation_and_storage_revenue: 8_934_000_000.0,
            services_and_other_revenue: 9_159_000_000.0,

            automotive_sales_cost_of_revenue: 42_393_000_000.0,
            automotive_leasing_cost_of_revenue: 692_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 6_230_000_000.0,
            services_and_other_cost_of_revenue: 8_526_000_000.0,

            research_and_development: 4_628_000_000.0,
            selling_general_and_administrative: 4_179_000_000.0,
            restructuring_and_other: 332_000_000.0,

            interest_income: 1_231_000_000.0,
            interest_expense: -253_000_000.0,
            other_expense_income_net: 173_000_000.0,

            provision_for_income_taxes: 1_098_000_000.0,
        },
    ];

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-20250930-gen.pdf"), equities::Ticker::TSLA).unwrap();
    reported::assert(r.process_balance_sheet().unwrap(), balance_sheets.into_iter().map(|sheet| sheet.reported()).flatten().collect());
    reported::assert(r.process_income_statement().unwrap(), income_statements.into_iter().map(|stmt| stmt.reported()).flatten().collect());
}
