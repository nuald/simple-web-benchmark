package main

import (
  "fmt"
  "net/http"
  "regexp"
)

func main() {
  reg := regexp.MustCompile("^/greeting/([a-z]+)$")
  http.ListenAndServe(":3000", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
    switch r.URL.Path {
      case "/":
        fmt.Fprint(w, "Hello World!")
      default:
        fmt.Fprintf(w, "Hello, %s", reg.FindStringSubmatch(r.URL.Path)[1])
    }
  }))
}
