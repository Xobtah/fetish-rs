FROM rtdlib:latest as builder

WORKDIR /opt/fetish

# Copy app
COPY ./ ./

# Env
ENV API_ID=${API_ID}
ENV API_HASH=${API_HASH}

# Install app
RUN cargo build --release


FROM debian:buster-slim

RUN apt update
RUN apt install -y libssl1.1

# Env
ENV API_ID=${API_ID}
ENV API_HASH=${API_HASH}

# Install app
COPY --from=builder /opt/fetish/target/release/fetish /usr/local/bin/fetish
COPY --from=builder /td/tdlib/lib/libtdjson.so.1.7.3 /usr/lib
# Copy conf
COPY --from=builder /opt/fetish/res/telegram-client.docker.toml /opt/fetish/
COPY --from=builder /opt/fetish/res /opt/fetish/

CMD [ "fetish", "/opt/fetish/telegram-client.docker.toml" ]
