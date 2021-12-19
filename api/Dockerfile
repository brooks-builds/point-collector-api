FROM rust:1
RUN cargo install cargo-watch
VOLUME /code
WORKDIR /code
CMD cargo watch -x run
EXPOSE 8000