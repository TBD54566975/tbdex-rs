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
        testVector(
            "parse-order.json",
            Order.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_orderstatus() {
        testVector(
            "parse-orderstatus.json",
            OrderStatus.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_rfq() {
        testVector("parse-rfq.json", { input ->
            val rfq = Rfq.fromJsonString(input)
            rfq.verifyAllPrivateData()
            rfq
        }, { it.toJsonString() }, { it.verify() })
    }

    @Test
    fun parse_rfq_omit_private_data() {
        testVector("parse-rfq-omit-private-data.json", { input ->
            val rfq = Rfq.fromJsonString(input)
            rfq.verifyPresentPrivateData()
            rfq
        }, { it.toJsonString() }, { it.verify() })
    }

    @Test
    fun parse_balance() {
        testVector(
            "parse-balance.json",
            Balance.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_offering() {
        testVector(
            "parse-offering.json",
            Offering.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_quote() {
        testVector(
            "parse-quote.json",
            Quote.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_close() {
        testVector(
            "parse-close.json",
            Close.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    @Test
    fun parse_cancel() {
        testVector(
            "parse-cancel.json",
            Cancel.Companion::fromJsonString,
            { it.toJsonString() },
            { it.verify() }
        )
    }

    private fun <T> testVector(
        vectorFileName: String,
        objectCreation: (String) -> T,
        toJsonString: (T) -> String,
        verify: (T) -> Unit
    ) {
        val vector = TestVectors.getVector(vectorFileName)
        assertNotNull(vector)

        val input = vector!!.get("input").textValue()
        val obj = objectCreation(input)

        verify(obj)

        assertEquals(vector["output"], Json.jsonMapper.readTree(toJsonString(obj)))
    }
}