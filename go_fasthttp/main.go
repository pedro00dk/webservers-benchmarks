package main

import (
	"github.com/valyala/fasthttp"
)

type MyHandler struct{}

var hello = []byte("Hello world!")

func (h *MyHandler) HandleFastHTTP(ctx *fasthttp.RequestCtx) {
	ctx.Write(hello)
}

func main() {
	var handler = &MyHandler{}

	fasthttp.ListenAndServe(
		":8000",
		handler.HandleFastHTTP,
	)
}
