package services

import (
	"io"
	"net/http"
	"os"
	"os/exec"
	"strings"

	log "github.com/sirupsen/logrus"
)

func CheckWebm(url string) bool {
	return strings.Contains(url, ".webm")
}

func Downloadwebm(url string) {
	println("Downloading webm")
	out, err := os.Create("output.webm")
	if err != nil {
		log.Error(err.Error())
	}
	defer out.Close()
	resp, err := http.Get(url)

	if resp.StatusCode != http.StatusOK {
		return
	}
	defer resp.Body.Close()

	_, err = io.Copy(out, resp.Body)
	if err != nil {
		log.Error(err.Error())
	}
}

func ConvertWebMToMP4(inputFile string, outputFile string) error {
	// Run FFmpeg command to convert WebM to MP4
	cmd := exec.Command("ffmpeg", "-i", inputFile, outputFile)
	err := cmd.Run()
	if err != nil {
		return err
	}

	return nil
}
