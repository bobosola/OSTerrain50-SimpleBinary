#!/opt/homebrew/bin/bash

# Signs a binary with the Apple "Developer ID Application" cert
# then creates & signs a DMG which contains the binary
# then notarizes the DMG and staples the notarization ticket
# to it so that Gatekeeper security requirements are fully satisfied.

# See "Signing a Mac Product For Distribution" at
# https://developer.apple.com/forums/thread/128166 and linked page
# "Customizing the Notarization Workflow" at 
# https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow


# The binary to be signed and put in a disk image DMG file
APP="/Users/bobosola/rust/osterrain50/target/universal/osterrain50"

# Apple "Developer ID Application" certificate installed in Keychain
CERT="Developer ID Application: Robert Osola"

# Arbitrary unique bundle identifier required to sign the DMG
BUNDLE_ID="uk.org.osola.osterrain50"

# Source directory of content to put inside DMG
IMG_SRC="/Users/bobosola/rust/osterrain50/target/universal"

# Output path and name of DMG 
IMG_DEST="/Users/bobosola/Desktop/OSTerrain50.dmg"

# The mounted volume name when the DMG is opened
IMG_VOL_NAME="OSTerrain50"

# Sign the binary (force overwrite, verbose and harden with '-o runtime')
codesign -s "$CERT" -fv -o runtime --timestamp "$APP" 

# Check the binary is signed
codesign -dv --verbose $APP

# Create the DMG with the signed binary inside it
hdiutil create -srcFolder "$IMG_SRC" -o "$IMG_DEST" -volname "$IMG_VOL_NAME"

# Sign the DMG
codesign -s "$CERT" -fv --timestamp -i "$BUNDLE_ID" "$IMG_DEST"

# Check the DMG is signed
codesign -dv --verbose "$IMG_DEST"

# Send the DMG to Apple for notarization using saved keychain
# profile app password AC_PASSWORD and wait for completion.
# NB: AC_PASSWORD is created as per instructions in
# "Customizing the Notarization Workflow" to avoid having actual
# passwords in scripts.
xcrun notarytool submit "$IMG_DEST" --keychain-profile "AC_PASSWORD" --wait

# Wait for the service to return a message...

# Assuming success, staple the ticket to the DMG
xcrun stapler staple "$IMG_DEST"

# Final check to test certificate & notarization have been successfully applied
spctl -a -vv -t install "$IMG_DEST"
