
target default{
        try abort()

        println("Current total *.rs line:" + $(qexec "sh" "-c" "find src/ -name *.rs |xargs cat|wc -l"))

        if 1
                return
        else
                println("You shouldn't see this!")
}
drop{
        println("You shouldn't see this!")
}