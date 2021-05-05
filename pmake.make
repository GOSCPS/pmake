
target 盘古{
        print("我")
}

target 父亲 : 盘古{
        print("是")
}

target 母亲 : 父亲 盘古{
        print("你")
}

target 你 : 父亲 母亲{
        println("爹")
}