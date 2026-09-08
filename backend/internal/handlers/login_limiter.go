package handlers

import (
	"sync"
	"time"
)

// LoginLimiter controls attempts for an opaque client/account key.
type LoginLimiter interface {
	Allow(key string) (bool, time.Duration)
	Reset(key string)
}

type loginWindow struct {
	count   int
	expires time.Time
}

// MemoryLoginLimiter is a per-process fixed-window limiter with bounded memory.
type MemoryLoginLimiter struct {
	mu       sync.Mutex
	limit    int
	window   time.Duration
	capacity int
	entries  map[string]loginWindow
	now      func() time.Time
}

func NewMemoryLoginLimiter(limit int, window time.Duration, capacity int) *MemoryLoginLimiter {
	if limit < 1 {
		limit = 1
	}
	if window <= 0 {
		window = time.Minute
	}
	if capacity < 1 {
		capacity = 1
	}
	return &MemoryLoginLimiter{
		limit:    limit,
		window:   window,
		capacity: capacity,
		entries:  make(map[string]loginWindow, capacity),
		now:      time.Now,
	}
}

func (l *MemoryLoginLimiter) Allow(key string) (bool, time.Duration) {
	l.mu.Lock()
	defer l.mu.Unlock()

	now := l.now()
	state, exists := l.entries[key]
	if exists && now.Before(state.expires) {
		if state.count >= l.limit {
			return false, state.expires.Sub(now)
		}
		state.count++
		l.entries[key] = state
		return true, 0
	}

	if exists {
		delete(l.entries, key)
	}
	l.removeExpired(now)
	if len(l.entries) >= l.capacity {
		l.evictClosestToExpiry()
	}
	l.entries[key] = loginWindow{count: 1, expires: now.Add(l.window)}
	return true, 0
}

func (l *MemoryLoginLimiter) Reset(key string) {
	l.mu.Lock()
	delete(l.entries, key)
	l.mu.Unlock()
}

func (l *MemoryLoginLimiter) removeExpired(now time.Time) {
	for key, state := range l.entries {
		if !now.Before(state.expires) {
			delete(l.entries, key)
		}
	}
}

func (l *MemoryLoginLimiter) evictClosestToExpiry() {
	var candidate string
	var candidateExpiry time.Time
	for key, state := range l.entries {
		if candidate == "" || state.expires.Before(candidateExpiry) ||
			(state.expires.Equal(candidateExpiry) && key < candidate) {
			candidate = key
			candidateExpiry = state.expires
		}
	}
	if candidate != "" {
		delete(l.entries, candidate)
	}
}

type allowAllLoginLimiter struct{}

func (allowAllLoginLimiter) Allow(string) (bool, time.Duration) { return true, 0 }
func (allowAllLoginLimiter) Reset(string)                       {}
