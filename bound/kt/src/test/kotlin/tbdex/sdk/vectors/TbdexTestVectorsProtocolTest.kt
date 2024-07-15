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
        testVector("parse-order.json", ::Order) { it.toJson() }
    }

    @Test
    fun parse_orderstatus() {
        testVector("parse-orderstatus.json", ::OrderStatus) { it.toJson() }
    }

    @Test
    fun parse_rfq() {
        testVector("parse-rfq.json", { input -> Rfq(input, true) }) { it.toJson() }
    }

    @Test
    fun parse_rfq_omit_private_data() {
        testVector("parse-rfq-omit-private-data.json", { input -> Rfq(input, false) }) { it.toJson() }
    }

    @Test
    fun parse_balance() {
        testVector("parse-balance.json", ::Balance) { it.toJson() }
    }

//    @Test
//    fun parse_offering() {
//        testVector("parse-offering.json", ::Offering) { it.toJson() }
//    }
//
//    @Test
//    fun parse_quote() {
//        testVector("parse-quote.json", ::Quote) { it.toJson() }
//    }

    private fun <T> testVector(vectorFileName: String, objectCreation: (String) -> T, toJson: (T) -> String) {
        val vector = TestVectors.getVector(vectorFileName)
        assertNotNull(vector)

        val input = vector!!.get("input").textValue()
        val obj = objectCreation(input)

        assertEquals(vector["output"], Json.jsonMapper.readTree(toJson(obj)))
    }
}