package main

import (
  "fmt"
  "net/http"
  "regexp"
)

func main() {
  reg := regexp.MustCompile("^/greeting/([a-z]+)$")
  http.ListenAndServe("127.0.0.1:3000", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
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
}
