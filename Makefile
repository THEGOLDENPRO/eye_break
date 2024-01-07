build:
	cargo build --release

install:
	cp ./target/release/eye-break /usr/local/bin
	cp ./eye-break.service /lib/systemd/system
	systemctl daemon-reload

install-assets:
	mkdir $(HOME)/.config/eye-break
	cp ./assets/take_break.gif $(HOME)/.config/eye-break/take_break.gif
	cp ./assets/break_done.gif $(HOME)/.config/eye-break/break_done.gif
	cp ./assets/icon.ico $(HOME)/.config/eye-break/icon.ico

uninstall:
	systemctl stop eye-break
	systemctl disable eye-break
	rm /usr/local/bin/eye-break
	rm /lib/systemd/system/eye-break.service
	systemctl daemon-reload

remove-assets:
	rm -r $(HOME)/.config/eye-break

clean:
	cargo clean