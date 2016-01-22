package main

import (
    "net/http"
    "fmt"
)

//https://github.com/golang/go/wiki/HttpStaticFiles

func main() {
    
    addres := "0.0.0.0:3333";
    
    fmt.Println("server running - " + addres);
    
    panic(http.ListenAndServe(addres, http.FileServer(http.Dir("../../static"))));
}