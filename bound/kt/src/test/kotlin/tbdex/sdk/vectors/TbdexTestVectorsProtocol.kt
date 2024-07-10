package tbdex.sdk.vectors

import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import tbdex.sdk.Json
import tbdex.sdk.messages.*
import tbdex.sdk.resources.*

class TbdexTestVectorsProtocol {
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

// TODO: Fix offering test vector - https://github.com/TBD54566975/tbdex/issues/346
//    @Test
//    fun parse_offering() {
//        val vector = TestVectors.getVector("parse-offering.json")
//        assertNotNull(vector)
//        val input = vector!!.get("input").textValue()
//        val offering = Offering(input)
//        assertEquals(vector["output"], Json.jsonMapper.readTree(offering.toJson()))
//    }

// TODO: Fix quote test vector - https://github.com/TBD54566975/tbdex/issues/345
//    @Test
//    fun parse_quote() {
//        val vector = TestVectors.getVector("parse-quote.json")
//        assertNotNull(vector)
//        val input = vector!!.get("input").textValue()
//        val quote = Quote(input)
//        assertEquals(vector["output"], Json.jsonMapper.readTree(quote.toJson()))
//    }

    private fun <T> testVector(vectorFileName: String, objectCreation: (String) -> T, toJson: (T) -> String) {
        val vector = TestVectors.getVector(vectorFileName)
        assertNotNull(vector)

        val input = vector!!.get("input").textValue()
        val obj = objectCreation(input)

        assertEquals(vector["output"], Json.jsonMapper.readTree(toJson(obj)))
    }
}