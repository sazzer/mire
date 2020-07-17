package internal

import (
	"net/http"
	"net/http/httptest"
)

// Inject will inject an HTTP Request into the service and return the response.
func (s *Service) Inject(req *http.Request) *httptest.ResponseRecorder {
	rr := httptest.NewRecorder()

	s.server.ServeHTTP(rr, req)

	return rr
}
