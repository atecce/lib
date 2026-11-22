import Foundation
import Library

/// A named line on a chart.
struct Series: Identifiable {
    let name: String
    let points: [Point]
    var id: String { name }
}

/// What a chart draws, and how to fetch it.
enum Source {
    /// A reported item: `item` as in `equities::Item`, `period` 3m, 12m or pit.
    case reported(item: String, period: String)
    /// A field of `market_v1`.
    case market(field: String)

    func fetch(ticker: String, currency: String) throws -> [Point] {
        switch self {
        case let .reported(item, period):
            return try Library.reported(ticker: ticker, item: item, period: period, currency: currency)
        case let .market(field):
            return try Library.market(ticker: ticker, field: field, currency: currency)
        }
    }
}

/// The calls into Rust block on the network: run them off the main thread.
func load(_ lines: [(String, Source)], ticker: String, currency: String) async throws -> [Series] {
    try await Task.detached {
        try lines.map { name, source in
            Series(name: name, points: try source.fetch(ticker: ticker, currency: currency))
        }
    }.value
}

/// A message for an error from Rust.
func describe(_ error: Error) -> String {
    switch error {
    case InfluxError.NoToken: return "INFLUXDB3_AUTH_TOKEN is not set: run from a shell that has sourced ~/.profile."
    case let InfluxError.Query(message): return message
    default: return error.localizedDescription
    }
}
