test:
	rm -rf gpui-docview-test || true
	cargo run -- --name gpui-docview-test
	cd gpui-docview-test; cargo run -p gpui-docview-test