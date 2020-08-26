distill:
	cd distill
	npm run build

generator:
	cd generator
	cargo build --release

build: distill generator
	rm -rf dist
	mkdir -p dist
	cp distill/dist/template* ./dist/
	
clean:
	rm -rf dist

install: distill
	echo "TODO"
