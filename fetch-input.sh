mkdir $DAY
curl "https://adventofcode.com/2024/day/$DAY/input" --header "Cookie: $(cat .token)" > $DAY/input.txt