# Rust_Blog
A blogsite using Rust as the backend. The blogsite has routes for the following:
  1)Register
  2)Login
  3)Schedule Post
  4)Reading the posts
  5)Creating the posts
  6)Updating the posts
  7)Deleting the posts

The example test cases for the following routes on Powershell are:

Register:
Invoke-WebRequest -Uri http://localhost:8000/register -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"

Login (assuming a valid user):
Invoke-WebRequest -Uri http://localhost:8000/login -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"

Schedule Post:
Invoke-WebRequest -Uri http://localhost:8000/schedule_post -Method POST -Body '{"title": "Scheduled Post", "content": "Scheduled post content", "publish_date": "2024-03-05T12:00:00Z"}' -ContentType "application/json"

Get Posts:
Invoke-WebRequest -Uri http://localhost:8000/posts

Create Post:
Invoke-WebRequest -Uri http://localhost:8000/posts -Method POST -Body '{"title": "New Post", "content": "New post content"}' -ContentType "application/json"

Update Post (assuming an existing post at index 0):
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method PUT -Body '{"title": "Updated Post", "content": "Updated post content"}' -ContentType "application/json"

Delete Post (assuming an existing post at index 0):
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method DELETE
