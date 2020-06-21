# boot_loader_signer

This program simply writes to the byte locations 511 and 512 of a file with the byte values of 0x55 and 0xaa.

This is mainly useful if you haven't written the magic bytes already in the boot loader code itself.

To build:

cargo build --release

To run:

target/release/boot_loader_signer <boot_loader_binary_file>
