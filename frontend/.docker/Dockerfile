FROM node:18 AS builder

ARG APP_ENVIRONMENT

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build -- --configuration=${APP_ENVIRONMENT}

FROM nginx:alpine
COPY .docker/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/dist/frontend/ /usr/share/nginx/html
