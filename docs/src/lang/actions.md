# Actions

Actions are the fundamental building blocks of src, representing functions that can perform side effects. They are explicitly declared with a signature that includes the potential side effects, aiding in understanding the requirements and potential impact of each action.

## Syntax

```ebnf
action := "action" ident "(" [param_list] ")" ["[" effects "]"] "->" type "{" ... "}"
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

