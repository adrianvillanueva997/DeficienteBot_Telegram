package metrics

import (
	"log"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus/promhttp"
)

func setup_server() *http.Server {
	return &http.Server{
		Addr:              ":2112",
		Handler:           nil,
		ReadTimeout:       5 * time.Second,
		ReadHeaderTimeout: 0,
		WriteTimeout:      10 * time.Second,
		IdleTimeout:       0,
		MaxHeaderBytes:    0,
	}

}

func Metrics() {
	server := setup_server()
	http.Handle("/metrics", promhttp.Handler())
	err := server.ListenAndServe()
	if err != nil {
		log.Panicln(err)
	}
}
