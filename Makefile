publish:
	make git
	make doc
	cargo publish
doc:
	cargo doc --no-deps
git:
	git add -A
	git commit -m "Update"
	git push 