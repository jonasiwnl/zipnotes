FROM rust:slim-bookworm

RUN rustup target add wasm32-unknown-unknown

ADD https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz ./tmp
RUN cd /tmp && tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && chmod +x trunk && mv trunk /bin

WORKDIR /usr/src/app
COPY . .

EXPOSE 8080

CMD ["trunk", "serve"]
