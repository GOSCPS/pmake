
target default : a b c d e{
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

target a sleep(1000000000);
target b sleep(1000000000);
target c sleep(1000000000);
target d sleep(1000000000);
target e sleep(1000000000);