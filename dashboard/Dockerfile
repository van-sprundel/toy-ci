FROM node:16-alpine as install
WORKDIR /app

COPY package*.json ./
RUN npm install

FROM node:16-alpine as build
WORKDIR /app

COPY . .
COPY --from=install /app/node_modules ./node_modules

CMD ["npm", "run", "dev"]
