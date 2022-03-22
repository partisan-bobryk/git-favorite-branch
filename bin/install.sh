#!/bin/bash

VERSION=$1
TARGET=$2
TMP_DIR="./tmp/gfb"
TMP_FILE="$TMP_DIR/gfb.zip"
INSTALL_DIR="/usr/local/revent-studio/gfb-$VERSION"
INSTALL_BIN="$INSTALL_DIR/gfb"

# Prep work
mkdir -p $TMP_DIR
sudo mkdir -p $INSTALL_DIR

# Perform The download
wget "https://github.com/VeprUA/git-favorite-branch/releases/download/$VERSION/gfb-$TARGET.zip" -O $TMP_FILE
unzip -d $TMP_DIR $TMP_FILE 

chmod +x "$TMP_DIR/gfb"

# Installation
sudo mv "$TMP_DIR/gfb" $INSTALL_BIN
sudo ln -sf $INSTALL_BIN /usr/local/bin/gfb

# Clean up
rm -rf ./tmp
