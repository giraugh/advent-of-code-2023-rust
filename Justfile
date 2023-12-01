prepare DAY:
	cargo init day{{DAY}}
	rm ./day{{DAY}}/src/main.rs
	ln ./main.tmpl.rs ./day{{DAY}}/src/main.rs
	cp ./puzzle.tmpl.rs ./day{{DAY}}/src/puzzle.rs
	touch ./day{{DAY}}/input.txt

test DAY:
	cd day{{DAY}} && cargo test
