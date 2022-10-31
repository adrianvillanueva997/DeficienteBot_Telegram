package services

import (
	"regexp"

	"golang.org/x/exp/slices"
)

const number_expr = `[-]?\d[\d,]*[\.]?[\d{2}]*`

func numbersList() []string {
	return []string{"69", "420"}
}
func numberCheck(message string) bool {
	numbers := numbersList()
	re := regexp.MustCompile(number_expr)
	match := false
	matches := (re.FindAllString(message, -1))
	if len(matches) != 0 {
		i := 0
		for i < len(numbers) && match == false {
			for j := 0; j < len(matches); j++ {
				if slices.Contains(matches, numbers[i]) {
					match = true
				}
			}
			i++
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
