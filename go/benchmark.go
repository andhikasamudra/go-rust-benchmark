package main

import (
	"bytes"
	"fmt"
	"image"
	"image/jpeg"
	"net/http"
	"time"

	_ "image/png"
)

func compressImage(img image.Image) ([]byte, error) {
	var buf bytes.Buffer
	if err := jpeg.Encode(&buf, img, &jpeg.Options{Quality: 50}); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func handler(w http.ResponseWriter, r *http.Request) {
	file, _, err := r.FormFile("image")
	if err != nil {
		http.Error(w, "Failed to read image", http.StatusBadRequest)
		return
	}
	defer file.Close()

	img, _, err := image.Decode(file)
	if err != nil {
		http.Error(w, "Failed to decode image", http.StatusBadRequest)
		return
	}

	start := time.Now()
	compressed, err := compressImage(img)
	if err != nil {
		http.Error(w, "Failed to compress image", http.StatusInternalServerError)
		return
	}
	duration := time.Since(start)
	fmt.Printf("Compression took: %s\n", duration)

	w.Header().Set("Content-Type", "image/jpeg")
	w.Write(compressed)
}

func main() {
	http.HandleFunc("/compress", handler)
	fmt.Println("Server started at :8080")
	http.ListenAndServe(":8080", nil)
}
