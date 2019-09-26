FROM rustlang/rust:nightly
MAINTAINER Mikkel Kroman <mk@maero.dk>

WORKDIR /usr/src/app

ENV PORT 8000
EXPOSE 8000

COPY . .

RUN apt update && \
  apt install -y musl-dev musl-tools && \
   rm -rf /var/lib/apt/lists/*;
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip --strip-unneeded target/x86_64-unknown-linux-musl/release/meta-title_fetcher

FROM alpine:latest
WORKDIR /app
RUN apk --no-cache add ca-certificates
COPY --from=0 /usr/src/app/target/x86_64-unknown-linux-musl/release/meta-title_fetcher .

CMD ["./meta-title_fetcher"]
