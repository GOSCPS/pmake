
target default{
        try abort()

        sh "echo" $(sh "echo" "Hello World")
}
drop{
        println("You shouldn't see this!")
}