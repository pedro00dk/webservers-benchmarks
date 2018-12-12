package main

import (
	"net/http"
)

var hello = []byte("Hello world!")

func main() {

	http.HandleFunc(
		"/",
		func(w http.ResponseWriter, r *http.Request) {
			w.Write(hello)
		},
	)
	http.ListenAndServe(":8000", nil)
}
