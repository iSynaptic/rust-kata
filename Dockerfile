FROM debian:jessie-slim
MAINTAINER Jordan Terrell <jterrell@wans.net>

ADD target/release/rustkata /app/rustkata
ADD sample_input/*.* /app/sample_input/
RUN chmod +x /app/rustkata

WORKDIR /app

CMD ["./rustkata"]