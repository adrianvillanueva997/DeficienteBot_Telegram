package services

import (
	"fmt"
	"net/url"
	"strings"
)

func Is_url(_url string) bool {
	_, err := url.ParseRequestURI(_url)
	if err != nil {
		return false

	}
	return true
}

func update_url(url string) string {
	// converts from twitter.com to vxtwitter.com
	return strings.Replace(url, "twitter", "vxtwitter", 1)
}

func Update_vx_twitter(url string) *string {
	if strings.Contains(url, "twitter") {
		url = update_url(url)
		fmt.Println(url)
		return &url
	}
	return nil
}
