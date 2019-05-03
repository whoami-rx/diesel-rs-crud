# CRUD with diesel-rs

```
$ echo "DATABSE_URL=src/database.sqlite" > .env
$ diesel migration run
```

## Create

```
# published equal false
$ cargo run --bin create_post
```

## Read

```
# published equal true
$ cargo run --bin read_posts
```

## Update

```
# set published true
$ cargo run --bin update_post 1
```

### Delete

```
$ cargo run --bin delete_post lorem title
```
