distill:
	cd distill;npm run build

gen:
	cd generator;cargo build

build:
	rm -rf public
	mkdir -p public
	cp distill/dist/template* ./public/
	generator/target/debug/gen build

clean:
	rm -rf public

install:
	echo "TODO"
