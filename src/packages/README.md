# Server package organisation

Packages and resources in this server will follow the organisation laid out in the official IEEE 2030.5 specification Annex B. The original protocol relied heavily on inheritance for resource organisation - a feature Rust does not have. In trying to keep with the protocol as closely as possible, the following schema has been chosen for this implemention: (the resources in section B.2.3.2/Figure B.2 will be used in all the examples below)

## Resource Organisation

### Overview
(the diagram "identificationOrganisation.png" is useful to visualise the below organisation, as it represents the `RespondableSubscribableIdentifiedObject` as a UML)

- Structs will be used to store class data (fields). 
- Every class in the spec will have a struct containing the data of that class only that are not inherited from the parent class (for example, the `IdentifiedObject` class contains 3 additional fields `description`, `mRID`, and `version`. These fields will be stored in `struct IdentifiedData`).   
- Classes in the spec that are extended will have traits of the same name, and will describe the methods that relate to that class (for example, the `Resource` class in the spec is extended by several other classes, so it's methods will be defined under `trait ResourceTrait`).
- If in the future a class is extended, the methods of that class will be extracted and encapsulated in a trait of the same name with the "Trait" suffix. This trait will then be implemented by that class and it's new subclasses.
- Sub-classes in the spec will contain the additional fields they require and an instance of the classes they immediately extend. They will also implement the trait associated with the super class. This is to enforce the standard's requirement that sub-classes contain all the data of the super-classes they extend.
    - Example 1: `class SubscribableResource` has 1 additional field. This can be taken out and placed in a new `struct SubscribableData`, and the behaviour of `SubscribableResource` can be placed in it’s own `trait Subscribable`.  Then `struct SubscribableResource`  can then contain an instance of the struct `SubscribableData` & `ResourceData`, as well as implement the traits `SubscribableTrait` and `ResourceTrait`)
    - Example 2: The `Event` class extends the `RespondableSubscribableIdentifiedObject`, which further extends several other classes. To enforce an `Event` instance to contain all the fields in it’s super class, the `struct Event` must contain an instance of a `RespondableSubscribableIdentifiedObject` as a field. Then, to have `Event` implement all the traits that `RespondableSubscribableIdentifiedObject` implements, we call the trait methods on the `RespondableSubscribableIdentifiedObject` field inside `struct Event`.

### Justification and improvements

This method of organisation has been chosen so the server may be in keeping with the protocols requirement that sub-classes contain the data of their super-classes. However, if this requirement is relaxed, another implementer may simply have resources encapsulate the individual `Data` ending structs with the data they require. 

A fault with this organisation is the high degree of repetition when implementing multiple traits. The `EndDeviceControl` struct for example would have to implement the traits `Randomizable`, `Event`, `Subscribable`, `Respondable`, and `Resource`, whilst containing an instance of the `RandomizableEvent` struct that would also implement all those traits. It is likely that to implement these traits for `EndDeviceControl`, the same methods are simply run on the encapsulated `RandomizableEvent` struct. Without optimisations, this may lead to some incredibly ugly boilerplate, considering that the protocol is very inheritance heavy, and the flexibility of different function implementations is not required. It may be possible at a later date to implement the `derive` macro, so these traits may be implemented immediately if the struct contains only 1 struct that also implements these same traits.


