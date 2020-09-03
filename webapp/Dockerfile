# Build the app
FROM node:14.4.0-alpine3.11 AS builder

WORKDIR /mire

COPY package.json yarn.lock tsconfig.json /mire/

RUN yarn install

COPY public /mire/public/
COPY src /mire/src/

RUN yarn build

# Create deployable image
FROM beamaustralia/react-env:latest

WORKDIR /var/www

COPY --from=builder /mire/build /var/www

ENV REACT_APP_URL_BASE=NOT_SPECIFIED
