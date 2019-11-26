FROM ekidd/rust-musl-builder:nightly-2019-11-06

# We need to add the source code to the image because `rust-musl-builder`
# assumes a UID of 1000, but TravisCI has switched to 2000.
ADD . ./
RUN sudo chown -R rust:rust .
#RUN sudo apt-get update && sudo apt-get install -y upx-ucl

CMD cargo build --release
