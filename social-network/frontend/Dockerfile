# FIRST STAGE
FROM node:18.5.0-alpine3.16

WORKDIR /app

# Dependencies installation
COPY package.json .
COPY package-lock.json .
RUN npm install

# Source files and building
COPY . ./
RUN npm run build

CMD [ "node", "./server/index.js" ]
