# docker.io/guzmansalv/youtube_dbase_scraper
FROM liuchong/rustup:musl AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-build-deps --verbose --color always
RUN cargo build-deps --release

ADD src src

ARG name=dbase-scraper
RUN cargo build --package $name --bin $name --verbose --jobs 2 --all-features --release --target=x86_64-unknown-linux-musl --color always

FROM scratch
COPY --from=base /root/app/target/x86_64-unknown-linux-musl/release/dbase-scraper /main

ENTRYPOINT ["/main"]