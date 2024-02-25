compress-mac: bundle-mac 
  create-dmg appvis-darwin-arm64.dmg ./target/release/bundle/osx/Appvis.app 

bundle-mac:
  cargo bundle --release
