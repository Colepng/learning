package main

import "time"

type Event struct {
	name      string
	createdBy string
	date      time.Time
}

func NewEvent(name string, createdBy string, date time.Time) Event {
	return Event{
		name,
		createdBy,
		date,
	}
}
