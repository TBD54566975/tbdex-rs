package com.example

import spark.Spark.port
import tbdex.sdk.web5.*
import java.io.File
import java.util.Properties

fun readEnv() {
    val envFile = File(Thread.currentThread().contextClassLoader.getResource(".env")?.file ?: throw Error("no .env file found"))
    val properties = Properties().apply {
        envFile.inputStream().use { load(it) }
    }
    properties.forEach { (key, value) ->
        System.setProperty(key.toString(), value.toString())
    }
}

fun main() {
    readEnv()

    val bearerDid = BearerDid(PortableDid(System.getProperty("PFI_PORTABLE_DID")))

    port(8082)

    val offeringsApi = OfferingsApi(bearerDid)
    offeringsApi.setupGetOfferings()

    val exchangesApi = ExchangesApi(bearerDid, offeringsApi)
    exchangesApi.setupCreateExchange()
    exchangesApi.setupSubmitOrder()
}
