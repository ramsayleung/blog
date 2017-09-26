FROM scratch

ADD target/release/blog /  
EXPOSE 8000

CMD ["/blog"]  
