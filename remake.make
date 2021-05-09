
target default{
        try abort()

        exec "echo" $(exec "echo" "Hello World")
}
drop{
        println("You shouldn't see this!")
}