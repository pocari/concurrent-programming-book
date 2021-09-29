package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Semaphore struct {
	locks chan struct{}
}

func newSemaphore(size uint) *Semaphore {
	return &Semaphore{
		locks: make(chan struct{}, size),
	}
}

func (s *Semaphore) wait() {
	s.locks <- struct{}{}
}

func (s *Semaphore) post() {
	<-s.locks
}

func main() {
	var counter int64
	counter = 0
	s := newSemaphore(5)

	wg := new(sync.WaitGroup)
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func(i int) {
			defer s.post()
			defer atomic.AddInt64(&counter, -1)
			defer wg.Done()
			s.wait()

			atomic.AddInt64(&counter, 1)
			if counter >= 5 {
				panic("too main concurrent goroutine")
			}

			fmt.Printf("id: %d, counter: %d\n", i, counter)
			time.Sleep(500 * time.Millisecond)

		}(i)
	}

	wg.Wait()
}
