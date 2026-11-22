import Charts
import Library
import SwiftUI

struct ContentView: View {
    @State private var tickers: [String] = []
    @State private var ticker: String?
    @State private var search = ""
    @State private var error: String?

    var body: some View {
        NavigationSplitView {
            List(filtered, id: \.self, selection: $ticker) { Text($0.uppercased()) }
                .searchable(text: $search, placement: .sidebar)
                .navigationSplitViewColumnWidth(min: 120, ideal: 150)
        } detail: {
            if let ticker {
                Dashboard(ticker: ticker).id(ticker)
            } else {
                Text(error ?? "Choose a company").foregroundStyle(.secondary)
            }
        }
        .task {
            do {
                tickers = try await Task.detached { try Library.tickers() }.value
            } catch {
                self.error = describe(error)
            }
        }
    }

    private var filtered: [String] {
        search.isEmpty ? tickers : tickers.filter { $0.localizedCaseInsensitiveContains(search) }
    }
}

/// The panels of equities.json ported so far, for one company.
struct Dashboard: View {
    let ticker: String
    @State private var currencies: [String] = []
    @State private var currency = "USD"

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                Panel(title: "stocks: equity and debt", ticker: ticker, currency: currency, lines: [
                    ("total assets", .reported(item: "TotalAssets", period: "pit")),
                    ("total liabilities", .reported(item: "TotalLiabilities", period: "pit")),
                    ("stockholders' equity", .reported(item: "StockholdersEquity", period: "pit")),
                    ("total debt", .reported(item: "TotalDebt", period: "pit")),
                ], format: .money(currency))
                Panel(title: "stocks: market value and book value (log scale)", ticker: ticker, currency: currency, lines: [
                    ("market cap", .market(field: "market_cap")),
                    ("book value", .reported(item: "StockholdersEquity", period: "pit")),
                ], format: .money(currency), log: true)
                Panel(title: "price-to-book", ticker: ticker, currency: currency, lines: [
                    ("price-to-book", .market(field: "price_to_book")),
                ], format: .ratio)
                Panel(title: "flows: return on equity and earnings yield (last four quarters)", ticker: ticker, currency: currency, lines: [
                    ("return on equity", .market(field: "return_on_equity")),
                    ("earnings yield", .market(field: "earnings_yield")),
                ], format: .percent)
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
        .task(id: currency) {
            do {
                series = try await load(lines, ticker: ticker, currency: currency)
                error = nil
            } catch {
                self.error = describe(error)
            }
        }
    }
}
