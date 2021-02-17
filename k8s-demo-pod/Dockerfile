FROM node:12.19.0-alpine

WORKDIR /app

COPY node_modules node_modules
COPY app.js .
COPY public public

EXPOSE 3001

CMD ["node", "app.js"]
