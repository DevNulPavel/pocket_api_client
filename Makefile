.SILENT:

ENCRYPT_TEST_ENV:
	gpg -a -r 0x0BD10E4E6E578FB6 -o .test.env.asc -e .test.env

DECRYPT_TEST_ENV:
	rm -rf .env
	gpg -a -r 0x0BD10E4E6E578FB6 -o .test.env -d .test.env.asc

TEST:
	cargo test -- --nocapture