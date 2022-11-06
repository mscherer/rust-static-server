#/bin/bash
for i in $(find site/ -name *html) ; do echo $i; gzip $i ; done

[ -f site/atom.xml ] && gzip site/atom.xml

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
