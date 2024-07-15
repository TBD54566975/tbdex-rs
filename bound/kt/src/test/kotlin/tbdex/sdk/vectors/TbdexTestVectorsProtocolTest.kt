package tbdex.sdk.vectors

import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import tbdex.sdk.Json
import tbdex.sdk.messages.*
import tbdex.sdk.resources.*

class TbdexTestVectorsProtocolTest {

    /**
     * Tbdex Test Vectors Message Tests
     */
    @Test
    fun parse_order() {
        testVector("parse-order.json", ::Order) { it.toJsonString() }
    }

    @Test
    fun parse_orderstatus() {
        testVector("parse-orderstatus.json", ::OrderStatus) { it.toJsonString() }
    }

    @Test
    fun parse_rfq() {
        testVector("parse-rfq.json", { input -> Rfq(input, true) }) { it.toJsonString() }
    }

    @Test
    fun parse_rfq_omit_private_data() {
        testVector("parse-rfq-omit-private-data.json", { input -> Rfq(input, false) }) { it.toJsonString() }
    }

    @Test
    fun parse_balance() {
        testVector("parse-balance.json", ::Balance) { it.toJsonString() }
    }

    @Test
    fun parse_offering() {
        testVector("parse-offering.json", ::Offering) { it.toJsonString() }
    }

    @Test
    fun parse_quote() {
        testVector("parse-quote.json", ::Quote) { it.toJsonString() }
    }

    @Test
    fun parse_close() {
        testVector("parse-close.json", ::Close) { it.toJsonString() }
    }

    @Test
    fun parse_cancel() {
        testVector("parse-cancel.json", ::Cancel) { it.toJsonString() }
    }

    private fun <T> testVector(vectorFileName: String, objectCreation: (String) -> T, toJsonString: (T) -> String) {
        val vector = TestVectors.getVector(vectorFileName)
        assertNotNull(vector)

        val input = vector!!.get("input").textValue()
        val obj = objectCreation(input)

        assertEquals(vector["output"], Json.jsonMapper.readTree(toJsonString(obj)))
    }
}