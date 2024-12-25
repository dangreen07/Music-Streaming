FROM node:22.12-alpine

WORKDIR /usr/server/app

COPY ./package.json /
RUN npm install
COPY ./ .

RUN npm run build
ENV NODE_ENV=production
ENV PORT=80
EXPOSE 80
CMD ["npm", "run", "start"]