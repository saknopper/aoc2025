package utils

import (
	"os"
	"strings"
)

// ReadLines reads the entire file at path and returns its lines as a slice of strings.
// It uses os.ReadFile and splits on '\n'. It normalizes CRLF to LF and drops a
// trailing empty line if the file ends with a newline.
func ReadLines(path string) ([]string, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	s := string(b)
	// normalize CRLF
	if strings.Contains(s, "\r\n") {
		s = strings.ReplaceAll(s, "\r\n", "\n")
	}
	lines := strings.Split(s, "\n")
	// drop trailing empty line caused by a final newline
	if len(lines) > 0 && lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-1]
	}
	return lines, nil
}
