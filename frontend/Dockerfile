FROM node:22.12-alpine AS builder

WORKDIR /usr/server/app

COPY ./package.json /
RUN npm install
COPY ./ .

ENV NODE_ENV=production
ENV PORT=3000
RUN npm run build

EXPOSE 3000
CMD ["npm", "run", "start"]