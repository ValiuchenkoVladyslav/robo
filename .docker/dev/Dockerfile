FROM alpine:3.21.0 AS build

RUN apk add --no-cache alpine-sdk curl

# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal --default-toolchain nightly -y
ENV PATH="/root/.cargo/bin:${PATH}"

# copy sources
COPY . /app

WORKDIR /app

# build the app
RUN cargo build --release

# final image
FROM alpine:3.21.0

# copy build output
COPY --from=build /app/target/release/robo /app/robo

# run the app
CMD /app/robo

# expose app port
EXPOSE 3000
