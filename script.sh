#!/bin/bash

# Make sure we have the arguments we need
if [[ -z $1 || -z $2 ]]; then
    echo "Generates a macOS bundle from a compiled dylib file"
    echo "Example:"
    echo -e "\t$0 Plugin target/release/plugin.dylib"
    echo -e "\tCreates a Plugin.vst bundle"
else
    # Make the bundle folder
    mkdir -p "$1.vst/Contents/MacOS"

    # Create the PkgInfo
    echo "BNDL????" > "$1.vst/Contents/PkgInfo"

    #build the Info.Plist
    echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>

    <key>CFBundleExecutable</key>
    <string>$1</string>

    <key>CFBundleGetInfoString</key>
    <string>vst</string>

    <key>CFBundleIconFile</key>
    <string></string>

    <key>CFBundleIdentifier</key>
    <string>com.rust-vst.$1</string>

    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>

    <key>CFBundleName</key>
    <string>$1</string>

    <key>CFBundlePackageType</key>
    <string>BNDL</string>

    <key>CFBundleVersion</key>
    <string>1.0</string>

    <key>CFBundleSignature</key>
    <string>$((RANDOM % 9999))</string>

    <key>CSResourcesFileMapped</key>
    <string></string>

</dict>
</plist>" > "$1.vst/Contents/Info.plist"

    # move the provided library to the correct location
    cp "$2" "$1.vst/Contents/MacOS/$1"

    echo "Created bundle $1.vst"
fi