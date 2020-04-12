FROM node:13-alpine as builder

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install

COPY . ./

RUN yarn build

FROM nginx:1.17.0-alpine

# Copy the react build from Stage 1
COPY --from=builder /app/public /var/www

# Copy our custom ng    inx config
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port 80 to the Docker host, so we can access it
# from the outside.
EXPOSE 80

ENTRYPOINT ["nginx","-g","daemon off;"]
