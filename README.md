#Rust_Blog
This project is a blog site built with Rust as the backend. The application includes routes for various functionalities, allowing users to interact with the system seamlessly. The available routes are as follows:

Register:<br />

Endpoint: /register<br />
Method: POST<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/register -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"<br />

Login:<br />
Endpoint: /login<br />
Method: POST<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/login -Method POST -Body '{"username": "test_user", "password": "test_password"}' -ContentType "application/json"<br />

Schedule Post:<br />
Endpoint: /schedule_post<br />
Method: POST<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/schedule_post -Method POST -Body '{"title": "Scheduled Post", "content": "Scheduled post content", "publish_date": "2024-03-05T12:00:00Z"}' -ContentType "application/json"<br />

Get Posts:<br />

Endpoint: /posts<br />
Method: GET<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/posts<br />

Create Post:<br />

Endpoint: /posts<br />
Method: POST<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/posts -Method POST -Body '{"title": "New Post", "content": "New post content"}' -ContentType "application/json"<br />

Update Post:<br />

Endpoint: /posts/{post_id} (assuming an existing post at index 0)<br />
Method: PUT<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method PUT -Body '{"title": "Updated Post", "content": "Updated post content"}' -ContentType "application/json"<br />

Delete Post:<br />

Endpoint: /posts/{post_id} (assuming an existing post at index 0)<br />
Method: DELETE<br />
Example Test Case (PowerShell):<br />
Invoke-WebRequest -Uri http://localhost:8000/posts/0 -Method DELETE<br />
Feel free to use these examples to interact with the Rust Blog API. Make sure to replace the placeholder data with actual values when testing.<br />
