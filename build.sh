#/bin/bash

TARGET=x86_64-unknown-linux-gnu

for i in $(find public/ -name *html -o -name atom.xml -o -name rss.xml) ; do echo $i; gzip $i ; done

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target $TARGET
strip target/$TARGET/release/static_http
