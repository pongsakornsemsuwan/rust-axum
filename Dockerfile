FROM rust:1.67

WORKDIR /app
COPY . .

RUN cargo install cargo-watch
RUN cargo install --path .

CMD ["app"]

EXPOSE 3000