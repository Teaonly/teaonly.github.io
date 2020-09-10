.DEFAULT_GOAL := build
build:
	rm -rf public
	mkdir -p public
	cd distill;npm run build
	cd generator;cargo build
	cd site/svelte;./build.sh
	cp distill/dist/template* ./public/
	generator/target/debug/gen

clean:
	rm -rf public

install:
	echo "TODO"
