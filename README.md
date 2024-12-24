## Music Streaming Website
This website is a music streaming website that allows users to stream music from their computers. It uses React and TypeScript for the frontend and Rust for the backend.

Currently, the website is in development and is not yet functional.
The rust backend, currently, sends the entire file to the frontend instead of streamign it.

## Current Features on the backend
- User registration
- User login
- Session invalidation (logout)
- User session management
- Output the chunks of the audio file
- Get the current song info

## Current Features on the frontend
- Login
- Sign up
- Logout
- Play the audio as a stream
- Session checking (using cookies)

## Future Features
- Use an s3 bucket to store the music files
- Add a search feature
- Add a playlist feature
- Add a favorites feature
- Database pooling connections on the backend
- Backend logging of requests

## Installation
To install the website, you need to have Docker and Docker Compose installed on your computer.

The easiest way to install the website is to use the `compose.yaml` file in the root directory of the project.

Before installing the website, you need to set up the environment variables in the `.env` file in the root directory of the project.
Create an environment variable called `DATABASE_URL` and set it to the postgresql database url.
Then, you can install the website by running the following command in the root directory of the project:
```
docker compose up --build -d
```
This command will build the frontend and backend containers and start them in the background.

Once the containers are running, you can access the website by opening the following URL in your browser:
```
http://localhost:3000
```
