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
        $ sudo apt install postgresql libpq-dev
        ```
    3. Diesel client (see [https://diesel.rs/](https://diesel.rs/))
        ```shell script
        $ cargo install diesel_cli --no-default-features --features postgres
        ``` 
2. Make sure that you have a user and password set up in postgres and can log in
    with a password. You may have to modify `/etc/postgresql/12/main/pg_hba.conf` 
    (or something along those lines) to use md5 authentication rather than peer 
    authentication. You will know this works when you can log in to postgres 
    using
    ```shell script
    $ psql -U <username> -W
    ```
3. Clone this repository.
4. Generate self-signed TLS/SSL certificate and keys for testing: 
    ```shell script
    $ mkdir tls-ssl
    $ openssl req -x509 -newkey rsa:4096 -nodes -keyout tls-ssl/private-key.pem -out tls-ssl/certificate.pem -days 365
    ```
   If you are running this in production, do not do this. Instead, you should use
   a certificate signed by a trusted certificate authority. See 
   [https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs](https://phoenixnap.com/kb/openssl-tutorial-ssl-certificates-private-keys-csrs)
   for more details.
5. Create a `.env` file to store your database config and other environment 
    variables to be used at runtime. 
    ```shell script
    DATABASE_URL="postgres://<username>:<password>@localhost/<database>"
    ```
6. Run the database migrations (this will create a development database locally).
    ```shell script
    $ diesel migration run
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

### Installation Debugging:
If you have issues running the website, here are a few things you can check/try:
1. All prerequisites are installed.
2. If you use Windows Subsystem for Linux, make sure that it's installed correctly. There have recently been issues with the installation of WSL2 and Rust: https://github.com/rust-lang/rustup/issues/2293
3. One you compile the website, launch it at the local IP address that your terminal generates. Make sure that 'https://' precedes the IP address.
4. If you get a blank page when launching the website, you need to regenerate your TLS/SSL certificate and keys. If that doesn't work, then switch browsers (e.g. Firefox, Chrome).
