### *`LOGIN`*
```
POST /api/v1/login
Content-Type: application/json
REQUEST

{
    "username":"korirum",
    "password":"admin123",
}

SUCCESS RESPONSE 
{
    "status" : "success",
    "message": "login success",
    "data"   : {
          "access_token": "",
          "token_type"  : "Bearer",
          "expires_in"  : ""
    }
}

BAD REQUEST USER
{
    "status"  : "error",
    "message" : "invalid username and password",
}

INTERNAL SERVER ERROR
{
    "status"  : "error",
    "message" : internal server error, please try agin later."
}  
```

### *`REGISTER`*
```
POST /api/v1/register
Content-Type: application/json
REQUEST

{
    "username" : "nathalie yov",
    "password" : "kiww",
    "email" : ""
}

SUCCESS RESPONSE
{
    "status" : "success",
    "message": "register success",
}

BAD REQUEST USER
{
    "status"  : "error",
    "message" : "username or email already exist",
}

INTERNAL SERVER ERROR
{
    "status"  : "error",
    "message" : "internal server error, please try agin later."
} 
```


### *`LOGOUT`*
```
POST /api/v1/logout
Content-Type: application/json
Authorization: Bearer <your-access-token>
REQUEST

{
    none
}

SUCCESS RESPONSE
{
    "status" : "success",
    "message": "logout success",
}

BAD REQUEST USER, tidak meneyertakan token di header
{
    "status"  : "error",
    "message" : "no token provide",
}

BAD REQUEST USER, token sudah kadaluarsa
{
    "status"  : "error",
    "message" : "invalid token",
}

INTERNAL SERVER ERROR
{
    "status"  : "error",
    "message" : "internal server error, please try agin later."
} 

```
 