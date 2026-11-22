import Foundation
import Library
import Security
import SwiftUI

/// Where InfluxDB is and the tokens to reach it, kept in the Keychain. Empty
/// settings mean the environment's (INFLUXDB3_AUTH_TOKEN and so on, as
/// `swift run` and simulator.sh pass them); a phone needs them set here.
struct ServerSettings: Equatable {
    var url = ""
    var token = ""
    var accessClientId = ""
    var accessClientSecret = ""

    /// Through Cloudflare (tunnel/README.md), from anywhere.
    static let cloudflareURL = "https://influx.atec.pub"

    var isEmpty: Bool { token.isEmpty }

    static func load() -> ServerSettings {
        ServerSettings(
            url: Keychain.get("url") ?? "",
            token: Keychain.get("token") ?? "",
            accessClientId: Keychain.get("accessClientId") ?? "",
            accessClientSecret: Keychain.get("accessClientSecret") ?? "")
    }

    func save() {
        Keychain.set("url", url)
        Keychain.set("token", token)
        Keychain.set("accessClientId", accessClientId)
        Keychain.set("accessClientSecret", accessClientSecret)
    }

    /// Point the Rust client at this server, or back at the environment.
    func apply() {
        guard !isEmpty else {
            Library.configure(server: nil)
            return
        }
        let trimmed = { (s: String) -> String? in
            let t = s.trimmingCharacters(in: .whitespacesAndNewlines)
            return t.isEmpty ? nil : t
        }
        Library.configure(server: Server(
            url: trimmed(url) ?? Self.cloudflareURL,
            token: token.trimmingCharacters(in: .whitespacesAndNewlines),
            accessClientId: trimmed(accessClientId),
            accessClientSecret: trimmed(accessClientSecret)))
    }
}

/// Strings in the Keychain, this device only (not synced through iCloud).
enum Keychain {
    private static let service = "pub.atec.Equities"

    private static func query(_ key: String) -> [String: Any] {
        [kSecClass as String: kSecClassGenericPassword,
         kSecAttrService as String: service,
         kSecAttrAccount as String: key]
    }

    static func get(_ key: String) -> String? {
        var q = query(key)
        q[kSecReturnData as String] = true
        q[kSecMatchLimit as String] = kSecMatchLimitOne
        var item: CFTypeRef?
        guard SecItemCopyMatching(q as CFDictionary, &item) == errSecSuccess,
              let data = item as? Data else { return nil }
        return String(data: data, encoding: .utf8)
    }

    static func set(_ key: String, _ value: String) {
        SecItemDelete(query(key) as CFDictionary)
        guard !value.isEmpty else { return }
        var q = query(key)
        q[kSecValueData as String] = Data(value.utf8)
        q[kSecAttrAccessible as String] = kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
        SecItemAdd(q as CFDictionary, nil)
    }
}

struct SettingsView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var settings = ServerSettings.load()
    /// Called after saving, to reload with the new server.
    let onSave: () -> Void

    var body: some View {
        NavigationStack {
            Form {
                Section {
                    TextField(ServerSettings.cloudflareURL, text: $settings.url)
                        .autocorrectionDisabled()
                        #if os(iOS)
                        .textInputAutocapitalization(.never)
                        .keyboardType(.URL)
                        #endif
                    SecureField("token", text: $settings.token)
                } header: {
                    Text("InfluxDB")
                } footer: {
                    Text("Empty address: \(ServerSettings.cloudflareURL). Empty token: the environment's, as from swift run or simulator.sh.")
                }
                Section {
                    TextField("client ID", text: $settings.accessClientId)
                        .autocorrectionDisabled()
                        #if os(iOS)
                        .textInputAutocapitalization(.never)
                        #endif
                    SecureField("client secret", text: $settings.accessClientSecret)
                } header: {
                    Text("Cloudflare Access service token")
                } footer: {
                    Text("For a server behind Cloudflare Access: equities-app, in ~/.cloudflared/equities-app.env on the Mac.")
                }
            }
            .navigationTitle("Server")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        settings.save()
                        settings.apply()
                        onSave()
                        dismiss()
                    }
                }
            }
        }
        #if os(macOS)
        .frame(minWidth: 460, minHeight: 360)
        #endif
    }
}
