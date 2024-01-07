build:
	cargo build --release

install: install-assets
	cp ./target/release/eye-break ~/.local/bin
	cp ./eye-break.service ~/.config/systemd/user
	systemctl --user daemon-reload

install-assets:
	mkdir $(HOME)/.config/eye-break
	cp ./assets/take_break.gif $(HOME)/.config/eye-break/take_break.gif
	cp ./assets/break_done.gif $(HOME)/.config/eye-break/break_done.gif
	cp ./assets/icon.ico $(HOME)/.config/eye-break/icon.ico

uninstall: remove-assets
	systemctl --user stop eye-break
	systemctl --user disable eye-break
	rm ~/.local/bin/eye-break
	rm ~/.config/systemd/user/eye-break.service
	systemctl --user daemon-reload

remove-assets:
	sudo rm -r $(HOME)/.config/eye-break

clean:
	cargo clean