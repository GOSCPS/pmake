# PMake标准二进制格式 版本 - 1
PMake文件使用.pmake作为其文件后缀名称。

以下二进制全部以**大端序**排列。
字符串全部以UTF-8编码排列。

## 定义
下文中的**u64**指代长度为64位的无符号整数。
以此类推，u8即8位的无符号整数。


我们将**长度**为**字符串**定义为:
一个u64指代字符串长度(字节),后随一个UTF-8字符串。

如
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| name | 字符串 | 名称 |

等价于

| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| name length | u64 | 指定了name的长度 |
| name | 等价于name length所代表的字节数 | 指定了当前target的名称 |

## 文件布局

它的布局如下:
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| version | u64 | 决定PMake文件的版本号，此版本为1 |
| os type | u64 | 决定PMake文件适用的操作系统类型 |
| arch type | u64 | 决定PMake文件使用的架构类型 |
| targets | 除了以上数据后的文件所有剩余数据 | 文件的所有targets |

os type有以下取值:
| 值  | 内容 |
|:---:|:----:|
| 0 | unix |
| 1 | linux |
| 2 | windows |
| 3 | macos |
| 4 | others |

arch type有以下取值:
| 值  | 内容 |
|:---:|:----:|
| 0 | x86(x64) |
| 1 | arm |
| 2 | risc-v |
| 3 | others |

target的布局如下:
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| name | 字符串 | 指定了当前target的名称 |
| depends count | u64 | 当前target的依赖的名称的数量 |
| depends | 字符串的数组，数组长度取决于depends count | 依赖 |
| inputs count | u64 | 输入文件的数量 |
| inputs | 字符串的数组，数组长度取决于inputs count | 输入文件 |
| output count | u64 | 输出文件的数量 |
| outputs | 字符串的数组，数组长度取决于inputs count | 输出文件 |
| command count | u64 | 决定有多少操作 |
| commands | 操作的数组，数组长度取决于command count | 执行操作的数据 |

操作(command)的定义:
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| command type | u64 | 操作的类型 |
| command | 不定长 | 操作的数据

command type有以下取值:
| 值  | 内容 |
|:---:|:----:|
| 0 | shell操作 |
| 1 | 进程操作 |

shell操作需要command的结构:
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| shell command | 字符串 | 要执行的shell命令 |
| error | u8 | 如果设置为0，则忽略返回值。如果设置为非0，则在shell返回非0时引发错误。|
| shell error | u8 | 如果设置为0，则无论是否成功启动shell都继续进行。如果设置为非0，则shell启动失败就引发错误。 |

注:error是判断shell返回值的设置。而shell error是判断启动shell的设置。

shell并不一定会启动成功。


process操作需要的command结构:
| 名称 | 长度 | 内容 |
|:---:|:----:|:----:|
| process name | 字符串 | 要启动的程序名称 |
| process args count | u64 | 要启动的进程的参数的数量 |
| process args | 字符串的数组，数组长度取决于process args count | 要启动的进程的参数 |
| error | u8 | 如果设置为0，则忽略返回值。如果设置为非0，则在进程返回非0时引发错误。|
| start error | u8 | 如果设置为0，则无论是否成功启动进程都继续进行。如果设置为非0，则进程启动失败就引发错误。 |

综上。
 
# 一个例子
这个例子的等价makefile
```makefile
editor: main.c text.c
	gcc editor main.c text.c
install:editor
	mv editor /usr/local
```

首先，我们先构建PMake文件的头部:
我们将在x64-linux下构建，所以头应该长这样:
```rust
// 伪代码
// 格式:
// let 变量名称 : 变量类型 = 值
// 将以下变量按定义顺序以 大端序 写入文件即可

// 我们的PMake版本号
let standard : u64 = 1;

// 我们的linux操作系统版本号
let os_type : u64 = 1;

// 我们的x86结构版本号
let arch_type : u64 = 0;
```
之后，我们定义我们的target:
```rust
// 还是按定义的顺序以 大端序 写入文件即可
// 在头之后写入

// 定义editor这个target
let name_length : u64 = "editor".length();
let name : String = "editor";

// 没有依赖，所以定义为0
let depends_count : u64 = 0;

// 定义输入文件
// 有两个输入文件
// PMake会检查这些文件以决定是否进行增量编译
let input_count : u64 = 2;
// !!注意!!
// 写入字符串时要先写入字符串的长度(u64)
// 本篇省略字符串的长度
// 扩展可得
// let input_1 : u64 = "main.c".length();
// let input_1_str : String = "main.c";
// 
// let input_2 : u64 = "text.c".length();
// let input_2_str : String = "text.c";
let inputs : String[] = ["main.c","text.c"];

// 输出文件
// PMake会检查输出文件以决定是否进行增量编译
// 如果输出文件不存在
// 或
// 输入文件晚于输出文件修改
// 则进行增量编译
// 否则什么也不干
let output_count : u64 = 1;

// !字符串长度
let outputs : String[] = ["editor"];

// 我们只需要执行一个操作
let command_count : u64 = 1;

// 定义所执行的操作
// 我们执行shell操作
let command_type : u64 = 0;

// 写入执行的命令
let shell_command_length : u64 = "gcc editor main.c text.c".length();
let shell_command : String = "gcc editor main.c text.c";

// 我们不忽略错误
let error : u8 = 1;
let shell_error : u8 =1;
```
下面是install的定义:
```rust
// 在editor之后写入

// 定义install这个target
let name_length : u64 = "install".length();
let name : String = "install";

// 依赖editor，1
let depends_count : u64 = 1;
let depends : String[] = ["editor"];

// 输入名为editor的文件
let input_count : u64 = 0;
let inputs : String[] = ["editor"];

// 我们无法检查editor的安装路径
// 设置输出为0
// PMake将会永远构建这个target
// 即，忽略增量编译
let output_count : u64 = 0;

// 我们只需要执行一个操作
let command_count : u64 = 1;

// 定义所执行的操作
// 我们执行shell操作
let command_type : u64 = 0;

// 写入执行的命令
let shell_command : String = "mv editor /usr/local";

// 我们不忽略错误
let error : u8 = 1;
let shell_error : u8 =1;
```
这个makefile并不能完美转换到PMake。如无法检查用户是否安装了editor。
但是PMake作为后端，应该提前获知安装路径。所以此问题并不存在真实的场景当中。