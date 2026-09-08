package handlers

import (
	"fmt"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"
)

func TestClientIdentityTrustPolicy(t *testing.T) {
	req := httptest.NewRequest("POST", "/api/auth/login", nil)
	req.RemoteAddr = "10.0.0.7:4321"
	req.Header.Set("X-Forwarded-For", "198.51.100.9, 203.0.113.5")

	if got := clientIdentity(req, false); got != "10.0.0.7" {
		t.Fatalf("untrusted identity = %q", got)
	}
	if got := clientIdentity(req, true); got != "198.51.100.9" {
		t.Fatalf("trusted identity = %q", got)
	}

	req.Header.Set("X-Forwarded-For", "invalid, 2001:db8::5")
	if got := clientIdentity(req, true); got != "2001:db8::5" {
		t.Fatalf("malformed fallback identity = %q", got)
	}
}

func TestLoginAttemptKeyNormalizesEmailAndHidesIt(t *testing.T) {
	req := httptest.NewRequest("POST", "/api/auth/login", nil)
	req.RemoteAddr = "192.0.2.10:1234"
	a := loginAttemptKey(req, " User@Example.com ", false)
	b := loginAttemptKey(req, "user@example.com", false)
	if a != b {
		t.Fatalf("normalized keys differ: %q != %q", a, b)
	}
	if strings.Contains(a, "user@example.com") {
		t.Fatalf("raw email leaked into limiter key: %q", a)
	}
}

func TestRetryAfterSecondsRoundsUp(t *testing.T) {
	for duration, want := range map[time.Duration]int{
		0: 1, time.Nanosecond: 1, time.Second: 1, time.Second + time.Nanosecond: 2,
	} {
		if got := retryAfterSeconds(duration); got != want {
			t.Fatalf("retryAfterSeconds(%s) = %d, want %d", duration, got, want)
		}
	}
}

func TestMemoryLoginLimiterAllowsBlocksExpiresAndResets(t *testing.T) {
	now := time.Date(2026, 9, 8, 12, 0, 0, 0, time.UTC)
	limiter := NewMemoryLoginLimiter(2, time.Minute, 10)
	limiter.now = func() time.Time { return now }

	if ok, _ := limiter.Allow("client"); !ok {
		t.Fatal("first attempt blocked")
	}
	if ok, _ := limiter.Allow("client"); !ok {
		t.Fatal("second attempt blocked")
	}
	if ok, retry := limiter.Allow("client"); ok || retry != time.Minute {
		t.Fatalf("third attempt = (%v, %v), want blocked for one minute", ok, retry)
	}

	limiter.Reset("client")
	if ok, _ := limiter.Allow("client"); !ok {
		t.Fatal("attempt after reset blocked")
	}

	now = now.Add(time.Minute)
	if ok, _ := limiter.Allow("client"); !ok {
		t.Fatal("attempt after expiry blocked")
	}
}

func TestMemoryLoginLimiterNeverExceedsCapacity(t *testing.T) {
	limiter := NewMemoryLoginLimiter(2, time.Hour, 3)
	for i := 0; i < 20; i++ {
		limiter.Allow(fmt.Sprintf("client-%02d", i))
		if got := len(limiter.entries); got > 3 {
			t.Fatalf("entries = %d, capacity = 3", got)
		}
	}
}

func TestMemoryLoginLimiterIsConcurrencySafe(t *testing.T) {
	limiter := NewMemoryLoginLimiter(100, time.Minute, 10)
	var wg sync.WaitGroup
	for i := 0; i < 200; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			limiter.Allow("shared")
		}()
	}
	wg.Wait()

	limiter.mu.Lock()
	count := limiter.entries["shared"].count
	limiter.mu.Unlock()
	if count != 100 {
		t.Fatalf("recorded count = %d, want 100", count)
	}
}
