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
RUN bun run scripts.ts build

# final image
FROM alpine:3.21.0

# copy build outputs
COPY --from=build /app/build /app/build
COPY --from=build /app/target/release/robo /app/robo

# run the app
CMD /app/robo

# expose app port
EXPOSE 3000
