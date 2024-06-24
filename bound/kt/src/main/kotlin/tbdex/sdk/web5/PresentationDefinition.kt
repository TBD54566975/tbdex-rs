package tbdex.sdk.web5

import tbdex.sdk.rust.InputDescriptorData as RustCoreInputDescriptor
import tbdex.sdk.rust.PresentationDefinition as RustCorePresentationDefinition
import tbdex.sdk.rust.PresentationDefinitionData as RustCorePresentationDefinitionData

typealias InputDescriptor = RustCoreInputDescriptor

class PresentationDefinition {
    val id: String
    val name: String?
    val purpose: String?
    val inputDescriptors: List<InputDescriptor>

    val rustCorePresentationDefinition: RustCorePresentationDefinition

    constructor(id: String, name: String? = null, purpose: String? = null, inputDescriptors: List<InputDescriptor>) {
        this.id = id
        this.name = name
        this.purpose = purpose
        this.inputDescriptors = inputDescriptors

        this.rustCorePresentationDefinition = RustCorePresentationDefinition(
            RustCorePresentationDefinitionData(id, name, purpose, inputDescriptors)
        )
    }

    constructor(rustCorePresentationDefinitionData: RustCorePresentationDefinitionData) {
        this.id = rustCorePresentationDefinitionData.id
        this.name = rustCorePresentationDefinitionData.name
        this.purpose = rustCorePresentationDefinitionData.purpose
        this.inputDescriptors = rustCorePresentationDefinitionData.inputDescriptors

        this.rustCorePresentationDefinition = RustCorePresentationDefinition(rustCorePresentationDefinitionData)
    }

    fun selectCredentials(vcJwts: List<String>): List<String> {
        return this.rustCorePresentationDefinition.selectCredentials(vcJwts)
    }
}