FROM rust:buster

WORKDIR /tmp

RUN wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > microsoft.asc.gpg
RUN mv microsoft.asc.gpg /etc/apt/trusted.gpg.d/
RUN wget -q https://packages.microsoft.com/config/debian/10/prod.list
RUN mv prod.list /etc/apt/sources.list.d/microsoft-prod.list
RUN chown root:root /etc/apt/trusted.gpg.d/microsoft.asc.gpg
RUN chown root:root /etc/apt/sources.list.d/microsoft-prod.list
RUN apt update
RUN apt install azure-functions-core-tools-2 musl-tools build-essential zlib1g-dev -y
RUN wget http://www.openssl.org/source/openssl-1.0.1g.tar.gz && \
    tar -xvzf openssl-1.0.1g.tar.gz && \
    cd openssl-1.0.1g && \
    ./config --prefix=/usr/local/openssl --openssldir=/usr/local/openssl && \
    make && \
    make install_sw
ENV OPENSSL_DIR /usr/local/openssl

WORKDIR /usr/src/app

COPY . /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl       
RUN cargo build --release --target=x86_64-unknown-linux-musl && \ 
    cp target/x86_64-unknown-linux-musl/release/handler .

ENV AzureWebJobsScriptRoot=/usr/src/app \
    AzureFunctionsJobHost__Logging__Console__IsEnabled=true

# CMD ["func", "start"]
