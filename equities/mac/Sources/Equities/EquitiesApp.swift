import AppKit
import SwiftUI

@main
struct EquitiesApp: App {
    init() {
        // Run from `swift run` there is no app bundle: make it a regular app
        // with a Dock icon and a window that takes focus.
        NSApplication.shared.setActivationPolicy(.regular)
        NSApplication.shared.activate(ignoringOtherApps: true)
    }

    var body: some Scene {
        WindowGroup("Equities") {
            ContentView()
                .frame(minWidth: 900, minHeight: 600)
        }
    }
}
