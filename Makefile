COOKIE='session=53616c7465645f5f4613e37e4d59e1c7f2d92b8b48bbb881a04374370e37ddbeec1e68721ec637c7802d4a51ed0d405e49a60119b4cbfe70ba14bd8ebedd0fec'
YEAR=2024
%:
	mkdir -p src/bin/day$@
	cp -u src/bin/.skeleton.rs src/bin/day$@/main.rs
	curl -b $(COOKIE) https://adventofcode.com/$(YEAR)/day/$@/input > src/bin/day$@/input.txt
	code src/bin/day$@/main.rs
