# https://github.com/oven-sh/bun/issues/6666
FROM oven/bun:1.1.8-alpine AS build

RUN apk add --no-cache alpine-sdk curl

# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal --default-toolchain nightly -y
ENV PATH="/root/.cargo/bin:${PATH}"

# copy sources
COPY . /app

WORKDIR /app

# install deps
RUN bun install

# build the app
RUN bun run scripts.ts build docker

# clone maigret
RUN git clone https://github.com/soxoj/maigret --depth 1

# final image
FROM alpine:3.21.0

# copy build output
COPY --from=build /app/build /app
COPY --from=build /app/maigret /app/maigret

# run the app
CMD /app/eagle-web

# expose app port
EXPOSE 8080
