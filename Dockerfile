# Build dist/ folder.
FROM rust:slim-bookworm as builder

RUN rustup target add wasm32-unknown-unknown

ADD https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz ./tmp
RUN cd /tmp && tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && chmod +x trunk && mv trunk /bin

WORKDIR /usr/src/app
COPY . .

RUN trunk build --release

# Use slimmer nginx image for server.
FROM nginx:latest

COPY --from=builder /usr/src/app/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/app/dist /usr/share/nginx/www

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]
