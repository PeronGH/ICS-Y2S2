#!/bin/sh

echo table.png > /tmp/extracted-tables.txt
cat /tmp/extracted-tables.txt | xargs -I{} python3 -m table_ocr.extract_cells {} | grep cells > /tmp/extracted-cells.txt
cat /tmp/extracted-cells.txt | xargs -I{} python3 -m table_ocr.ocr_image {}

for image in $(cat /tmp/extracted-tables.txt); do
    dir=$(dirname $image)
    python3 -m table_ocr.ocr_to_csv $(find $dir/cells -name "*.txt")
done