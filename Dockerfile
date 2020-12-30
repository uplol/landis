FROM ubuntu:20.04

RUN mkdir -p /app \
    && mkdir -p /app/out \
    && mkdir -p /app/mcserver
WORKDIR /app

COPY palette.tar.gz /app/palette.tar.gz
COPY /web /app/web
COPY /landis /app/landis

RUN chmod +x /app/landis

ENTRYPOINT ["/app/landis"]