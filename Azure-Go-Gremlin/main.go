package main

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

    "github.com/joho/godotenv"
	"github.com/rs/zerolog"
	"github.com/supplyon/gremcos"
	"github.com/supplyon/gremcos/api"
	// "github.com/supplyon/gremcos/interfaces"
)

var id = 0

type Container struct {
	cosmos gremcos.Cosmos
	logger zerolog.Logger
}

func main() {
	logger := zerolog.New(os.Stdout).Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: zerolog.TimeFieldFormat}).With().Timestamp().Logger()

    err := godotenv.Load();
    if (err != nil) {
        logger.Fatal().Err(err).Msg("Error loading .env file");
    }

	hostURL := os.Getenv("URL");

	api.SetQueryLanguageTo(api.QueryLanguageTinkerpopGremlin)

    graph := os.Getenv("Graph");
    key := os.Getenv("AzureKey");

	cosmos, err := gremcos.New(hostURL, gremcos.WithAuth(
        graph,
        key,
	), gremcos.WithLogger(logger), gremcos.NumMaxActiveConnections(10), gremcos.ConnectionIdleTimeout(time.Second*1))

	if err != nil {
		logger.Fatal().Err(err).Msg("Failed to create the cosmos connector")
	}

	cleanUp(cosmos, logger)

	exitChannel := make(chan struct{})
	go processLoop(cosmos, logger, exitChannel)

	<-exitChannel
	if err := cosmos.Stop(); err != nil {
		logger.Error().Err(err).Msg("Failed to stop cosmos connector")
	}
	logger.Info().Msg("Teared down")
}

func processLoop(cosmos gremcos.Cosmos, logger zerolog.Logger, exitChannel chan<- struct{}) {
	// register for common exit signals (e.g. ctrl-c)
	signalChannel := make(chan os.Signal, 1)
	signal.Notify(signalChannel, syscall.SIGINT, syscall.SIGTERM)

	// create tickers for doing health check and queries
	queryTicker := time.NewTicker(time.Second * 2)
	healthCheckTicker := time.NewTicker(time.Second * 20)
    container := Container {
        cosmos,
        logger,
    }

	// ensure to clean up as soon as the processLoop has been left
	defer func() {
		queryTicker.Stop()
		healthCheckTicker.Stop()
	}()

	stopProcessing := false
	logger.Info().Msg("Process loop entered")
	for !stopProcessing {
		select {
		case <-signalChannel:
			close(exitChannel)
			stopProcessing = true
		case <-queryTicker.C:
			queryCosmos(cosmos, logger)
            newEvent(container, NewEvent("name", fmt.Sprint(id), time.Now()))
			id += 1
		case <-healthCheckTicker.C:
			err := cosmos.IsHealthy()
			logEvent := logger.Debug()
			if err != nil {
				logEvent = logger.Warn().Err(err)
			}
			logEvent.Bool("healthy", err == nil).Msg("Health Check")
		}
	}

	logger.Info().Msg("Process loop left")
}

func cleanUp(cosmos gremcos.Cosmos, logger zerolog.Logger) {

	g := api.NewGraph("g")

	for i := 0; i < 5; i++ {
		_, err := cosmos.ExecuteQuery(g.V().Limit(10).Drop())

		if err != nil {
			logger.Error().Err(err).Msg("Failed to execute a gremlin command")
			return
		}

	}
}

func newEvent(container Container, event Event) {
	g := api.NewGraph("g")

	query := g.AddV("Event").Property("name", event.name).Property("date", event.date).Property("userId", event.createdBy).AddE("createdBy").To(g.VByStr(event.createdBy))

	_, err := container.cosmos.ExecuteQuery(query)

	if err != nil {
		container.logger.Error().Err(err).Msg("Failed to execute a gremlin command")
		return
	}
}

func queryCosmos(cosmos gremcos.Cosmos, logger zerolog.Logger) {

	// If you want to run your queries against a apache tinkerpop gremlin server it is recommended
	// to switch the used query language to QueryLanguageTinkerpopGremlin.
	// Per default the CosmosDB compatible query language will be used.

	g := api.NewGraph("g")

	query := g.AddV("User").Property("userId", "12345").Property("id", fmt.Sprint(id)).AddE("knows").To(g.VByStr(fmt.Sprint(id - 1))).InV().AddE("also knows").To(g.VByStr(fmt.Sprint(id - 5)))

	//
	// logger.Info().Msgf("Query: %s", query)
	res, err := cosmos.ExecuteQuery(query)

	if err != nil {
		logger.Error().Err(err).Msg("Failed to execute a gremlin command")
		return
	}

	responses := api.ResponseArray(res)
	//
	// // Example for converting the returned response into the different supported types.
	values, err := responses.ToValues()
	if err == nil {
		logger.Info().Msgf("Received Values: %v", values)
	}
	//
	properties, err := responses.ToProperties()
	if err == nil {
		logger.Info().Msgf("Received Properties: %v", properties)
	} else {
		logger.Error().Err(err)
	}

	// fmt.Printf("helllo %v asd", properties);
	//
	vertices, err := responses.ToVertices()
	if err == nil {
		logger.Info().Msgf("Received Vertices: %v", vertices)
	}
	//
	edges, err := responses.ToEdges()
	if err == nil {
		logger.Info().Msgf("Received Edges: %v", edges)
	}
}
