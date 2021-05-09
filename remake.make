
target default{
        try abort()

        println("Current total *.rs line:" + $(exec "sh" "-c" "find src/ -name *.rs |xargs cat|wc -l"))
}
drop{
        println("You shouldn't see this!")
}