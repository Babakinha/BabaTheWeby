#/bin/sh
# Btw u need perseus-cli installed
rm -rf docs
perseus export --release
cp --dereference --recursive ./dist/exported/ docs
