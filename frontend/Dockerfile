FROM node:22.12-alpine

WORKDIR /usr/server/app

COPY ./package.json /
RUN npm install
COPY ./ .

RUN npm run build
ENV NODE_ENV=production
ENV PORT=3000
EXPOSE 3000
CMD ["npm", "run", "start"]