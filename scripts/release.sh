#!/bin/sh

FILES=$(find target/release -maxdepth 1 -name "carina_*" -executable)

for file in $FILES
do
  echo "Current file: $file"
  echo "Size: $(du -h $file | cut -f -1)"
  echo "Stripping, just for you"
  strip $file
  echo "Done. New size: $(du -h $file | cut -f -1)"
  echo ""
done