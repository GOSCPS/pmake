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
| command type | 操作类型 | 要执行的操作的类型 |
| command | 取决于操作类型，见下文 | 执行操作的数据 |

command type有以下取值:
| 值  | 内容 |
|:---:|:----:|
| shell | shell操作 |
| process | 进程操作 |

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