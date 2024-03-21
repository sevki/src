# Sagas

Sagas are a pivotal feature of src, capable of managing sequences of actions with support for asynchronous operations and automatic compensation for operations within defined namespaces. Sagas are used to orchestrate complex workflows, ensuring that operations are executed in a consistent and reliable manner, even in the presence of failures.

## Syntax

A saga is defined using the `saga` keyword followed by an identifier and a block of statements enclosed in curly braces. The `exports` block specifies the successful end state, detailing the resources or side effects produced by the saga's execution.

```src
saga my_saga {
    // Saga statements
} exports {
    // Export definitions
}
```

## Example

The following example demonstrates a simple saga that orchestrates a sequence of actions to perform a distributed search operation. The saga is responsible for coordinating the search across multiple nodes and handling the results.

```src
saga distributed_search {
    let results = search_nodes("query")
    if results.empty() {
        throw("No results found")
    }
} exports {
    results
}
```