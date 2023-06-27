## How to `manually` run the backend with Docker

1.  Clone the repository of the project
2.  Navigate to the project's backend directory (should be `/social-network/backend/`)
3.  Create the Docker image: `docker build -t backend:sn .`
4.  Create a container for the image: `docker container create --name be-sn_c backend:sn`
5.  Start or stop the container: `docker start be-sn_c` or `docker stop be-sn_c`

## How to truly manually run the backend

1.  Clone the repository of the project
2.  Navigate to the project's backend directory (should be `/social-network/backend/`)
3.  Make sure you have GoLang installed. Version `1.17` will work best
4.  Download all the dependencies with `go mod download`
5.  Also, download the gcc (C compiler), which is required for SQLite to work. Just google about it
6.  Run the backend with `go run .`
