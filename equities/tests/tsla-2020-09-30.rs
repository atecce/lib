mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;
use equities::tsla::IncomeStatement;

use equities::item::Item::RestructuringAndOther;

#[test]
fn q3_2020() {

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

    let income_statements = vec![
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2019, 9, 30).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 5_132_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 221_000_000.0,

            energy_generation_and_storage_revenue: 402_000_000.0,
            services_and_other_revenue: 548_000_000.0,

            automotive_sales_cost_of_revenue: 4_014_000_000.0,
            automotive_leasing_cost_of_revenue: 117_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 314_000_000.0,
            services_and_other_cost_of_revenue: 667_000_000.0,

            research_and_development: 334_000_000.0,
            selling_general_and_administrative: 596_000_000.0,
            restructuring_and_other: 0.0,

            interest_income: 15_000_000.0,
            interest_expense: -185_000_000.0,
            other_expense_income_net: 85_000_000.0,

            provision_for_income_taxes: 26_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2019, 9, 30).unwrap(),
            period: equities::Period::NineMonths,

            automotive_sales_revenue: 13_809_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 644_000_000.0,

            energy_generation_and_storage_revenue: 1_095_000_000.0,
            services_and_other_revenue: 1_646_000_000.0,

            automotive_sales_cost_of_revenue: 11_124_000_000.0,
            automotive_leasing_cost_of_revenue: 340_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 956_000_000.0,
            services_and_other_cost_of_revenue: 2_096_000_000.0,

            research_and_development: 998_000_000.0,
            selling_general_and_administrative: 1_947_000_000.0,
            restructuring_and_other: 161_000_000.0,

            interest_income: 34_000_000.0,
            interest_expense: -515_000_000.0,
            other_expense_income_net: 70_000_000.0,

            provision_for_income_taxes: 68_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2020, 9, 30).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 7_346_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 265_000_000.0,

            energy_generation_and_storage_revenue: 579_000_000.0,
            services_and_other_revenue: 581_000_000.0,

            automotive_sales_cost_of_revenue: 5_361_000_000.0,
            automotive_leasing_cost_of_revenue: 145_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 558_000_000.0,
            services_and_other_cost_of_revenue: 644_000_000.0,

            research_and_development: 366_000_000.0,
            selling_general_and_administrative: 888_000_000.0,
            restructuring_and_other: 0.0,

            interest_income: 6_000_000.0,
            interest_expense: -163_000_000.0,
            other_expense_income_net: -97_000_000.0,

            provision_for_income_taxes: 186_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2020, 9, 30).unwrap(),
            period: equities::Period::NineMonths,

            automotive_sales_revenue: 17_150_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 772_000_000.0,

            energy_generation_and_storage_revenue: 1_242_000_000.0,
            services_and_other_revenue: 1_628_000_000.0,

            automotive_sales_cost_of_revenue: 12_774_000_000.0,
            automotive_leasing_cost_of_revenue: 415_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 1_189_000_000.0,
            services_and_other_cost_of_revenue: 1_850_000_000.0,

            research_and_development: 969_000_000.0,
            selling_general_and_administrative: 2_176_000_000.0,
            restructuring_and_other: 0.0,

            interest_income: 24_000_000.0,
            interest_expense: -502_000_000.0,
            other_expense_income_net: -166_000_000.0,

            provision_for_income_taxes: 209_000_000.0,
        },
    ];

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-10q_20200930-gen.pdf"), equities::Ticker::TSLA).unwrap();

    reported::assert(r.process_balance_sheet().unwrap(), balance_sheets.into_iter()
        .map(|sheet| sheet.reported()).flatten()
        .filter(|report| report.val != 0.0)
        .collect());
    reported::assert(r.process_income_statement().unwrap(), income_statements.into_iter()
        .map(|stmt| stmt.reported()).flatten()
        .filter(|report| report.val != 0.0 || report.item == RestructuringAndOther)
        .collect());
}
