
target default{
        try abort()
        println("You should see this!")
}
drop{
        println("You shouldn't see this!")
}