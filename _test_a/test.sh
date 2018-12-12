GOPATH="$(pwd)/.go"
${GOPATH}/bin/bombardier -c 125 -n 1000000 127.0.0.1:8000
