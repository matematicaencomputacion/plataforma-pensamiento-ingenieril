package main

import "testing"

func TestListenAddrRespectsPORT(t *testing.T) {
	t.Setenv("PORT", "")
	if got := listenAddr(); got != ":8080" {
		t.Fatalf("default: got %q", got)
	}
	t.Setenv("PORT", "8080")
	if got := listenAddr(); got != ":8080" {
		t.Fatalf("cloud-run style: got %q", got)
	}
	t.Setenv("PORT", ":9090")
	if got := listenAddr(); got != ":9090" {
		t.Fatalf("prefixed: got %q", got)
	}
}
