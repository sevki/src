# Effects

Actions in src can perform side effects, such as reading from or writing to a file, making a network request, or throwing an error. The src language provides explicit declarations for these effects, aiding in understanding the potential side effects and requirements of each action.

## Syntax

```ebnf
effect_decl := "[" effect_list "]"
effect_list := effect { "," effect }
effect := "read" | "write" | "await" | "throw" | "network" | "trace"
```

## Example

```src
action read_file(path: string) [throws, open] -> string {
    // ...
} compensate {
    when {
        throw(e) => {
            // close fid
        },
    }
} 
```