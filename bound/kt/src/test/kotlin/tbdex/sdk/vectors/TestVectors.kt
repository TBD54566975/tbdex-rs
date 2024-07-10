package tbdex.sdk.vectors

import com.fasterxml.jackson.databind.JsonNode
import tbdex.sdk.Json
import java.nio.file.Files
import java.nio.file.Paths

object TestVectors {
    val vectors = readVectors()

    fun readVectors(): MutableMap<String, JsonNode> {
        val vectors = mutableMapOf<String, JsonNode>()
        val vectorFiles = arrayOf(
            "parse-close.json",
            "parse-offering.json",
            "parse-order.json",
            "parse-orderstatus.json",
            "parse-quote.json",
            "parse-rfq.json",
            "parse-rfq-omit-private-data.json",
            "parse-balance.json"
        )

        // Print the current working directory
        val currentWorkingDir = Paths.get(".").toAbsolutePath().normalize()
        println("Current working directory for test vectors: $currentWorkingDir")

        // Define the base path
        val basePath = "../../tbdex/hosted/test-vectors/protocol/vectors/"

        for (vectorFile in vectorFiles) {
            val filePath = Paths.get(basePath, vectorFile).normalize()
            val vectorJson = Files.newBufferedReader(filePath).use { it.readText() }
            vectors[vectorFile] = Json.jsonMapper.readTree(vectorJson)
        }

        return vectors
    }

    fun getVector(vectorFile: String): JsonNode? {
        return vectors[vectorFile]
    }
}
