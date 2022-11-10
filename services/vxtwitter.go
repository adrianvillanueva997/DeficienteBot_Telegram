package services

import (
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
	if strings.Contains(url, "vxtwitter") {
		return nil
	}
	if strings.Contains(url, "twitter") && strings.Contains(url, "status") {
		url = update_url(url)
		return &url
	}
	return nil
}
