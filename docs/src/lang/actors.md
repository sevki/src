# Actors

Actors are a pivotal feature of src, providing a powerful concurrency model for managing state and processing messages in a distributed environment. The actor model in src is designed to ensure safe concurrent operations, with support for message queuing (mailboxes) and reentrancy, enabling sophisticated task management.

## Syntax

An actor is defined using the `actor` keyword followed by an identifier and a block of statements enclosed in curly braces. The `state` block specifies the initial state of the actor, while the `receive` block defines the message handlers for processing incoming messages.

```src
actor my_actor {
    state: int, 
}
```