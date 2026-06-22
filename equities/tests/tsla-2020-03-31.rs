//   [
//     "Assets",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Current assets",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Total assets",
//     "",
//     "$",
//     "37,250",
//     "",
//     "",
//     "$",
//     "34,309",
//     ""
//   ],
//   [
//     "Liabilities",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Current liabilities",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Commitments and contingencies (Note 12)",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Redeemable noncontrolling interests in subsidiaries",
//     "",
//     "",
//     "632",
//     "",
//     "",
//     "",
//     "643",
//     ""
//   ],
//   [
//     "Convertible senior notes (Note 10)",
//     "",
//     "",
//     "60",
//     "",
//     "",
//     "",
//     "—",
//     ""
//   ],
//   [
//     "Equity",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Stockholders' equity",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     "",
//     ""
//   ],
//   [
//     "Preferred stock; $0.001 par value; 100 shares authorized;\nno shares issued and outstanding",
//     "",
//     "",
//     "—",
//     "",
//     "",
//     "",
//     "—",
//     ""
//   ],
//   [
//     "Common stock; $0.001 par value; 2,000 shares authorized; 185 and\n181 shares issued and outstanding as of March 31, 2020 and December 31,\n2019, respectively",
//     "",
//     "",
//     "0",
//     "",
//     "",
//     "",
//     "0",
//     ""
//   ],
//   [
//     "Additional paid-in capital",
//     "",
//     "",
//     "15,390",
//     "",
//     "",
//     "",
//     "12,737",
//     ""
//   ],
//   [
//     "Accumulated other comprehensive loss",
//     "",
//     "",
//     "(113",
//     ")",
//     "",
//     "",
//     "(36",
//     ")"
//   ],
//   [
//     "Accumulated deficit",
//     "",
//     "",
//     "(6,104",
//     ")",
//     "",
//     "",
//     "(6,083",
//     ")"
//   ],
//   [
//     "Total stockholders' equity",
//     "",
//     "",
//     "9,173",
//     "",
//     "",
//     "",
//     "6,618",
//     ""
//   ],
//   [
//     "Noncontrolling interests in subsidiaries",
//     "",
//     "",
//     "867",
//     "",
//     "",
//     "",
//     "849",
//     ""
//   ],
//   [
//     "Total liabilities and equity",
//     "",
//     "$",
//     "37,250",
//     "",
//     "",
//     "$",
//     "34,309",
//     ""
//   ]
// ]
mod reported;

use equities::Reader;
use equities::tsla::BalanceSheet;
use equities::tsla::IncomeStatement;

#[test]
fn report() {

    let balance_sheets = vec![
// [
//   [
//     "",
//     "",
//     "March 31,",
//     null,
//     "",
//     "",
//     "December 31,",
//     null,
//     ""
//   ],
//   [
//     "",
//     "",
//     "2020",
//     null,
//     "",
//     "",
//     "2019",
//     null,
//     ""
//   ],
        BalanceSheet {
            date: chrono::NaiveDate::from_ymd_opt(2019, 12, 31).unwrap(),

            cash_and_cash_equivalents: 6_268_000_000.0,
            marketable_securities: 0.0,
            accounts_receivable_net: 1_324_000_000.0,
            inventories: 3_552_000_000.0,
            prepaid_expenses_and_other_current_assets: 959_000_000.0,

            operating_lease_vehicles_net: 2_447_000_000.0,
            solar_energy_systems_net: 6_138_000_000.0,

            property_and_equipment_net: 10_396_000_000.0,
            operating_lease_assets: 1_218_000_000.0,
            digital_assets: 0.0,
            intangible_assets_net: 339_000_000.0,
            goodwill: 198_000_000.0,
            deferred_income_tax_assets: 0.0,
            other_assets: 1_470_000_000.0,

            accounts_payable: 3_771_000_000.0,
            accrued_and_other_current_liabilities: 3_222_000_000.0,
            deferred_revenue: 1_163_000_000.0,
            customer_deposits: 726_000_000.0,
            current_portion_of_debt_and_finance_leases: 1_785_000_000.0,

            debt_and_finance_leases_net_of_current_portion: 11_634_000_000.0,
            deferred_revenue_net_of_current_portion: 1_207_000_000.0,
            other_long_term_liabilities: 2_691_000_000.0,
        },
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

    let mut r = equities::pdf::new_reader(std::path::Path::new("tsla/tsla-10q_20200331-gen_0.pdf"), equities::Ticker::TSLA).unwrap();

    let expected_reports = balance_sheets.into_iter()
        .map(|sheet| sheet.reported()).flatten()
        .filter(|report| report.val != 0.0)
        .collect();

    reported::assert(r.process_balance_sheet().unwrap(), expected_reports);
}
