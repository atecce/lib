import SwiftUI
#if os(macOS)
import AppKit
#endif

@main
struct EquitiesApp: App {
    init() {
        #if os(macOS)
        // Run from `swift run` there is no app bundle: make it a regular app
        // with a Dock icon and a window that takes focus.
        NSApplication.shared.setActivationPolicy(.regular)
        NSApplication.shared.activate(ignoringOtherApps: true)
        #endif
    }

    var body: some Scene {
        WindowGroup("Equities") {
            #if os(macOS)
            ContentView().frame(minWidth: 900, minHeight: 600)
            #else
            ContentView()
            #endif
        }
    }
}
