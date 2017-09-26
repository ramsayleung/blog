FROM ubuntu:16.04

ADD target/release/blog /  
EXPOSE 8000

CMD ["/blog"]  
