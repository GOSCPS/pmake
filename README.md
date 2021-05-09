# The Remake Readme
The remake is a build system.

## Use remake
Input \`remake\ -help` to get help.

The remake usually read \`remake.make\` file to build.


## Write make file
The base target define like:
```make
target TARGET_NAME : DEPS1 DEPS2 ...
{
    # Use `#` at the beginning of a line to indicate a comment.
    # Your target body here
}
drop{
    # Drop here
}
```
The depends is anthor target.

The target name must obey that:
 - Start with \`_\` or char::is_alphabetic()
 - Body obey char::.is_alphanumeric() or is \`_\`


The remake will build depends before build the target.Default build the \`default\` target.

If the target build filed.The drop will execute.
The drop is optional.

To execute shell command:
```make
    exec EXPR:ProgramName EXPR:ARG1 EXPR:ARG2 ...
```

Some example:
```make
target default{
        println("Current total *.rs line:" + $(exec "sh" "-c" "find src/ -name *.rs |xargs cat|wc -l"))
}
drop{
        println("You shouldn't see this!")
}
```



