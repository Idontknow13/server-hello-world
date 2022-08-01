package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	router := mux.NewRouter()
	router.HandleFunc("/", greet).Methods("GET") // makes `name` query optional
	router.HandleFunc("/", greet).Methods("GET").Queries("name", "{name}")

	log.Fatal(http.ListenAndServe(":8080", router))
}

func greet(writer http.ResponseWriter, req *http.Request) {
	name := req.FormValue("name")
	if name == "" {
		name = "World"
	}
	greeting := fmt.Sprintf("Hello, %s!\n", name)

	writer.WriteHeader(http.StatusOK) // 200
	writer.Write([]byte(greeting))    // body
}
