#!/bin/bash

if [ "$EUID" -ne 0 ]; then
  echo "Root access required."
  exit 1
fi

# Compile and install binary
cargo build --release
install -m 755 ./target/release/oui /usr/bin/oui

# Copy assets
mkdir -p ~/.local/share/oui/
cp -f ./assets/IEEE_OUI.csv ~/.local/share/oui/

# Install manpage
gzip < ./man/oui.1 > ./man/man.1.gz
install -m 644 ./man/man.1.gz /usr/share/man/man1/
mandb

exit 0
