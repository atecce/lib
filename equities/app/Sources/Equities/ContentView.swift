import Charts
import Library
import SwiftUI

struct ContentView: View {
    @State private var tickers: [String] = []
    @State private var ticker: String?
    @State private var search = ""
    @State private var error: String?
    @State private var loading = true
    @State private var showSettings = false
    /// Bumped after the settings change, to load the companies again.
    @State private var loads = 0

    init() {
        ServerSettings.load().apply()
    }

    var body: some View {
        NavigationSplitView {
            List(filtered, id: \.self, selection: $ticker) { Text($0.uppercased()) }
                .searchable(text: $search, placement: .sidebar)
                .navigationSplitViewColumnWidth(min: 120, ideal: 150)
                // In the list, so a phone shows it too (its detail pane is
                // hidden until a company is picked).
                .overlay {
                    if loading {
                        ProgressView()
                    } else if let error {
                        VStack(spacing: 12) {
                            Text(error).font(.callout).foregroundStyle(.secondary).multilineTextAlignment(.center)
                            Button("Server settings") { showSettings = true }
                        }
                        .padding()
                    }
                }
                .toolbar {
                    ToolbarItem {
                        Button { showSettings = true } label: { Label("Server", systemImage: "gearshape") }
                    }
                }
        } detail: {
            if let ticker {
                Dashboard(ticker: ticker).id(ticker)
            } else {
                Text("Choose a company").foregroundStyle(.secondary)
            }
        }
        .sheet(isPresented: $showSettings) {
            SettingsView { loads += 1 }
        }
        .task(id: loads) {
            loading = true
            defer { loading = false }
            do {
                tickers = try await Task.detached { try Library.tickers() }.value
                error = nil
                // EQUITIES_TICKER=aapl opens a company at launch (simulator.sh).
                if let t = ProcessInfo.processInfo.environment["EQUITIES_TICKER"]?.lowercased(), tickers.contains(t) {
                    ticker = t
                }
            } catch {
                tickers = []
                self.error = describe(error)
                // Nothing configured, as on a new phone: ask.
                if case InfluxError.NoToken = error { showSettings = true }
            }
        }
    }

    private var filtered: [String] {
        search.isEmpty ? tickers : tickers.filter { $0.localizedCaseInsensitiveContains(search) }
    }
}

/// The two sides of a company's accounts: stocks are levels at a point in
/// time (the balance sheet, market value), flows are what changes them over
/// a period (the income and cash flow statements).
enum Side: String, CaseIterable, Identifiable {
    case stocks, flows
    var id: Self { self }
}

/// The period of a flow: a quarter or a year (`3m`, `12m` in InfluxDB).
enum FlowPeriod: String, CaseIterable, Identifiable {
    case quarter = "3m", year = "12m"
    var id: Self { self }
    var label: String { self == .quarter ? "Quarter" : "Year" }
}

/// One chart's worth: what equities.json shows in a panel.
struct PanelSpec {
    let title: String
    let lines: [(String, Source)]
    var format: Format? = nil   // money in the company's currency unless set
    var log = false
}

/// The panels of equities.json for one company, stocks and flows on two
/// tabs. Each stock panel has a flow panel at the same position, the flows
/// that change it, as in the dashboard's rows.
struct Dashboard: View {
    let ticker: String
    @State private var currencies: [String] = []
    @State private var currency = "USD"
    // Kept across companies, and across launches.
    @AppStorage("side") private var side = Side.stocks
    @AppStorage("period") private var period = FlowPeriod.quarter

    private func pit(_ item: String) -> Source { .reported(item: item, period: "pit") }
    private func flow(_ item: String) -> Source { .reported(item: item, period: period.rawValue) }

    private var stocks: [PanelSpec] {[
        PanelSpec(title: "equity and debt", lines: [
            ("total assets", pit("TotalAssets")),
            ("total liabilities", pit("TotalLiabilities")),
            ("stockholders' equity", pit("StockholdersEquity")),
            ("total debt", pit("TotalDebt")),
        ]),
        PanelSpec(title: "cash and working capital", lines: [
            ("cash", pit("CashAndCashEquivalents")),
            ("marketable securities", pit("MarketableSecurities")),
            ("receivables", pit("AccountsReceivableNet")),
            ("inventories", pit("Inventories")),
        ]),
        PanelSpec(title: "long-lived assets", lines: [
            ("property and equipment", pit("PropertyAndEquipmentNet")),
            ("goodwill", pit("Goodwill")),
            ("intangibles", pit("IntangibleAssetsNet")),
        ]),
        PanelSpec(title: "market value and book value (log scale)", lines: [
            ("market cap", .market(field: "market_cap")),
            ("book value", pit("StockholdersEquity")),
        ], log: true),
        PanelSpec(title: "price-to-book", lines: [
            ("price-to-book", .market(field: "price_to_book")),
        ], format: .ratio),
    ]}

    private var flows: [PanelSpec] {[
        PanelSpec(title: "income and returns to shareholders", lines: [
            ("revenue", flow("Revenue")),
            ("operating income", flow("OperatingIncome")),
            ("net income", flow("NetIncome")),
            ("buybacks", flow("PaymentsRelatedToRepurchasesOfCommonStock")),
            ("dividends", flow("DividendsPaid")),
        ]),
        PanelSpec(title: "cash", lines: [
            ("operating", flow("NetCashProvidedByOperatingActivities")),
            ("investing", flow("NetCashUsedInInvestingActivities")),
            ("financing", flow("NetCashUsedInFinancingActivities")),
        ]),
        PanelSpec(title: "investment", lines: [
            ("capex", flow("PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets")),
            ("depreciation and amortization", flow("DepreciationAndAmoritization")),
            ("acquisitions", flow("AcquisitionsNetOfCashAcquired")),
        ]),
        // Always the last four quarters, whatever the period picked.
        PanelSpec(title: "return on equity and earnings yield (last four quarters)", lines: [
            ("return on equity", .market(field: "return_on_equity")),
            ("earnings yield", .market(field: "earnings_yield")),
        ], format: .percent),
    ]}

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                Picker("Side", selection: $side) {
                    ForEach(Side.allCases) { Text($0.rawValue.capitalized).tag($0) }
                }
                .pickerStyle(.segmented)
                .labelsHidden()
                if side == .flows {
                    Picker("Period", selection: $period) {
                        ForEach(FlowPeriod.allCases) { Text($0.label).tag($0) }
                    }
                    .pickerStyle(.segmented)
                    .labelsHidden()
                }
                ForEach(side == .stocks ? stocks : flows, id: \.title) { spec in
                    Panel(title: "\(side.rawValue): \(spec.title)", ticker: ticker, currency: currency,
                          lines: spec.lines, format: spec.format ?? .money(currency), log: spec.log)
                }
            }
            .padding()
        }
        .navigationTitle(ticker.uppercased())
        .toolbar {
            if currencies.count > 1 {
                Picker("Currency", selection: $currency) {
                    ForEach(currencies, id: \.self) { Text($0) }
                }
            }
        }
        .task {
            let found = (try? await Task.detached { [ticker] in try Library.currencies(ticker: ticker) }.value) ?? []
            currencies = found
            if let first = found.first, !found.contains(currency) { currency = first }
        }
    }
}

enum Format {
    case money(String), ratio, percent

    func label(_ v: Double) -> String {
        switch self {
        case let .money(code): return "\(code) " + v.formatted(.number.notation(.compactName))
        case .ratio: return v.formatted(.number.precision(.fractionLength(0...1)))
        case .percent: return v.formatted(.percent.precision(.fractionLength(0)))
        }
    }
}

struct Panel: View {
    let title: String
    let ticker: String
    let currency: String
    let lines: [(String, Source)]
    let format: Format
    var log = false

    @State private var series: [Series] = []

    /// What the panel draws: fetch again when it changes.
    private var key: String {
        currency + lines.map { "\($0.0)=\($0.1)" }.joined(separator: ",")
    }
    @State private var error: String?

    var body: some View {
        VStack(alignment: .leading) {
            Text("\(ticker.uppercased()) \(title) (\(currency))").font(.headline)
            if let error {
                Text(error).foregroundStyle(.red).font(.caption)
            }
            Chart {
                ForEach(series) { s in
                    ForEach(s.points, id: \.time) { p in
                        // A log scale cannot show zero or negative values.
                        if !log || p.value > 0 {
                            LineMark(x: .value("Date", p.time), y: .value("Value", p.value))
                                .foregroundStyle(by: .value("Series", s.name))
                        }
                    }
                }
            }
            .chartYScale(type: log ? .log : .linear)
            .chartYAxis {
                AxisMarks { value in
                    AxisGridLine()
                    AxisValueLabel { if let v = value.as(Double.self) { Text(format.label(v)) } }
                }
            }
            .frame(height: 260)
        }
        .task(id: key) {
            do {
                series = try await load(lines, ticker: ticker, currency: currency)
                error = nil
            } catch {
                self.error = describe(error)
            }
        }
    }
}
