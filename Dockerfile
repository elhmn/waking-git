FROM rust:1.67-slim AS base

WORKDIR /build

RUN apt-get update && apt-get install -y \
	g++ \
	pkg-config \
	libssl-dev

# Create appuser
ENV USER=user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . ./

RUN cargo build -p wake --release

WORKDIR /app

RUN cp /build/target/release/wake .

FROM  debian:bullseye

COPY --from=base /etc/passwd /etc/passwd
COPY --from=base /etc/group /etc/group

WORKDIR /server

COPY --from=base /app/wake .

# set user to `user`
USER user

# expose api port
EXPOSE 3000

CMD ["./wake", "serve", "--port", "3000"]
