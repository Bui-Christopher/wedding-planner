FROM nginx:latest

COPY nginx.conf /etc/nginx/nginx.conf

# SSL stuff
COPY generate_certs.sh /usr/local/bin/generate_certs.sh
RUN chmod +x /usr/local/bin/generate_certs.sh

RUN apt-get update && apt-get install -y openssl
# Remove package cache and unnecessary files
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

RUN /usr/local/bin/generate_certs.sh

CMD ["nginx", "-g", "daemon off;"]
