package tbdex.sdk

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.ObjectWriter
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.module.kotlin.registerKotlinModule

const val dateTimeFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSXXX"

/**
 * A singleton for json serialization/deserialization, shared across the SDK as ObjectMapper instantiation
 * is an expensive operation.
 * - Serialize ([stringify])
 * - Deserialize ([parse])
 *
 * ### Example Usage:
 * ```kotlin
 * val offering = Json.objectMapper.readValue<Offering>(payload)
 *
 * val jsonString = Json.stringify(myObject)
 *
 * val node = Json.parse(payload)
 * ```
 */
internal object Json {
    /**
     * The Jackson object mapper instance, shared across the lib.
     *
     * It must be public in order for typed parsing to work as we cannot use reified types for Java interop.
     */
    val jsonMapper: ObjectMapper = ObjectMapper()
        .registerKotlinModule()
        .findAndRegisterModules()
        .setSerializationInclusion(JsonInclude.Include.NON_NULL)
        .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)

    private val objectWriter: ObjectWriter = jsonMapper.writer()

    /**
     * Converts a kotlin object to a json string.
     *
     * @param obj The object to stringify.
     * @return json string.
     */
    fun stringify(obj: Any): String {
        return objectWriter.writeValueAsString(obj)
    }
}