FROM rust:latest AS build

WORKDIR /app

RUN cargo install trunk wasm-bindgen-cli

COPY . .

RUN cd showcase && trunk build --release

FROM nginx:alpine

COPY --from=build /app/showcase/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
