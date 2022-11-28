#/bin/bash
for i in $(find site/ -name *html -o -name atom.xml -o -name rss.xml) ; do echo $i; gzip $i ; done

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/static_http
