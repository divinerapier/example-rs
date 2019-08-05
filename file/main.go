package main

import (
	"bytes"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.OpenFile("./test.txt", os.O_CREATE|os.O_TRUNC|os.O_RDWR, 0666)
	if err != nil {
		panic(err)
	}

	count := 1000000000

	if len(os.Args) > 1 {
		countI64, err := strconv.ParseInt(os.Args[1], 10, 64)
		count = int(countI64)
		if err != nil {
			fmt.Printf("error: invalid count. %v\n", err)
			os.Exit(1)
		}
		if count <= 0 {
			fmt.Printf("error: invalid count. %d\n", count)
			os.Exit(2)
		}
	}

	var buffer bytes.Buffer

	for i := 0; i < count; i++ {
		buffer.WriteString(fmt.Sprintf("%015d\n", i))
		if buffer.Len() > 81920 {
			file.Write(buffer.Bytes())
			buffer.Reset()
		}
	}
	file.Write(buffer.Bytes())
	buffer.Reset()
	file.Sync()
	file.Close()
}
