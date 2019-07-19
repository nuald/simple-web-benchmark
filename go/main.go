package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"regexp"
)

func main() {
	port := flag.Int("port", 3000, "server port")
	flag.Parse()
	pid := fmt.Sprintf("%d", os.Getpid())
	ioutil.WriteFile(".pid", []byte(pid), 0644)
	fmt.Printf("Master %s is running on port %d\n", pid, *port)
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
