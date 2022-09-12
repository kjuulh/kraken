FROM golang 

COPY . .

RUN go build cmd/server/server.go

CMD [ "server", "start" ]
