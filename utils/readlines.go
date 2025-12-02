package utils

import (
	"os"
	"strings"
)

// ReadLines reads the entire file at path and returns its lines as a slice of strings.
func ReadLines(path string) ([]string, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	s := strings.ReplaceAll(string(b), "\r\n", "\n")
	s = strings.TrimSuffix(s, "\n")

	if s == "" {
		return []string{}, nil
	}

	return strings.Split(s, "\n"), nil
}
