#!/bin/bash

# Generate a new private key
openssl genrsa -out /etc/nginx/nginx.key 2048

# Generate a certificate signing request
openssl req -new -key /etc/nginx/nginx.key -out /etc/nginx/nginx.csr \
  -subj "/C=US/ST=CA/L=Los Angeles/O=Company/CN=localhost"

# Generate a self-signed certificate using the private key and CSR
openssl x509 -req -days 365 -in /etc/nginx/nginx.csr -signkey /etc/nginx/nginx.key -out /etc/nginx/nginx.crt

# Clean up the CSR file
rm /etc/nginx/nginx.csr
