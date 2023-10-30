# Axum Simple Crud

Goal of this project was to write REST service, which will support basic CRUD operations for simple data structures with few fields. Required operations:

- GET
- CREATE
- UPDATE
- DELETE

Also - data has to be saved in a map, which must be available for each HTTP request (implement simple cache solution).

## Create sample `User`

```shell
curl -X POST 127.0.0.1:3000/users --data '{ "name": "xx", "surname": "yy", "age": 32 }' -H 'Content-Type: application/json'
```

## Get all `Users`

```shell
curl -X GET 127.0.0.1:3000/users
```

## Get single `User`

```shell
curl -X GET 127.0.0.1:3000/users/ab94494c-d296-42ed-90b1-1ac38f87b9b6
```

## Patch `User`

```shell

curl -X PATCH 127.0.0.1:3000/users/ab94494c-d296-42ed-90b1-1ac38f87b9b6 --data '{"name": "yy", "surname": "Nowaak", "age": 11 }' -H 'Content-Type: application/json'
```

## Delete `User`

```shell
curl -X DELETE 127.0.0.1:3000/users/ab94494c-d296-42ed-90b1-1ac38f87b9b6
```
