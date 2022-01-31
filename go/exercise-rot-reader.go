package main

import (
	"io"
	"os"
	"strings"
)

type rot13Reader struct {
	r io.Reader
}

func (rot rot13Reader) Read(buffer []byte) (int, error) {
	buf := make([]byte, 8)
	n, err := rot.r.Read(buf)
	if err == io.EOF {
		return 0, io.EOF
	}
	for i := 0; i < n; i++ {
		if (buf[i] >= 'A' && buf[i] <= 'M') || (buf[i] >= 'a' && buf[i] <= 'm') {
			buffer[i] = buf[i] + 13
		} else {
			buffer[i] = buf[i] - 13
		}
	}
	return n, nil
}

func main() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := rot13Reader{s}
	io.Copy(os.Stdout, &r)
}
