#!/bin/zsh
# Publish the Rust library for Swift apps: build bibleFFI.xcframework (iOS,
# the iOS Simulator, macOS; release), zip it, and attach it to a GitHub
# release of lib, tagged library-YYYYMMDD-<commit>. Apps depend on it as a
# Swift package binary target by URL and checksum, so their builds (Xcode
# Cloud's included) never compile Rust.
#
#     Library/release.sh [--no-build] [--dry-run] [--into DIR]
#
# --dry-run packages target/library-release/ and stops before releasing.
#
# --into DIR writes the released package into DIR/Library: Package.swift
# pointing at the release, and the Swift bindings (Sources/Library) that
# cargo swift generated with this very build, which must match the binary.
# (DIR is an app's checkout, e.g. ~/optimates.)
#
# Run from a clean checkout whose commit is pushed to origin: the release's
# tag points at it.
set -euo pipefail
LIB=${0:A:h:h}
cd $LIB

build=1 into="" dry=0
while (( $# )); do
  case $1 in
    --no-build) build=0 ;;
    --dry-run) dry=1 ;;
    --into) into=${2:?--into needs a directory}; shift ;;
    *) echo "usage: release.sh [--no-build] [--dry-run] [--into DIR]" >&2; exit 2 ;;
  esac
  shift
done

sha=$(git rev-parse HEAD)
git fetch -q origin
(( dry )) || git branch -r --contains $sha | grep -q origin/ || { echo "commit $sha is not on origin: push it first" >&2; exit 1 }

(( build )) && cargo swift package -p macos@12 ios@15 -n Library -y -r
# cargo swift rewrites Info.plist with its platforms in another order.
git checkout -q -- Library/bibleFFI.xcframework/Info.plist
[[ -z $(git status --porcelain -- Library/Sources Library/Package.swift Library/bibleFFI.xcframework) ]] || { echo "Library/ differs from the commit: commit the bindings first" >&2; exit 1 }

tag=library-$(date +%Y%m%d)-$(git rev-parse --short=7 HEAD)
out=$LIB/target/library-release
rm -rf $out && mkdir -p $out
# cargo swift puts every crate's C module (nameFFI, influxFFI, ...) in one
# module.modulemap under Headers/bibleFFI/. Xcode 26 and later find a module
# by a folder of its name, so only bibleFFI resolved ("Unable to resolve
# module dependency: 'nameFFI'" in Xcode Cloud). Give each module its own
# folder: Headers/<module>/{<module>.h, module.modulemap}.
cp -R Library/bibleFFI.xcframework $out/
for headers in $out/bibleFFI.xcframework/*/Headers; do
  # Aside first: bibleFFI's own folder is the shared one's name.
  shared=$headers/.shared
  mv $headers/bibleFFI $shared
  awk -v dir=$headers '
    /^module [A-Za-z0-9_]+ \{/ { name = $2; file = dir "/" name ".modulemap.part" }
    name != "" { print > file }
    /^\}/ { close(file); name = "" }
  ' $shared/module.modulemap
  for part in $headers/*.modulemap.part; do
    module=${${part:t}%.modulemap.part}
    mkdir -p $headers/$module
    mv $part $headers/$module/module.modulemap
    mv $shared/$module.h $headers/$module/
  done
  rm $shared/module.modulemap
  rmdir $shared 2>/dev/null || { echo "headers left without a module in $shared" >&2; exit 1 }
done
(cd $out && zip -qry bibleFFI.xcframework.zip bibleFFI.xcframework)
checksum=$(swift package compute-checksum $out/bibleFFI.xcframework.zip)
(( dry )) && { echo "packaged $out (checksum $checksum); not released"; exit 0 }
url=https://github.com/atecce/lib/releases/download/$tag/bibleFFI.xcframework.zip

gh release create $tag $out/bibleFFI.xcframework.zip --repo atecce/lib --target $sha \
  --title "Library $tag" \
  --notes "bibleFFI.xcframework (iOS, iOS Simulator, macOS) built from $sha by Library/release.sh.

Swift package binary target:
    .binaryTarget(name: \"bibleFFI\", url: \"$url\", checksum: \"$checksum\")"

echo "url      $url"
echo "checksum $checksum"

if [[ -n $into ]]; then
  dest=${into:A}/Library
  rm -rf $dest/Sources/Library && mkdir -p $dest/Sources/Library
  cp Library/Sources/Library/*.swift $dest/Sources/Library/
  sed -e "s|.binaryTarget(name: \"bibleFFI\", path: \"./bibleFFI.xcframework\")|.binaryTarget(\n            name: \"bibleFFI\",\n            // Library/release.sh in atecce/lib, $tag ($sha)\n            url: \"$url\",\n            checksum: \"$checksum\"\n        )|" \
    Library/Package.swift > $dest/Package.swift
  grep -q "$checksum" $dest/Package.swift || { echo "could not point $dest/Package.swift at the release" >&2; exit 1 }
  echo "wrote $dest"
fi
