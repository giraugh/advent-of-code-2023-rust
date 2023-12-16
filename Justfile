prepare DAY:
	cargo init day{{DAY}}
	rm ./day{{DAY}}/src/main.rs
	ln ./main.tmpl.rs ./day{{DAY}}/src/main.rs
	cp ./puzzle.tmpl.rs ./day{{DAY}}/src/puzzle.rs
	touch ./day{{DAY}}/input.txt
	touch ./day{{DAY}}/sample.txt
	cd ./day{{DAY}} && cargo add --path ../aoc

test DAY:
	cd day{{DAY}} && cargo test
