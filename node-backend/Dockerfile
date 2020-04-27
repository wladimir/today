FROM node:13-alpine

RUN apk add g++ make python
RUN yarn global add pm2

WORKDIR /usr/src/app

COPY package.json yarn.lock /opt/rms/feature-flags/

RUN yarn install

COPY .env.template /opt/rms/feature-flags/.env
COPY . /opt/rms/feature-flags/

RUN yarn run build

EXPOSE 8080

CMD [ "yarn", "start" ]
