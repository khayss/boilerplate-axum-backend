server:
	@cargo watch -q -c -w src/ -x run

dev:
	@cargo watch -q -c -w examples/ -x "run --example dev"