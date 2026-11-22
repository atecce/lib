import Foundation
import Library

/// A named line on a chart.
struct Series: Identifiable {
    let name: String
    let points: [Point]
    var id: String { name }
}

/// What a chart line draws.
enum Source {
    /// A reported item: `item` as in `equities::Item`, `period` 3m, 12m or pit.
    case reported(item: String, period: String)
    /// A field of `market_v1`.
    case market(field: String)

    /// Its place in a company's `Dashboard` data.
    var key: String {
        switch self {
        case let .reported(item, period): return "\(item)/\(period)"
        case let .market(field): return "\(field)/market"
        }
    }
}

/// A company's dashboard data, by currency, then by `Source.key`.
typealias CompanyData = [String: [String: [Point]]]

/// Everything the given lines need, every period and currency, in one query
/// (Library.dashboard). The call blocks on the network: run it off the main
/// thread.
func fetchCompany(_ ticker: String, sources: [Source], periods: [String]) async throws -> CompanyData {
    var items = Set<String>(), fields = Set<String>()
    for source in sources {
        switch source {
        case let .reported(item, _): items.insert(item)
        case let .market(field): fields.insert(field)
        }
    }
    let (itemList, fieldList) = (items.sorted(), fields.sorted())
    let series = try await Task.detached {
        try Library.dashboard(ticker: ticker, items: itemList, periods: periods, market: fieldList)
    }.value
    var data = CompanyData()
    for s in series {
        data[s.currency, default: [:]]["\(s.item)/\(s.period)"] = s.points
    }
    return data
}

/// A message for an error from Rust.
func describe(_ error: Error) -> String {
    switch error {
    case InfluxError.NoToken: return "No InfluxDB token: set one in Server settings, or run from a shell that has sourced ~/.profile."
    case let InfluxError.Query(message): return message
    default: return error.localizedDescription
    }
}
