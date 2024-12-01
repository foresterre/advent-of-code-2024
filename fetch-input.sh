mkdir -p $DAY
curl "https://adventofcode.com/2024/day/$DAY/input" --header "Cookie: session=$(cat .token)" > $DAY/input.txt