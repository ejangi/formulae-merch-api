# ForumalaE Merch API

Turns the Formula E *New Merch* webpage into a JSON API.

## Development

I've included a `docker compose` development environment. To run the tests, simply run:

```bash
docker compose build
docker compose up
```

## Production

To build the production docker image, you can run something like:

```bash
docker build -t formulae-merch-api .
```

And test the build:

```bash
docker run -p8080:8080 formulae-merch-api
```