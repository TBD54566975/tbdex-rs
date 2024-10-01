package api

import spark.Response
import spark.Spark.get
import tbdex.sdk.http.GetOfferingsResponseBody

class Offerings(private val offeringsRepository: data.Offerings) {
    init {
        get("/offerings") { _, res -> getOfferings(res)}
    }

    private fun getOfferings(res: Response): String {
        println("GET /offerings")

        val offerings = offeringsRepository.getOfferings()
        val responseBody = GetOfferingsResponseBody(offerings)

        return responseBody.toJsonString()
    }
}