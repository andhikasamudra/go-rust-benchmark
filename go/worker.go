package main

import (
	"log"
	"sync"
	"time"

	"github.com/gofiber/fiber/v2"
)

const workerCount = 10 // Number of workers in the pool

var taskQueue = make(chan int, 100) // Buffered channel for requests

func worker(wg *sync.WaitGroup, id int) {
	defer wg.Done()
	for job := range taskQueue {
		// Simulate heavy computation
		time.Sleep(50 * time.Millisecond)
		log.Printf("Worker %d processed task %d\n", id, job)
	}
}

func main() {
	app := fiber.New(fiber.Config{
		Prefork: true, // Enable process forking for performance
	})

	// Start worker pool
	var wg sync.WaitGroup
	for i := 0; i < workerCount; i++ {
		wg.Add(1)
		go worker(&wg, i)
	}

	app.Get("/process", func(c *fiber.Ctx) error {
		select {
		case taskQueue <- time.Now().Nanosecond():
			return c.JSON(fiber.Map{"status": "queued"})
		default:
			return c.Status(503).JSON(fiber.Map{"error": "Queue full"})
		}
	})

	go func() {
		if err := app.Listen(":8080"); err != nil {
			log.Fatal(err)
		}
	}()

	// Wait for workers to complete on shutdown
	wg.Wait()
}
