package services

import (
	"regexp"
)

const number_expr = `[-]?\d[\d,]*[\.]?[\d{2}]*`

func numbersList() []string {
	return []string{"69", "420"}
}
func numberCheck(message string) bool {
	numbers := numbersList()
	re := regexp.MustCompile(number_expr)
	match := false
	for i := 0; i < len(numbers); i++ {
		if re.MatchString(message) {
			match = true
		}

	}
	return match
}

func NumberText(receivedMessage string) *string {
	matches := numberCheck(receivedMessage)
	if matches {
		var message string
		message = "> Nice"
		return &message
	}
	return nil
}
