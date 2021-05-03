
# 输出构建信息
sh echo "Build at $$pwd"

if !isDef("cc"){
    glodef cc = "clang"
}

rule compileC : cc source flags output{
    sh $cc $source $flags $output
}

target name : deps1 deps2 deps3{
    def source = "main.c"
    def flasgs = "-O3"
    def output = "main"

    source compileC
}





