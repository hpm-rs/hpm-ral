git diff > ./diff
file_size=$(stat --printf="%s" ./diff)

if [ $file_size == 0 ]; then
    echo "No difference found"
    exit 0
else
    echo "Found difference"
    git --no-pager diff
    exit 1
fi
