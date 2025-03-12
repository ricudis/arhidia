# arhidia
Quick and dirty mass MD5 file hasher

Sometimes I need a tool to easily find identical files on two large sets of files,
dispersed across bandwidth restricted connections.

So I wrote this.

It just recursively walks a directory, computes the MD5 checksum of each file, and prints the
file size, checksum, and filename. Can also optionally hash only the first N 4096-byte blocks
of each file.

The name arhidia is derived from the Greek words for "files" ("αρχεία") and "identical" ("ίδια").

Believe me. I'm a Greek. That's what it really means.

Having used the Rust clap crate to parse command line arguments, i was tempted to name it claparhidia
instead, but I finally decided this would be too much.

I deliberately use MD5 for compatibility with the md5sum tool. HOW COOL IS THAT!