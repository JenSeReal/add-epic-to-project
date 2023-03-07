FROM rust:1.67 as build

# create a new empty shell project
RUN USER=root cargo new --bin add-epic-to-project
WORKDIR /add-epic-to-project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./schemas ./schemas

# build for release
RUN rm ./target/release/deps/add_epic_to_project*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc AS runtime

# copy the build artifact from the build stage
COPY --from=build /add-epic-to-project/target/release/add-epic-to-project .

# set the startup command to run your binary
ENTRYPOINT ["/add-epic-to-project"]