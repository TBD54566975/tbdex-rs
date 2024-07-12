import spark.Spark.port
import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
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

    val offeringsRepository = data.Offerings(bearerDid)
    api.Offerings(offeringsRepository)

    val exchangesRepository = data.Exchanges()
    api.Exchanges(bearerDid, offeringsRepository, exchangesRepository)

    println("PFI server running...")
}
