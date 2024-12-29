FROM node:22.12-alpine AS builder

WORKDIR /app

COPY ./package.json ./package-lock.json /
RUN npm install
COPY ./ .

ENV NODE_ENV=production
ENV PORT=3000
RUN npm run build
RUN npm prune

FROM node:22.12-alpine AS release

WORKDIR /app

COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/build ./build
COPY --from=builder /app/public ./public
ADD . .

EXPOSE 3000
CMD ["npm", "run", "start"]