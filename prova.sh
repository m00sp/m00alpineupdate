#!/bin/sh
apks=$(sudo apk upgrade -vvs | wc -l)
dif=1
result=$(echo "$apks - $dif" | bc)
echo $result
