package server

import "net/http"

// Inject a request into the HTTP Server and handle the response.
func (s *Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.router.ServeHTTP(w, r)
}
