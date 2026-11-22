#!/bin/zsh
# Build the iOS app and run it in the Simulator, reading InfluxDB on this Mac.
#
#     equities/app/simulator.sh [DEVICE]      e.g. "iPhone 15 Pro" (the default)
#     EQUITIES_TICKER=aapl equities/app/simulator.sh    # open a company at launch
#
# The Simulator shares the Mac's network, so the app reaches InfluxDB at
# localhost:8181 as the macOS app does. Its token comes from ~/.profile, like
# every job here; on a real phone it would have to come from the Keychain.
# Needs jq (brew install jq).
#
# Library (the Rust code, as a Swift package) must already be built for iOS:
#
#     cargo swift package -p macos@12 ios@15 -n Library -y    # from the workspace root

# INFLUXDB3_AUTH_TOKEN.
source ~/.profile
device=${1:-iPhone 15 Pro}
# The directory this script is in: equities/app.
cd ${0:A:h} || exit 1

# Several simulators can share a name (one per iOS version installed), so
# pick one by its ID: a running one if there is, else the newest iOS.
id=$(xcrun simctl list devices available -j | jq -r --arg name "$device" '
  [.devices | to_entries[] | select(.key | contains("iOS")) | .key as $os
   | .value[] | select(.name == $name) | {id: .udid, os: $os, booted: (.state == "Booted")}]
  | sort_by(.booted, .os) | last | .id // empty')
[ -n "$id" ] || { echo "no simulator named $device (xcrun simctl list devices)" >&2; exit 1 }

# Build for the Simulator into ./build (git-ignored).
xcodebuild -project Equities.xcodeproj -scheme Equities -configuration Debug \
  -destination "platform=iOS Simulator,id=$id" -derivedDataPath build \
  -quiet build || exit 1

# Boot the simulated phone, unless it is already running, and show it.
xcrun simctl boot $id 2>/dev/null
open -a Simulator

# Install the app, replacing any earlier build.
xcrun simctl install $id build/Build/Products/Debug-iphonesimulator/Equities.app || exit 1

# Launch it. simctl passes variables prefixed SIMCTL_CHILD_ into the app's
# environment with the prefix removed, so the Rust code sees
# INFLUXDB3_AUTH_TOKEN just as it does on the Mac. --terminate-running-process
# restarts the app if it was already open, so a new build takes effect.
SIMCTL_CHILD_INFLUXDB3_AUTH_TOKEN=$INFLUXDB3_AUTH_TOKEN SIMCTL_CHILD_EQUITIES_TICKER=$EQUITIES_TICKER \
  xcrun simctl launch --terminate-running-process $id pub.atec.Equities
