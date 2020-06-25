# Telescope
Telescope intends to replace [Observatory](https://github.com/rcos/observatory-server) 
as the RCOS website.

### Prerequisites:
1. Install dependencies:
    1. Rust (see [https://www.rust-lang.org/](https://www.rust-lang.org/))
        ```shell script
        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        $ source ~/.cargo/env
        ```
    2. Postgres (see [https://www.postgresql.org/](https://www.postgresql.org/))
        ```shell script
        $ sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
        $ wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
        $ sudo apt update
        $ sudo apt-get install postgresql libpq-dev
        ```
    3. Diesel client (see [https://diesel.rs/](https://diesel.rs/))
        ```shell script
        $ cargo install diesel_cli --no-default-features --features postgres
        ``` 
2. Clone this repository.
3. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.
4. Create a `.env` file to store your database config and other environment 
    variables to be used at runtime. 
    ```shell script
    DATABASE_URL="postgres://username:password@localhost/telescope"
    ```

### Running:
```shell script
$ # Usage:
$ cargo run -- -h

$ # Development profile (for testing on local machine).
$ cargo run -- --development

$ # Production profile (used for deploying in production environments).
$ cargo run --release -- --production
```
