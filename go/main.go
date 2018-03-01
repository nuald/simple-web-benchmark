package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"regexp"
)

func main() {
	port := flag.Int("port", 3000, "server port")
	flag.Parse()
	fmt.Printf("Master %d is running on port %d\n", os.Getpid(), *port)
	reg := regexp.MustCompile("^/greeting/([a-z]+)$")
	host := fmt.Sprintf("127.0.0.1:%d", *port)
	err := http.ListenAndServe(host, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/":
			fmt.Fprint(w, "Hello World!")
		default:
			matches := reg.FindStringSubmatch(r.URL.Path)
			if matches != nil {
				fmt.Fprintf(w, "Hello, %s", matches[1])
			} else {
				http.NotFound(w, r)
			}
		}
	}))
	if err != nil {
		log.Fatal(err)
	}
}
