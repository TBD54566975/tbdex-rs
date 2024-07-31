import spark.Spark.port
import java.io.File
import java.util.Properties
import web5.sdk.dids.BearerDid
import web5.sdk.dids.PortableDid

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

    api.Exchanges(bearerDid, offeringsRepository)

    println("PFI server running...")
}
