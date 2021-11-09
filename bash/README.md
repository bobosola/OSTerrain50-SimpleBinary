<h1>Utility Scripts</h1>

``release.sh`` builds a Universal Binary for Intel x86_64 & M1 ARM64 macs.

``unzip.sh`` is for use where you may want to unzip the OS zip file independently. Written for use on Macs but easily editable for other Unix-like platforms. The OS zip file is difficult to unzip by hand as the parent zip contains 2858 child zips in multiple sub-directory trees.

``codesign.sh`` Mac only. This handles packaging, code signing and notarization to satisfy Gatekeeper requirements for Macs running macOS Catalina or later. This requires paid-for membership of the Apple Developer Program which then allows an Apple "Developer ID Application" certificate to be installed locally in the keychain. More details are in the script.