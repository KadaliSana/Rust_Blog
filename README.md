
Rust_Blog
This project is a blog site built with Rust as the backend. The application includes routes for various functionalities, allowing users to interact with the system seamlessly. The available routes are as follows:

Register:

Endpoint: /register
Method: POST
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/register -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"

Login:
Endpoint: /login
Method: POST
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/login -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"

Schedule Post:
Endpoint: /schedule_post
Method: POST
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/schedule_post -Method POST -Body '{"title": "Scheduled Post", "content": "Scheduled post content", "publish_date": "2024-03-05T12:00:00Z"}' -ContentType "application/json"

Get Posts:

Endpoint: /posts
Method: GET
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/posts

Create Post:

Endpoint: /posts
Method: POST
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/posts -Method POST -Body '{"title": "New Post", "content": "New post content"}' -ContentType "application/json"

Update Post:

Endpoint: /posts/{post_id} (assuming an existing post at index 0)
Method: PUT
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method PUT -Body '{"title": "Updated Post", "content": "Updated post content"}' -ContentType "application/json"

Delete Post:

Endpoint: /posts/{post_id} (assuming an existing post at index 0)
Method: DELETE
Example Test Case (PowerShell):
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method DELETE
Feel free to use these examples to interact with the Rust Blog API. Make sure to replace the placeholder data with actual values when testing.
