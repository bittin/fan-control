
run:
	clear && cargo run

fake:
	clear && cargo run --features fake_hardware -- -p ./.config

conf:
	clear && cargo run -- -p ./.config

config:
	clear && ./target/debug/fan-control -p ./config

release:
	clear && cargo run --release

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

git-cache:
	git rm -rf --cached .
	git add .

expand:
	clear && cargo expand

libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../target/libsensors_build ETCDIR=./../../target/libsensors_build/etc

clean-libsensors:
	make -C ./hardware/libsensors/ clean uninstall PREFIX=./../../target/libsensors_build ETCDIR=./../../target/libsensors_build/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c release

run-lhm:
	dotnet run --project ./hardware/LibreHardwareMonitorWrapper/ -c release

test:
	cargo test --all --all-features


package-linux:
	cargo bundle --release


.PHONY: clean-libsensors libsensors