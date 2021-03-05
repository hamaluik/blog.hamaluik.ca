#!/bin/bash
for f in `find . -name "*.HEIC" -type f`;
do
    heif-convert -q 100 "$f" "$f.png"
done

