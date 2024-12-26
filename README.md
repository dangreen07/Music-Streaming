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
- Use an s3 bucket to store the music files

## Current Features on the frontend
- Login
- Sign up
- Logout
- Session checking (using cookies)
- Play the audio as a stream
- Stopping playback if the next sample is not loaded

## Future Features
- Add a search feature
- Add a playlist feature
- Add a favorites feature
- Database pooling connections on the backend
- Backend logging of requests
- Splitting the files in the s3 bucket into smaller chunks to be loaded
- Storing information about the songs in the database
- Admin console

## Installation
To install the website, you need to have Docker and Docker Compose installed on your computer.

The easiest way to install the website is to use the `compose.yaml` file in the root directory of the project.

Before installing the website, you need to set up the environment variables in the `.env` file in the root directory of the project.  
Create an environment variable called `DATABASE_URL` and set it to the postgresql database url.  
You also need to set up the environment variables for the s3 bucket.  
You need to put an environment variable called `SERVER_URL` in the `.env` file in the root directory of the project.  
This variable should be set to the url of the server as accessed from the internet.  

Create an environment variable called `DO_ACCESS_KEY_ID` and set it to your access key id.  
Create an environment variable called `DO_SECRET_ACCESS_KEY` and set it to your secret access key.  
Create an environment variable called `DO_REGION` and set it to the region where your s3 bucket is located.  
Create an environment variable called `DO_ENDPOINT` and set it to the endpoint url of your s3 bucket.  
Create an environment variable called `DO_BUCKET_NAME` and set it to the name of your s3 bucket.  

Then, you can install the website by running the following command in the root directory of the project:
```
docker compose up --build -d
```
This command will build the frontend and backend containers and start them in the background.

Once the containers are running, you can access the website by opening the following URL in your browser:
```
http://localhost:3000
```

To add songs to your site you can use the admin console. This admin console is located at the following URL:
```
http://localhost:3000/admin
```
To access the admin console you need to be logged in as an admin with your permissions set to `admin`.  