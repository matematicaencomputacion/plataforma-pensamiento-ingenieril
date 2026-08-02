package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		_, err := fmt.Fprint(w, "Hola Mundo")
		if err != nil {
			log.Printf("error writing response: %v", err)
		}
	})

	addr := ":8080"
	log.Printf("servidor escuchando en %s", addr)
	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
