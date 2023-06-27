# Social Network

---

## How to run the project with Docker Compose

1.  First make sure you have Docker and Docker Compose installed on your computer. Best way to do so is to google up a guide that is specific for your operating system
2.  Clone the repository with `git clone https://01.kood.tech/git/Kanguste/social-network.git`
3.  Make sure you are in the project's directory (should be `/social-network`). If not, use the command `cd social-network`
4.  Use the command `docker compose up -d` to start up the Social Network!
5.  Go to the URL `localhost:8080` in the browser and enjoy the website
6.  If you want to close the website, run `docker compose down`

### Additional Docker commands

8.  For advanced closing, to also clean up all images, use `docker compose down --rmi all -v` instead. To clean up the building cache, use `docker system prune -a` and enter the `y` option
9.  Check out the provided options in the `.env` file. NB! After changing those options, make sure the images are regenerated (essentually just run the commands in step `8` to clear everything up, follwed by the command in step `4` to start the composer again)

---
