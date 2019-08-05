# 文件相关操作

## Seek 性能测试

### 生成测试文件

使用 `main.go` 可以生成测试文件，固定为 `test.txt`。每行包含 `16 Bytes`。

默认生成 1000000000 行，即 16 * 1000000000 = 14.9G，同时支持参数，指定行数。

``` sh
$ go run main.go [1000]
```

### 性能测试

使用 `main.rs` `bench_seek` 函数进行性能测试。需要当前工作目录存在 `test.txt` 文件。

``` sh
# debug
$ ./file
file length 16000000000
seek: 100000 times, elapsed: 194082, speed: 1.94us/seek
```

``` sh
# release
$ ./file
file length 16000000000
seek: 100000 times, elapsed: 75141, speed: 0.75us/seek
```
