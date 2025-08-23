install:
	cargo install cargo-license cargo-about

bundle-license:
	cargo about generate about.hbs > ./license.html
