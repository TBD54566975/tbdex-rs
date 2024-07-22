import tbdex.sdk.web5.BearerDid
import tbdex.sdk.web5.PortableDid
import java.io.File
import java.util.Properties

enum class ExchangeFlowType {
    HAPPY_PATH_WEBHOOK_FLOW,
    HAPPY_PATH_POLLING_FLOW,
    CANCEL_FLOW,
    ERROR_FLOW,
    ALL_FLOWS
}

// Set the desired flow type here
val FLOW_TYPE = ExchangeFlowType.ERROR_FLOW

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

    val pfiDidUri = System.getProperty("PFI_DID_URI")
    val verifiableCredential = System.getProperty("HOSTED_WALLET_VERIFIABLE_CREDENTIAL")
    val bearerDid = BearerDid(PortableDid(System.getProperty("HOSTED_WALLET_PORTABLE_DID_JSON")))
    val replyToUrl = System.getProperty("REPLY_TO_URL")

    when (FLOW_TYPE) {
        ExchangeFlowType.HAPPY_PATH_WEBHOOK_FLOW -> runHappyPathFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl)
        ExchangeFlowType.HAPPY_PATH_POLLING_FLOW -> runHappyPathPollingFlow(pfiDidUri, verifiableCredential, bearerDid)
        ExchangeFlowType.CANCEL_FLOW -> runCancelFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl)
        ExchangeFlowType.ERROR_FLOW -> runErrorFlow(pfiDidUri, verifiableCredential, bearerDid)
        ExchangeFlowType.ALL_FLOWS -> {
            runHappyPathFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl)
            runHappyPathPollingFlow(pfiDidUri, verifiableCredential, bearerDid)
            runCancelFlow(pfiDidUri, verifiableCredential, bearerDid, replyToUrl)
            runErrorFlow(pfiDidUri, verifiableCredential, bearerDid)

            val allExchanges = tbdex.sdk.httpclient.getExchanges(pfiDidUri, bearerDid)
            println("All Exchanges Completed: $allExchanges")
        }
    }
}
