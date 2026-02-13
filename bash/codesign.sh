#!/opt/homebrew/bin/bash

# Last updated 12-Feb-2026
# 
# 1) Signs a binary with your Apple "Developer ID Application" certificate
# 2) Creates & signs a DMG and adds the binary to it
# 3) Notarizes the DMG and staples the notarization ticket to it so that
#    Gatekeeper security requirements are fully satisfied.
# 
# For information about notarization, see:
# https://developer.apple.com/documentation/security/customizing-the-notarization-workflow#Upload-your-app-to-the-notarization-service
#
# Notarizing by script requires these two items:
# 
#  (i) an Apple "Developer ID Application" certificate in your keychain (costs $99 per annum)
#      NB: If your certificate or membership expires, users can still install packages that were signed with this 
#      certificate while it was valid so long as the package includes a timestamp (added below)
# 
# (ii) an 'App-Specifc Password' for the CLI app 'notarytool', created thus:
#      Sign in to your regular (non dev) Apple account at https://account.apple.com
#      Select the option for 'App-Specific Passwords'
#      Hit the plus button, then in the prompt box enter: notarytool
#      It will then create a password for you - save the password to a temp location for later use
#      Now log in to you developer account at https://developer.apple.com and go to 'Account->Membership Details'
#      Take a note of your Developer ID (probably your email address) and Team ID (probably assigned by Apple)
#      Now run in a terminal: notarytool store-credentials --apple-id "{your developer id}" --team-id "{your team id}"
#      You should then get a prompt to enter Profile Name, enter (say): NOTARY_PASSWORD
#      Then you will get a weird non-responsive padlock sign (i.e no visible prompt) but it's still active! 
#      Just type out the password you saved earlier (into seemingly nothing) then hit Return.
#      You should get a success message like this:
#           
#           Profile name:
#           NOTARYTOOL_PASSWORD
#           App-specific password for {your developer id}:
#           Validating your credentials...
#           Success. Credentials validated.
#           Credentials saved to Keychain.
#           To use them, specify `--keychain-profile "NOTARYTOOL_PASSWORD"`
# 
# Once all the above has been completed, we can do the rest...

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

# Sign the binary (force overwrite, verbose, harden with '-o runtime', and add a timestamp)
printf "\nSign the binary ....\n"
codesign -s "$CERT" -fv -o runtime --timestamp "$APP" 
printf "Binary signed ....\n\n"

# Check the binary is signed
printf "Check the signing ....\n"
codesign -dv --verbose $APP

# Create the DMG with the signed binary inside it
printf "\nCreate the DMG ....\n"
hdiutil create -srcFolder "$IMG_SRC" -o "$IMG_DEST" -volname "$IMG_VOL_NAME"

# Sign the DMG
printf "\nSign the DMG ....\n"
codesign -s "$CERT" -fv --timestamp -i "$BUNDLE_ID" "$IMG_DEST"

# Check the DMG is signed
printf "\nCheck the signing ....\n"
codesign -dv --verbose "$IMG_DEST"

# Notarise the DMG (this also notarizes all content inside it)
printf "\nNotarize the DMG ....\n"
xcrun notarytool submit "$IMG_DEST" --keychain-profile "NOTARYTOOL_PASSWORD" --wait

# Wait for the service to return a message, should take a minute or so...

# If you get a failure message like:
# 
#    Current status: Rejected..........
#    Processing complete
#    id: 60f42301-4836-47b2-894a-c4eae0dce83c
#    status: Rejected
# 
# You can get the failure reason by entering in a browser:
# https://appstoreconnect.apple.com/notary/v2/submissions/{id from the failure message}/logs
# It will download a JSON log file. Open the log file and extract the 'developerLogUrl' value (a very long URL)
# Enter the 'developerLogUrl' value into a browser. It will download  a 'developer_log.json' file with error details.
# It's the exact same failure log you get if you tried and failed to notarize in XCode at:
# Main top menu > Product > Archive > Distribute App > Custom > Upload > {either certificate option} > Upload
# The failure log may have some advice, but don't bank on it :-(

# Assuming success, staple the ticket to the DMG
printf "\nStaple the DMG ....\n"
xcrun stapler staple "$IMG_DEST"

# Final check to test the DMG certificate & notarization
printf "\nheck the DMG cert and notarization ....\n"
spctl -a -vv -t install "$IMG_DEST"
