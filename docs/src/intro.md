<img src="taocp.png" width="200">

#  src
## src Language Design

Domain-Specific Language (DSL): src is envisioned as a DSL tailored for orchestrating distributed systems and handling complex computational workflows, particularly suitable for scenarios like distributed compilation or search operations.

### Syntax and Constructs:

Basic Types: Strings, numbers (integers and floats), and booleans are the fundamental data types.
Identifiers: Used for naming variables, functions, structs, sagas, and actors, adhering to a convention that starts with a letter or underscore, followed by any combination of alphanumeric characters and underscores.

### Structs and Implementations

Structs provide a way to define custom data types with a fixed set of fields, supporting complex data structures necessary for distributed operations.
Separate implementation blocks (impl) associate actions (functions) with structs, emphasizing a clear distinction between data definitions and behavior.

### Actions and Effects

Actions are functions that can perform side effects, with explicit declarations for effects like reads, writes, awaits, throws, network operations, and system calls (traces).
This explicitness in effect declarations aids in understanding the potential side effects and requirements of each action.

### Sagas

Sagas orchestrate complex workflows and are a pivotal feature of src, capable of managing sequences of actions with support for asynchronous operations and automatic compensation for operations within defined namespaces.
An exports block in sagas specifies the successful end state, detailing the resources or side effects produced by the saga's execution.

### Actors

Actors handle concurrency and state encapsulation, processing messages and executing tasks in isolation from other actors to ensure safe concurrent operations.
The actor model in src supports complex concurrency patterns, including message queuing (mailboxes) and reentrancy, for sophisticated task management.

### Namespaces and Automatic Compensation

Namespaces within sagas provide a scoped environment for executing operations, such as setting environment variables and managing resources, with the system automatically handling compensation for actions taken within this scope.
This feature ensures robust error handling and rollback capabilities for operations that may fail or require cleanup.

### Import and Export Mechanisms

src supports importing and exporting functions, structs, and sagas, facilitating code reuse and modularity across different parts of a src application.
This mechanism encourages a clean separation of concerns and the development of reusable components.
