FROM rust as builder

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/rustappoint /build-out/

FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build-out/rustappoint /

EXPOSE 3000

CMD /rustappoint
