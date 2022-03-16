FROM docker.io/rust:latest

WORKDIR /usr/src/birthdays
COPY . .

RUN cargo install --path .

CMD ["birthdays"]

