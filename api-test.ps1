// Route test
curl http://localhost:3000/task

// Route GET linux
curl -X GET http://localhost:3000/api/task/get

//Route GET Windows
irm http://localhost:3000/api/task/get

//Route POST (Login) users
$body = @{
    user = @{
        username = "test"
        email = "test@test"
        password = "test"
        angkatan = 2024
    }
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://localhost:3000/api/users/create" `
    -Method POST `
    -Body $body `
    -ContentType "application/json"