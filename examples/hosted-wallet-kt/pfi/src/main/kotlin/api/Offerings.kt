package api

import spark.Response
import spark.Spark.get

class Offerings(private val offeringsRepository: data.Offerings) {
    init {
        get("/offerings") { _, res -> getOfferings(res)}
    }

    private fun getOfferings(res: Response): String {
        val offerings = offeringsRepository.getOfferings()

        res.type("application/json")
        return "{\"data\": [${offerings.joinToString(separator = ",") { it.toJson() }}]}"
    }
}