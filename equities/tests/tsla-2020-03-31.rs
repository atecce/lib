mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;
use equities::tsla::IncomeStatement;

use equities::item::Item::RestructuringAndOther;

#[test]
fn report() {

    let balance_sheets = vec![
        equities::tsla::Q4_2019_BALANCE_SHEET,
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2020, 03, 31).unwrap(),

            cash_and_cash_equivalents: 8_080_000_000.0,
            marketable_securities: 0.0,
            accounts_receivable_net: 1_274_000_000.0,
            inventories: 4_494_000_000.0,
            prepaid_expenses_and_other_current_assets: 1_045_000_000.0,

            operating_lease_vehicles_net: 2_527_000_000.0,
            solar_energy_systems_net: 6_106_000_000.0,

            property_and_equipment_net: 10_638_000_000.0,
            operating_lease_assets: 1_197_000_000.0,
            digital_assets: 0.0,
            intangible_assets_net: 323_000_000.0,
            goodwill: 193_000_000.0,
            deferred_income_tax_assets: 0.0,
            other_assets: 1_373_000_000.0,

            accounts_payable: 3_970_000_000.0,
            accrued_and_other_current_liabilities: 2_825_000_000.0,
            deferred_revenue: 1_186_000_000.0,
            customer_deposits: 788_000_000.0,
            current_portion_of_debt_and_finance_leases: 3_217_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 10_666_000_000.0,
            deferred_revenue_net_of_current_portion: 1_199_000_000.0,
            other_long_term_liabilities: 2_667_000_000.0,
        },
    ];

    let income_statements = vec![
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2019, 3, 31).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 3_509_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 215_000_000.0,

            energy_generation_and_storage_revenue: 324_000_000.0,
            services_and_other_revenue: 493_000_000.0,

            automotive_sales_cost_of_revenue: 2_856_000_000.0,
            automotive_leasing_cost_of_revenue: 117_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 316_000_000.0,
            services_and_other_cost_of_revenue: 686_000_000.0,

            research_and_development: 340_000_000.0,
            selling_general_and_administrative: 704_000_000.0,
            restructuring_and_other: 44_000_000.0,

            interest_income: 9_000_000.0,
            interest_expense: -158_000_000.0,
            other_expense_income_net: 26_000_000.0,

            provision_for_income_taxes: 23_000_000.0,
        },
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2020, 3, 31).unwrap(),
            period: equities::Period::ThreeMonths,

            automotive_sales_revenue: 4_893_000_000.0,
            automotive_regulatory_credits: 0.0,
            automotive_leasing_revenue: 239_000_000.0,

            energy_generation_and_storage_revenue: 293_000_000.0,
            services_and_other_revenue: 560_000_000.0,

            automotive_sales_cost_of_revenue: 3_699_000_000.0,
            automotive_leasing_cost_of_revenue: 122_000_000.0,

            energy_generation_and_storage_cost_of_revenue: 282_000_000.0,
            services_and_other_cost_of_revenue: 648_000_000.0,

            research_and_development: 324_000_000.0,
            selling_general_and_administrative: 627_000_000.0,
            restructuring_and_other: 0.0,

            interest_income: 10_000_000.0,
            interest_expense: -169_000_000.0,
            other_expense_income_net: -54_000_000.0,

            provision_for_income_taxes: 2_000_000.0,
        },
    ];

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-10q_20200331-gen_0.pdf"), equities::Ticker::TSLA).unwrap();

    reported::assert(r.process_balance_sheet().unwrap(), balance_sheets.into_iter()
        .map(|sheet| sheet.reported()).flatten()
        .filter(|report| report.val != 0.0)
        .collect());
    reported::assert(r.process_income_statement().unwrap(), income_statements.into_iter()
        .map(|stmt| stmt.reported()).flatten()
        .filter(|report| report.val != 0.0 || report.item == RestructuringAndOther)
        .collect());
}
