# Ratrix

Ratrix is a small Rust application that helps reading the CGD (Caixa Geral de Depósitos) matrix card which is used in the homebanking application for secure operations.
It leverages the GPG (Gnu PGP) tool to secure the file with the matrix card data.

## Requirements
* Rust compiler
* Cargo tool
* gpg command line tool

## Getting Started

Have a CGD Matrix card. If you're reading this I assume you have one.

1. Create a file with the representation of the matrix card. It should be an 8x8 matrix identical to your matrix card:
    ```
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    111 222 333 444 555 666 777 888
    ```
    
2. After creating the file (in any directory you like) and in order to secure it encrypt the file with your PGP public key. E.g.

    ```bash
    gpg --output <encrypted-filename> --encrypt --recipient <key-identifier> <file-to-encrypt>

    ```

    You can now safely delete the plain text file.

3. Have `Rust` and `Cargo` tool installed.

4. a) Download or clone the code and run it everytime you require in a transaction:
    ```bash
    cargo run -- --file <encrypted-file-location> --positions A11,B21,D31
    ```
    It should ask for your gpg key password to decrypt it. It will use the configured [pinentry](https://www.gnupg.org/related_software/pinentry/index.html) in your system.
    The positions should be provided as requested in the transaction. For instance `A11` represents the **first** number of the set of numbers in the `A1` position of the matrix.

4. b) Alternative run `cargo install` and run it as some program in the `PATH`:
    ```bash
    ratrix --file <encrypted-file-location> --positions A11,B21,D31
    ```

And that's it! While there is no binary release this is the procedure.

## Misc
I chose to use the `gpg` tool instead of some security crate because it is something that most users have and is tried and tested. Most of the crates I found out there were not really certified nor inspired much confidence. As a tradeoff it requires the users to have the tool installed making it a little more cumbersome to run in some systems.
