## How to run manually with Docker

1.  Clone the repository of the project
2.  Navigate to the project's frontend directory (should be `/social-network/frontend`)
3.  Create the image: `docker build -t frontend:sn .`
4.  Create container with the image: `docker container create --name fe-sn_c -p 8080:3000 frontend:sn`
5.  Start or Stop the container: `docker start fe-sn_c` or `docker stop fe-sn_c`
6.  Once the container is running, open the browser at `http://localhost:8080` to enjoy the website

## How to truly run manually

1.  Clone the repository of the project
2.  Navigate to the project's frontend directory (should be `/social-network/frontend`)
3.  Make sure you have NodeJS and NPM installed. Just google how it is done, shouldn't be hard. For this project `node v18.5.0` was used
4.  Install the project dependencies with `npm install`
5.  Compile the website code with `npm run build`
6.  Run the website with `node ./build/index.js`
7.  Open `http://localhost:8080` in the browser and enjoy!

### Used stuff:

-   FONTS:
    -   https://www.cufonfonts.com/font/whitney-2
