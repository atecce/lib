
use crate::Reader;
use crate::tsla::BalanceSheet;
use crate::tsla::IncomeStatement;

use crate::item::Item::RestructuringAndOther;

#[test]
fn report() {

    let balance_sheets = vec![
        crate::tsla::Q4_2019_BALANCE_SHEET,
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

            stockholders_equity: 9_173_000_000.0,
        },
    ];

    let income_statements = vec![
        IncomeStatement {
            date: chrono::NaiveDate::from_ymd_opt(2019, 3, 31).unwrap(),
            period: crate::Period::ThreeMonths,

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
            period: crate::Period::ThreeMonths,

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

    let mut r = crate::testing::filing_dir(std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tsla/filings/000156459020019931")), crate::Ticker::TSLA).unwrap();

    let balance = r.process_balance_sheet().unwrap();
    // Total debt: the current portion of debt and finance leases plus the
    // rest (3,217 + 10,666 and 1,785 + 11,634, in millions).
    let mut debt: Vec<(String, f64)> = balance.iter()
        .filter(|i| i.item == crate::item::Item::TotalDebt)
        .map(|i| (i.date.to_string(), i.val))
        .collect();
    debt.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(debt, [("2019-12-31".to_string(), 13_419_000_000.0), ("2020-03-31".to_string(), 13_883_000_000.0)]);
    crate::testing::assert_reported(balance, balance_sheets.into_iter()
        .map(|sheet| sheet.reported()).flatten()
        .filter(|report| report.val != 0.0)
        .collect());
    let income = r.process_income_statement().unwrap();
    // "Total automotive revenues", from the statement's automotive section.
    let mut automotive: Vec<(String, f64)> = income.iter()
        .filter(|i| i.item == crate::item::Item::AutomotiveRevenue)
        .map(|i| (i.date.to_string(), i.val))
        .collect();
    automotive.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(automotive, [("2019-03-31".to_string(), 3_724_000_000.0), ("2020-03-31".to_string(), 5_132_000_000.0)]);
    crate::testing::assert_reported(income, income_statements.into_iter()
        .map(|stmt| stmt.reported()).flatten()
        .filter(|report| report.val != 0.0 || report.item == RestructuringAndOther)
        .collect());
}
