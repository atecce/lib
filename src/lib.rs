uniffi::setup_scaffolding!();

bible::uniffi_reexport_scaffolding!();
greece::uniffi_reexport_scaffolding!();

pub mod persia;
pub mod rome;

// import SwiftUI
// import Charts
// 
// @available(iOS 16.0, macOS 13.0, *)
// public struct FinancialChartView: View {
//     public let data: [ReportedItem]
//     public let title: String
//     
//     private let dateFormatter: DateFormatter = {
//         let df = DateFormatter()
//         df.dateFormat = "yyyy-MM-dd"
//         return df
//     }()
//     
//     public init(data: [ReportedItem], title: String = "NVIDIA Financials") {
//         self.data = data
//         self.title = title
//     }
//     
//     public var body: some View {
//         VStack(alignment: .leading) {
//             Text(title)
//                 .font(.title2)
//                 .bold()
//             
//             Chart {
//                 ForEach(data.filter { $0.p == .threeMonths }) { item in
//                     if let date = dateFormatter.date(from: item.t) {
//                         LineMark(
//                             x: .value("Date", date),
//                             y: .value("Value", item.val)
//                         )
//                         .foregroundStyle(by: .value("Metric", String(describing: item.item)))
//                         .interpolationMethod(.catmullRom)
//                         
//                         PointMark(
//                             x: .value("Date", date),
//                             y: .value("Value", item.val)
//                         )
//                         .foregroundStyle(by: .value("Metric", String(describing: item.item)))
//                     }
//                 }
//             }
//             .frame(minHeight: 300)
//             .chartYAxis {
//                 AxisMarks(format: .currency(code: "USD").notation(.compactName))
//             }
//         }
//         .padding()
//     }
// }
// 
// extension ReportedItem: Identifiable {
//     public var id: String {
//         return "\(t)-\(item)-\(p)"
//     }
// }
