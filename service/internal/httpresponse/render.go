package httpresponse

import (
	"io"

	"github.com/rs/zerolog/log"
	renderPkg "github.com/unrolled/render"
)

// RenderJSON will render a JSON object to the output stream along with the given status code.
func RenderJSON(w io.Writer, status int, v interface{}) {
	render := renderPkg.New()
	err := render.JSON(w, status, v)

	if err != nil {
		log.Fatal().Err(err).Msg("Failed to render output")
	}
}
