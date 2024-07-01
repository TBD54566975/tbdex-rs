package com.example

import spark.Spark.get
import spark.Spark.post
import spark.Spark.port

fun main() {
    port(8082)

    get("/hello-world") { _, res ->
        res.type("text/html")
        "{\"hello\": \"world\"}"
    }
}
